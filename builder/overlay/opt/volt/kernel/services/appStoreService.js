const fs = require("node:fs/promises");
const path = require("node:path");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class AppStoreService {
  constructor(logger, appStaticServer) {
    this.logger = logger;
    this.appStaticServer = appStaticServer || null;
    this.appsRoot = path.resolve(__dirname, "../../.volt/user/Apps");
    this.registryFile = path.resolve(__dirname, "../../.volt/system/apps-registry.json");
  }

  async ensureRoot() {
    await fs.mkdir(this.appsRoot, { recursive: true });
    await fs.mkdir(path.dirname(this.registryFile), { recursive: true });
  }

  async loadRegistry() {
    await this.ensureRoot();
    try {
      const raw = await fs.readFile(this.registryFile, "utf8");
      return JSON.parse(raw);
    } catch {
      return [];
    }
  }

  async fileExists(targetPath) {
    try {
      const stat = await fs.stat(targetPath);
      return stat.isFile();
    } catch {
      return false;
    }
  }

  async directoryExists(targetPath) {
    try {
      const stat = await fs.stat(targetPath);
      return stat.isDirectory();
    } catch {
      return false;
    }
  }

  normalizeEntry(entry) {
    return {
      ...entry,
      manifest: entry.manifest || null,
      launchable: Boolean(entry.launchable),
      launchStrategy: entry.launchStrategy || null,
      launchRoot: entry.launchRoot || null,
      launchEntry: entry.launchEntry || null,
      launchUrl: entry.launchUrl || null,
      launchError: entry.launchError || null,
      source: "app-store"
    };
  }

  async inspectInstall(entry) {
    const normalized = this.normalizeEntry(entry);
    const packagePath = path.join(normalized.path, "package.json");
    let manifest = null;

    try {
      const raw = await fs.readFile(packagePath, "utf8");
      const pkg = JSON.parse(raw);
      manifest = {
        name: typeof pkg.name === "string" ? pkg.name : normalized.repoName,
        displayName:
          typeof pkg.displayName === "string"
            ? pkg.displayName
            : typeof pkg.productName === "string"
              ? pkg.productName
              : typeof pkg.name === "string"
                ? pkg.name
                : normalized.repoName,
        version: typeof pkg.version === "string" ? pkg.version : null,
        description: typeof pkg.description === "string" ? pkg.description : null,
        icon: typeof pkg.samaris?.icon === "string" ? pkg.samaris.icon : null,
        samaris: typeof pkg.samaris === "object" && pkg.samaris ? pkg.samaris : null
      };
    } catch {
      manifest = null;
    }

    const rootCandidates = [];
    if (manifest?.samaris?.entry && typeof manifest.samaris.entry === "string") {
      rootCandidates.push({
        root: path.dirname(manifest.samaris.entry),
        entry: path.basename(manifest.samaris.entry)
      });
    }
    const buildDirs = ["dist", "build", "out", "www", "public", "static"];
    for (const dir of buildDirs) {
      const absDir = path.resolve(normalized.path, dir);
      if (!(await this.directoryExists(absDir))) continue;
      if (await this.fileExists(path.resolve(absDir, "index.html"))) {
        rootCandidates.push({ root: dir, entry: "index.html" });
        continue;
      }
      const subdirs = await fs.readdir(absDir, { withFileTypes: true }).catch(() => []);
      for (const entry of subdirs) {
        if (!entry.isDirectory()) continue;
        if (await this.fileExists(path.resolve(absDir, entry.name, "index.html"))) {
          rootCandidates.push({ root: `${dir}/${entry.name}`, entry: "index.html" });
        }
      }
    }
    rootCandidates.push(
      { root: ".", entry: "index.html" }
    );

    let launchRoot = null;
    let launchEntry = null;
    for (const candidate of rootCandidates) {
      const absoluteRoot = path.resolve(normalized.path, candidate.root);
      const absoluteEntry = path.resolve(absoluteRoot, candidate.entry);
      if (!(await this.directoryExists(absoluteRoot))) continue;
      if (!(await this.fileExists(absoluteEntry))) continue;
      if (absoluteRoot !== normalized.path && !absoluteRoot.startsWith(`${normalized.path}${path.sep}`)) continue;
      launchRoot = path.relative(normalized.path, absoluteRoot) || ".";
      launchEntry = candidate.entry;
      break;
    }

    const launchable = Boolean(launchRoot && launchEntry);
    let launchUrl = null;
    if (launchable) {
      const launchRootAbs = path.resolve(normalized.path, launchRoot);
      try {
        const appPort = this.appStaticServer.getAppPort(normalized.appId);
        if (appPort) {
          launchUrl = `http://127.0.0.1:${appPort}/`;
        } else {
          const { port } = await this.appStaticServer.startApp(normalized.appId, launchRootAbs);
          launchUrl = `http://127.0.0.1:${port}/`;
        }
      } catch {
        launchUrl = null;
      }
    }
    return {
      ...normalized,
      manifest,
      launchable: launchable && Boolean(launchUrl),
      launchStrategy: launchable ? "static-site" : null,
      launchRoot,
      launchEntry,
      launchUrl,
      launchError: launchable && !launchUrl
        ? "Unable to start app server."
        : launchable
          ? null
          : manifest
            ? "Built files not found. Expected an index.html output in dist, public, or the samaris.entry path."
            : "Missing package.json manifest. Samaris cannot validate or launch this app yet."
    };
  }

  async saveRegistry(entries) {
    await this.ensureRoot();
    await fs.writeFile(this.registryFile, JSON.stringify(entries, null, 2), "utf8");
  }

  normalize(url = "") {
    const clean = String(url).trim();
    const repoName = clean.replace(/\/+$/, "").split("/").pop()?.replace(/\.git$/, "") || `app-${Date.now()}`;
    const appId = repoName.toLowerCase().replace(/[^a-z0-9-]+/g, "-");
    return {
      url: clean,
      repoName,
      appId,
      targetDir: path.join(this.appsRoot, appId)
    };
  }

  async run(cmd, args, cwd) {
    try {
      const { stdout, stderr } = await execFileAsync(cmd, args, { cwd, timeout: 120000 });
      return { ok: true, log: [stdout, stderr].filter(Boolean).join("\n") };
    } catch (error) {
      return { ok: false, log: [error.stdout, error.stderr, error.message].filter(Boolean).join("\n") };
    }
  }

  async listInstalled() {
    const registry = await this.loadRegistry();
    const inspected = await Promise.all(registry.map((entry) => this.inspectInstall(entry)));
    await this.saveRegistry(inspected);
    return inspected;
  }

  async clone(payload = {}) {
    const normalized = this.normalize(payload.url);
    const registry = await this.loadRegistry();
    await fs.rm(normalized.targetDir, { recursive: true, force: true });
    const cloneResult = await this.run("git", ["clone", normalized.url, normalized.targetDir], path.dirname(normalized.targetDir));
    if (!cloneResult.ok) {
      return { ok: false, stage: "clone", logs: cloneResult.log };
    }
    const entry = {
      appId: normalized.appId,
      repoName: normalized.repoName,
      url: normalized.url,
      path: normalized.targetDir,
      installedAt: new Date().toISOString(),
      status: "cloned"
    };
    const inspected = await this.inspectInstall(entry);
    await this.saveRegistry([...registry.filter((item) => item.appId !== entry.appId), inspected]);
    return { ok: true, stage: "clone", entry: inspected, logs: cloneResult.log };
  }

  async build(payload = {}) {
    const registry = await this.loadRegistry();
    const entry = registry.find((item) => item.appId === payload.appId);
    if (!entry) {
      return { ok: false, stage: "build", logs: "Unknown app" };
    }
    const installResult = await this.run("npm", ["install", "--legacy-peer-deps"], entry.path);
    if (!installResult.ok) {
      return { ok: false, stage: "install", logs: installResult.log };
    }
    const buildResult = await this.run("npm", ["run", "build"], entry.path);
    const nextRegistry = registry.map((item) =>
      item.appId === payload.appId
        ? {
            ...item,
            status: buildResult.ok ? "ready" : "build_failed",
            lastBuiltAt: new Date().toISOString()
          }
        : item
    );
    const inspectedRegistry = await Promise.all(nextRegistry.map((item) => this.inspectInstall(item)));
    await this.saveRegistry(inspectedRegistry);
    return {
      ok: buildResult.ok,
      stage: "build",
      entry: inspectedRegistry.find((item) => item.appId === payload.appId),
      logs: [installResult.log, buildResult.log].filter(Boolean).join("\n\n")
    };
  }

  async update(payload = {}) {
    const registry = await this.loadRegistry();
    const entry = registry.find((item) => item.appId === payload.appId);
    if (!entry) {
      return { ok: false, stage: "update", logs: "Unknown app" };
    }
    this.appStaticServer.stopApp(entry.appId);
    const pullResult = await this.run("git", ["pull"], entry.path);
    if (!pullResult.ok) {
      return { ok: false, stage: "pull", logs: pullResult.log };
    }
    return this.build({ appId: payload.appId });
  }

  async remove(payload = {}) {
    const registry = await this.loadRegistry();
    const entry = registry.find((item) => item.appId === payload.appId);
    if (entry) {
      this.appStaticServer.stopApp(entry.appId);
      await fs.rm(entry.path, { recursive: true, force: true });
    }
    await this.saveRegistry(registry.filter((item) => item.appId !== payload.appId));
    return { ok: true };
  }

  async startApp(payload = {}) {
    const registry = await this.loadRegistry();
    const entry = registry.find((item) => item.appId === payload.appId);
    if (!entry) return { ok: false, error: "Unknown app" };
    if (!entry.launchable || !entry.launchRoot) return { ok: false, error: "App is not launchable" };
    const launchRootAbs = path.resolve(entry.path, entry.launchRoot);
    try {
      const appPort = this.appStaticServer.getAppPort(entry.appId);
      if (appPort) return { ok: true, port: appPort, launchUrl: `http://127.0.0.1:${appPort}/` };
      const { port } = await this.appStaticServer.startApp(entry.appId, launchRootAbs);
      const launchUrl = `http://127.0.0.1:${port}/`;
      const updatedEntry = { ...entry, launchUrl };
      await this.saveRegistry(registry.map((item) => item.appId === entry.appId ? updatedEntry : item));
      return { ok: true, port, launchUrl };
    } catch (error) {
      return { ok: false, error: error.message };
    }
  }

  async stopApp(payload = {}) {
    const result = this.appStaticServer.stopApp(payload.appId);
    return { ok: result.stopped };
  }
}

module.exports = AppStoreService;
