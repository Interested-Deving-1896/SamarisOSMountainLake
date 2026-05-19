const fs = require("node:fs/promises");
const path = require("node:path");
const { parseFile } = require("music-metadata");

const AUDIO_EXTENSIONS = new Set([".mp3", ".flac", ".wav", ".m4a", ".aac", ".ogg", ".weba"]);
const VIDEO_EXTENSIONS = new Set([".mp4", ".webm", ".ogg", ".avi", ".mkv", ".mov", ".m4v"]);

function titleFromPath(name) {
  return String(name || "")
    .replace(/\.[^.]+$/, "")
    .replace(/[_-]+/g, " ")
    .trim();
}

function durationLabel(totalSeconds = 0) {
  if (!Number.isFinite(totalSeconds) || totalSeconds <= 0) return "--:--";
  const minutes = Math.floor(totalSeconds / 60);
  const seconds = Math.floor(totalSeconds % 60);
  return `${String(minutes).padStart(1, "0")}:${String(seconds).padStart(2, "0")}`;
}

async function walkFiles(root) {
  const files = [];
  const queue = [root];

  while (queue.length) {
    const current = queue.shift();
    const entries = await fs.readdir(current, { withFileTypes: true }).catch(() => []);
    for (const entry of entries) {
      if (entry.name.startsWith(".")) continue;
      const fullPath = path.join(current, entry.name);
      if (entry.isDirectory()) {
        queue.push(fullPath);
      } else if (entry.isFile()) {
        files.push(fullPath);
      }
    }
  }

  return files;
}

class MediaService {
  constructor(logger, fileSystem) {
    this.logger = logger;
    this.fileSystem = fileSystem;
  }

  async listMusicLibrary() {
    await this.fileSystem.ensureVirtualRoots();
    const { actualPath } = this.fileSystem.toActualPath("/User/Music");
    const files = await walkFiles(actualPath);
    const tracks = await Promise.all(
      files
        .filter((filePath) => AUDIO_EXTENSIONS.has(path.extname(filePath).toLowerCase()))
        .map(async (filePath) => {
          const relative = path.relative(actualPath, filePath).split(path.sep).join("/");
          const virtualPath = `/User/Music/${relative}`.replace(/\/+/g, "/");
          const stat = await fs.stat(filePath);
          let metadata = null;
          try {
            metadata = await parseFile(filePath, { skipCovers: false });
          } catch {
            metadata = null;
          }

          const common = metadata?.common || {};
          const format = metadata?.format || {};
          const cover = Array.isArray(common.picture) && common.picture[0] ? common.picture[0] : null;

          return {
            id: virtualPath,
            path: virtualPath,
            fileName: path.basename(filePath),
            title: common.title || titleFromPath(path.basename(filePath)),
            artist: common.artist || "Unknown Artist",
            album: common.album || "Unknown Album",
            genre: common.genre?.[0] || "Library",
            year: common.year || null,
            durationSeconds: Number.isFinite(format.duration) ? Number(format.duration) : 0,
            durationLabel: durationLabel(format.duration || 0),
            size: stat.size,
            coverDataUrl: cover
              ? `data:${cover.format || "image/jpeg"};base64,${Buffer.from(cover.data).toString("base64")}`
              : null
          };
        })
    );

    tracks.sort((left, right) => {
      if (left.artist !== right.artist) return left.artist.localeCompare(right.artist);
      if (left.album !== right.album) return left.album.localeCompare(right.album);
      return left.title.localeCompare(right.title);
    });

    return tracks;
  }

  async listVideoLibrary() {
    await this.fileSystem.ensureVirtualRoots();
    const { actualPath } = this.fileSystem.toActualPath("/User/Videos");
    const files = await walkFiles(actualPath);
    const videos = await Promise.all(
      files
        .filter((filePath) => VIDEO_EXTENSIONS.has(path.extname(filePath).toLowerCase()))
        .map(async (filePath) => {
          const relative = path.relative(actualPath, filePath).split(path.sep).join("/");
          const virtualPath = `/User/Videos/${relative}`.replace(/\/+/g, "/");
          const stat = await fs.stat(filePath);
          return {
            id: virtualPath,
            path: virtualPath,
            fileName: path.basename(filePath),
            title: titleFromPath(path.basename(filePath)),
            format: path.extname(filePath).slice(1).toUpperCase(),
            size: stat.size
          };
        })
    );

    videos.sort((left, right) => left.title.localeCompare(right.title));
    return videos;
  }
}

module.exports = MediaService;
