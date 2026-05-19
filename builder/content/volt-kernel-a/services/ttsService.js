const { spawn } = require("node:child_process");
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");

class TtsService {
  constructor(logger) {
    this.logger = logger;
    this.llamaTtsBin = process.env.LLAMA_TTS_BIN || "llama-tts";
    this.ttsModel = process.env.OUTETTS_MODEL || "/opt/volt/ai-models/outetts/OuteTTS-0.2-500M-Q8_0.gguf";
    this.ttsVocoder = process.env.WAVTOKENIZER_MODEL || "/opt/volt/ai-models/outetts/WavTokenizer-Large-75-F16.gguf";
  }

  async speak(payload = {}) {
    const started = Date.now();
    const text = String(payload.text || "").trim();
    if (!text) {
      const error = new Error("tts_text_required");
      error.code = "TTS_TEXT_REQUIRED";
      throw error;
    }

    const tmp = await fs.mkdtemp(path.join(os.tmpdir(), "samaris-tts-"));
    const output = path.join(tmp, "speech.wav");

    try {
      await this.runOuteTTS(text, output);
      const audio = await fs.readFile(output);
      return {
        audioBase64: audio.toString("base64"),
        mimeType: "audio/wav",
        durationMs: Date.now() - started
      };
    } finally {
      await fs.rm(tmp, { recursive: true, force: true }).catch(() => {});
    }
  }

  runOuteTTS(text, output) {
    return new Promise((resolve, reject) => {
      const child = spawn(this.llamaTtsBin, [
        "-m", this.ttsModel,
        "-mv", this.ttsVocoder,
        "-p", text,
        "-o", output,
      ], {
        stdio: ["ignore", "pipe", "pipe"]
      });

      let stderr = "";
      child.stderr.on("data", (chunk) => { stderr += chunk.toString("utf8"); });
      child.on("error", reject);
      child.on("close", (code) => {
        if (code === 0) resolve();
        else {
          const error = new Error(stderr.trim() || "outetts_failed");
          error.code = "TTS_SYNTH_FAILED";
          reject(error);
        }
      });
    });
  }
}

module.exports = TtsService;
