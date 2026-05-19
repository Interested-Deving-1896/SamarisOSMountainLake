const { execFile } = require("node:child_process");
const fs = require("node:fs/promises");
const os = require("node:os");
const path = require("node:path");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

function extForMime(mimeType) {
  const mime = String(mimeType || "").toLowerCase();
  if (mime.includes("wav")) return ".wav";
  if (mime.includes("ogg")) return ".ogg";
  if (mime.includes("mp4") || mime.includes("m4a")) return ".m4a";
  return ".webm";
}

class SttService {
  constructor(logger) {
    this.logger = logger;
    this.whisperBin = process.env.WHISPER_BIN || "/opt/volt/ai-models/bin/whisper";
    this.whisperModel = process.env.WHISPER_MODEL || "/opt/volt/ai-models/whisper/ggml-small.bin";
    this.ffmpegBin = process.env.FFMPEG_BIN || "ffmpeg";
  }

  async transcribe(payload = {}) {
    const started = Date.now();
    const normalized = String(payload.audioBase64 || "")
      .replace(/^data:[^;]+;base64,/, "")
      .replace(/\s+/g, "");
    if (!normalized) {
      const error = new Error("stt_audio_required");
      error.code = "STT_AUDIO_REQUIRED";
      throw error;
    }

    const tmp = await fs.mkdtemp(path.join(os.tmpdir(), "samaris-stt-"));
    const source = path.join(tmp, `source${extForMime(payload.mimeType)}`);
    const wav = path.join(tmp, "output.wav");
    const outPrefix = path.join(tmp, "transcript");

    try {
      await fs.writeFile(source, Buffer.from(normalized, "base64"));
      await this.ensureFile(this.whisperBin, "STT_BINARY_MISSING");
      await this.ensureFile(this.whisperModel, "STT_MODEL_MISSING");

      try {
        await execFileAsync(this.ffmpegBin, [
          "-y",
          "-i", source,
          "-ar", "16000",
          "-ac", "1",
          "-c:a", "pcm_s16le",
          wav
        ], { timeout: 60000, maxBuffer: 1024 * 1024 * 4 });
      } catch (fferr) {
        this.logger?.info("stt:ffmpeg_fallback", fferr.message);
        await fs.copyFile(source, wav);
      }

      const language = payload.language ? String(payload.language) : "auto";
      const { stdout, stderr } = await execFileAsync(this.whisperBin, [
        "-m", this.whisperModel,
        "-f", wav,
        "-l", language,
        "-otxt",
        "-of", outPrefix,
        "-nt"
      ], { timeout: Number(process.env.WHISPER_TIMEOUT_MS || 180000), maxBuffer: 1024 * 1024 * 8 });

      let text = "";
      try {
        text = await fs.readFile(`${outPrefix}.txt`, "utf8");
      } catch {
        text = `${stdout}\n${stderr}`;
      }

      return {
        text: this.cleanTranscript(text),
        language: language === "auto" ? undefined : language,
        durationMs: Date.now() - started
      };
    } finally {
      await fs.rm(tmp, { recursive: true, force: true }).catch(() => {});
    }
  }

  async ensureFile(filePath, code) {
    try {
      await fs.access(filePath);
    } catch {
      const error = new Error(code.toLowerCase());
      error.code = code;
      throw error;
    }
  }

  cleanTranscript(raw) {
    return String(raw || "")
      .split("\n")
      .map((line) => line.replace(/^\s*\[[^\]]+\]\s*/g, "").trim())
      .filter(Boolean)
      .join(" ")
      .replace(/\s+/g, " ")
      .trim();
  }
}

module.exports = SttService;
