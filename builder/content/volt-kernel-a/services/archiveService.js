const path = require("node:path");
const fs = require("node:fs/promises");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

function isWithin(base, target) {
  const relative = path.relative(base, target);
  return Boolean(relative && !relative.startsWith("..") && !path.isAbsolute(relative));
}

function safeResolve(destDir, entryName) {
  const resolved = path.resolve(destDir, entryName);
  if (!isWithin(destDir, resolved)) {
    throw new Error(`Path traversal blocked: ${entryName}`);
  }
  return resolved;
}

class ArchiveService {
  constructor(logger) {
    this.logger = logger;
  }

  async extract(archivePath, destDir) {
    await fs.mkdir(destDir, { recursive: true });
    const ext = path.extname(archivePath).toLowerCase();
    const baseName = path.basename(archivePath).toLowerCase();

    try {
      let stdout;
      if (ext === ".zip") {
        const result = await execFileAsync("unzip", ["-o", archivePath, "-d", destDir]);
        stdout = result.stdout;
      } else if (ext === ".gz" && baseName.endsWith(".tar.gz")) {
        const result = await execFileAsync("tar", ["-xzf", archivePath, "-C", destDir]);
        stdout = result.stdout;
      } else if (ext === ".tgz") {
        const result = await execFileAsync("tar", ["-xzf", archivePath, "-C", destDir]);
        stdout = result.stdout;
      } else if (ext === ".tar") {
        const result = await execFileAsync("tar", ["-xf", archivePath, "-C", destDir]);
        stdout = result.stdout;
      } else if (ext === ".bz2" && baseName.endsWith(".tar.bz2")) {
        const result = await execFileAsync("tar", ["-xjf", archivePath, "-C", destDir]);
        stdout = result.stdout;
      } else {
        return { ok: false, error: `Unsupported archive format: ${ext}` };
      }

      const files = [];
      const walk = async (dir) => {
        const entries = await fs.readdir(dir, { withFileTypes: true });
        for (const entry of entries) {
          const fullPath = path.join(dir, entry.name);
          files.push(fullPath);
          if (entry.isDirectory()) await walk(fullPath);
        }
      };
      await walk(destDir).catch(() => {});

      return { ok: true, path: destDir, files };
    } catch (error) {
      this.logger.error("archive:extract", error instanceof Error ? error.message : String(error));
      return { ok: false, error: error instanceof Error ? error.message : String(error) };
    }
  }

  async listContents(archivePath) {
    const ext = path.extname(archivePath).toLowerCase();
    const baseName = path.basename(archivePath).toLowerCase();

    try {
      let stdout;
      if (ext === ".zip") {
        const result = await execFileAsync("unzip", ["-l", archivePath]);
        stdout = result.stdout;
      } else if (ext === ".gz" && baseName.endsWith(".tar.gz")) {
        const result = await execFileAsync("tar", ["-tzf", archivePath]);
        stdout = result.stdout;
      } else if (ext === ".tgz") {
        const result = await execFileAsync("tar", ["-tzf", archivePath]);
        stdout = result.stdout;
      } else if (ext === ".tar") {
        const result = await execFileAsync("tar", ["-tf", archivePath]);
        stdout = result.stdout;
      } else if (ext === ".bz2" && baseName.endsWith(".tar.bz2")) {
        const result = await execFileAsync("tar", ["-tjf", archivePath]);
        stdout = result.stdout;
      } else {
        return { ok: false, error: `Unsupported archive format: ${ext}` };
      }

      const entries = stdout.trim().split("\n").filter(Boolean);
      return { ok: true, entries };
    } catch (error) {
      this.logger.error("archive:listContents", error instanceof Error ? error.message : String(error));
      return { ok: false, error: error instanceof Error ? error.message : String(error) };
    }
  }
}

module.exports = ArchiveService;
