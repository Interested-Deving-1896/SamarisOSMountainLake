const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { spawn } = require("node:child_process");

const MODEL_FILENAME = "Qwen3-1.7B-Q8_0.gguf";
const LLAMA_HOST = "127.0.0.1:8081";
const LLAMA_URL = `http://${LLAMA_HOST}`;
const IDLE_SHUTDOWN_MS = 45000;

function wait(ms) {
  return new Promise((resolve) => setTimeout(resolve, ms));
}

function formatBytes(bytes) {
  if (!Number.isFinite(bytes) || bytes <= 0) return "--";
  const units = ["B", "KB", "MB", "GB"];
  let value = bytes;
  let index = 0;
  while (value >= 1024 && index < units.length - 1) {
    value /= 1024;
    index += 1;
  }
  return `${value >= 100 ? Math.round(value) : value.toFixed(value >= 10 ? 1 : 2)} ${units[index]}`;
}

function buildMessages(input) {
  const isThinking = input.modeId === "smart" || input.modeId === "code" || input.modeId === "data-science";
  const modeTag = isThinking ? "/think" : "/no_think";
  return [
    {
      role: "user",
      content: `${input.prompt}\n${modeTag}`
    }
  ];
}

class OrbitRuntimeService {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.kernelRoot = path.resolve(__dirname, "..");
    this.llamaProcess = null;
    this.spawnedByService = false;
    this.modelPath = null;
    this.preparePromise = null;
    this.idleTimer = null;
    this.orbitBridge = null;
    this.kernelB = null;
  }

  setKernelB(kb) { this.kernelB = kb; }

  setUnifier(unifier) {
    this.orbitBridge = unifier?.bridges?.orbit || null;
  }

  async status() {
    const modelPath = await this.resolveModelPath();
    const modelStats = await this.readModelStats(modelPath);
    const serverReady = await this.isLlamaReady();

    if (!modelPath) {
      return {
        name: MODEL_FILENAME,
        sizeLabel: "--",
        runtimeStatus: "unavailable",
        runtimeLabel: "Offline model unavailable",
        provider: "llama.cpp",
        modelId: "qwen3-1.7b"
      };
    }

    return {
      name: path.basename(modelPath),
      sizeLabel: modelStats.sizeLabel,
      runtimeStatus: serverReady ? "ready" : "loading",
      runtimeLabel: serverReady ? "Local model ready" : "Starting…",
      provider: "llama.cpp",
      modelId: "qwen3-1.7b"
    };
  }

  async generate(input, stream) {
    await this.ensureReady();
    this.clearIdleShutdown();

    // Notify Volt modules before inference
    this.orbitBridge?.notifyInferenceStart(input.modeId).catch(() => {});

    stream?.send({
      type: "orbit.stream.start",
      data: {
        requestId: input.requestId,
        modelId: "qwen3-1.7b"
      }
    });

    const isThinking = input.modeId === "smart" || input.modeId === "code" || input.modeId === "data-science";

    const response = await fetch(`${LLAMA_URL}/v1/chat/completions`, {
      method: "POST",
      headers: { "content-type": "application/json" },
      body: JSON.stringify({
        model: "qwen3-1.7b",
        messages: buildMessages(input),
        stream: true,
        temperature: isThinking ? (input.modeId === "code" ? 0.5 : 0.6) : 0.7,
        top_p: isThinking ? 0.95 : 0.8,
        top_k: 20,
        min_p: 0,
        presence_penalty: 1.5,
        repeat_penalty: 1.15,
        n_predict: isThinking ? 4096 : 1024,
        cache_prompt: true
      })
    });

    if (!response.ok || !response.body) {
      const detail = await response.text().catch(() => "");
      const error = new Error(detail || "orbit_generate_failed");
      error.code = "orbit_generate_failed";
      throw error;
    }

    const reader = response.body.getReader();
    const decoder = new TextDecoder();
    let buffer = "";
    let content = "";

    while (true) {
      const { value, done } = await reader.read();
      if (done) break;

      buffer += decoder.decode(value, { stream: true });

      while (buffer.includes("\n")) {
        const lineBreakIndex = buffer.indexOf("\n");
        const line = buffer.slice(0, lineBreakIndex).trim();
        buffer = buffer.slice(lineBreakIndex + 1);

        if (!line || line === "data: [DONE]") continue;
        if (!line.startsWith("data: ")) continue;

        try {
          const json = JSON.parse(line.slice(6));
          const choice = json.choices?.[0];
          if (!choice) continue;

          const reasoningDelta = choice.delta?.reasoning_content || "";
          const contentDelta = choice.delta?.content || "";

          if (contentDelta) {
            content += contentDelta;
            stream?.send({
              type: "orbit.stream.delta",
              data: {
                requestId: input.requestId,
                delta: contentDelta,
                content
              }
            });
          }

          if (reasoningDelta) {
            stream?.send({
              type: "orbit.stream.reasoning",
              data: {
                requestId: input.requestId,
                delta: reasoningDelta
              }
            });
          }
        } catch {
          // skip malformed SSE frames
        }
      }
    }

    stream?.send({
      type: "orbit.stream.done",
      data: {
        requestId: input.requestId,
        content
      }
    });

    // Notify Volt modules after inference
    this.orbitBridge?.notifyInferenceEnd().catch(() => {});
    this.orbitBridge?.logGpuMetrics().catch(() => {});

    this.scheduleIdleShutdown();

    return {
      finalAnswer: content.trim(),
      modelId: "qwen3-1.7b",
      stats: null
    };
  }

  stop() {
    this.clearIdleShutdown();
    if (!this.spawnedByService || !this.llamaProcess || this.llamaProcess.exitCode !== null) {
      return { ok: true, stopped: false };
    }
    this.logger.info("orbit:runtime", "stopping llama-server");
    this.llamaProcess.kill("SIGTERM");
    this.llamaProcess = null;
    this.spawnedByService = false;
    return { ok: true, stopped: true };
  }

  async ensureReady() {
    if (!this.preparePromise) {
      this.preparePromise = this.prepareRuntime().finally(() => {
        this.preparePromise = null;
      });
    }
    return this.preparePromise;
  }

  async prepareRuntime() {
    if (await this.isLlamaReady()) return;

    const modelPath = await this.resolveModelPath();
    if (!modelPath) {
      const error = new Error("orbit_model_not_found");
      error.code = "orbit_model_not_found";
      throw error;
    }

    await this.startLlama(modelPath);
  }

  async startLlama(modelPath) {
    if (this.llamaProcess && this.llamaProcess.exitCode === null) {
      await this.waitForLlama();
      return;
    }

    this.logger.info("orbit:runtime", "starting llama-server");

    const child = spawn("llama-server", [
      "-m", modelPath,
      "--jinja",
      "--reasoning-format", "deepseek",
      "--host", "127.0.0.1",
      "--port", "8081",
      "-ngl", "99",
      "--flash-attn", "on",
      "-t", "6",
      "-c", "8192",
      "--no-context-shift"
    ], {
      stdio: ["ignore", "pipe", "pipe"]
    });

    child.stdout.on("data", (chunk) => {
      this.logger.info("orbit:llama", String(chunk).trim());
    });
    child.stderr.on("data", (chunk) => {
      this.logger.info("orbit:llama", String(chunk).trim());
    });
    child.on("exit", (code) => {
      this.logger.info("orbit:runtime", `llama-server exited (${code ?? "unknown"})`);
      if (this.llamaProcess === child) {
        this.llamaProcess = null;
      }
    });
    child.on("error", (error) => {
      this.logger.error("orbit:runtime", error && error.stack ? error.stack : String(error));
    });

    this.llamaProcess = child;
    this.spawnedByService = true;
    await this.waitForLlama();
  }

  async waitForLlama() {
    const deadline = Date.now() + 60000;
    while (Date.now() < deadline) {
      if (await this.isLlamaReady()) {
        this.logger.info("orbit:runtime", "llama-server ready");
        return;
      }
      await wait(500);
    }
    const error = new Error("llama_start_timeout");
    error.code = "llama_start_timeout";
    throw error;
  }

  async isLlamaReady() {
    try {
      const response = await fetch(`${LLAMA_URL}/v1/models`, {
        signal: AbortSignal.timeout(3000)
      });
      return response.ok;
    } catch {
      return false;
    }
  }

  clearIdleShutdown() {
    if (this.idleTimer) {
      clearTimeout(this.idleTimer);
      this.idleTimer = null;
    }
  }

  scheduleIdleShutdown() {
    this.clearIdleShutdown();
    this.idleTimer = setTimeout(() => {
      this.stop();
    }, IDLE_SHUTDOWN_MS);
  }

  async resolveModelPath() {
    if (this.modelPath) return this.modelPath;

    const candidates = [];
    if (process.env.VOLT_ORBIT_MODEL_PATH) {
      candidates.push(process.env.VOLT_ORBIT_MODEL_PATH);
    }

    candidates.push(path.join(path.resolve(this.kernelRoot, ".."), "ai-models", MODEL_FILENAME));
    candidates.push(path.join(this.kernelRoot, "models", MODEL_FILENAME));
    candidates.push(path.join(path.resolve(this.kernelRoot, ".."), "models", MODEL_FILENAME));
    candidates.push(path.join(path.resolve(this.kernelRoot, "..", "..", "overlay", "opt", "volt", "ai-models"), MODEL_FILENAME));

    for (const candidate of candidates) {
      if (await this.exists(candidate)) {
        this.modelPath = candidate;
        return candidate;
      }
    }

    const desktopPath = await this.findModelUnder(path.join(os.homedir(), "Desktop"), 4);
    if (desktopPath) {
      this.modelPath = desktopPath;
      return desktopPath;
    }

    return null;
  }

  async findModelUnder(root, depth) {
    if (depth < 0 || !(await this.exists(root))) return null;

    const entries = await fs.readdir(root, { withFileTypes: true }).catch(() => []);
    for (const entry of entries) {
      const fullPath = path.join(root, entry.name);
      if (entry.isFile() && entry.name === MODEL_FILENAME) return fullPath;
    }

    for (const entry of entries) {
      if (!entry.isDirectory() || entry.name.startsWith(".")) continue;
      const found = await this.findModelUnder(path.join(root, entry.name), depth - 1);
      if (found) return found;
    }

    return null;
  }

  async readModelStats(modelPath) {
    if (!modelPath) return { sizeLabel: "--" };
    try {
      const stats = await fs.stat(modelPath);
      return { sizeLabel: formatBytes(stats.size) };
    } catch {
      return { sizeLabel: "--" };
    }
  }

  async exists(targetPath) {
    try {
      await fs.access(targetPath);
      return true;
    } catch {
      return false;
    }
  }
}

module.exports = OrbitRuntimeService;
