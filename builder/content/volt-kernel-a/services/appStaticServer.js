const http = require("node:http");
const fs = require("node:fs");
const fsp = require("node:fs/promises");
const path = require("node:path");

const PORT_START = 19000;
const PORT_END = 19999;

const MIME_TYPES = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".mjs": "application/javascript; charset=utf-8",
  ".cjs": "application/javascript; charset=utf-8",
  ".jsx": "application/javascript; charset=utf-8",
  ".ts": "text/typescript; charset=utf-8",
  ".tsx": "text/typescript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".map": "application/json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".ico": "image/x-icon",
  ".xml": "application/xml; charset=utf-8",
  ".woff": "font/woff",
  ".woff2": "font/woff2",
  ".ttf": "font/ttf",
  ".wasm": "application/wasm",
  ".txt": "text/plain; charset=utf-8"
};

class AppStaticServer {
  constructor(logger) {
    this.logger = logger;
    this.servers = new Map();
    this.portPool = new Set();
    for (let p = PORT_START; p <= PORT_END; p++) this.portPool.add(p);
  }

  async allocatePort() {
    for (const port of this.portPool) {
      if (this.servers.has(port)) continue;
      const free = await this.isPortFree(port);
      if (free) return port;
    }
    throw new Error("No free port available for app server");
  }

  isPortFree(port) {
    return new Promise((resolve) => {
      const server = require("node:net").createServer();
      server.once("error", () => resolve(false));
      server.once("listening", () => { server.close(); resolve(true); });
      server.listen(port, "127.0.0.1");
    });
  }

  async startApp(appId, rootDir, spaFallback = true) {
    if (this.getAppPort(appId)) {
      return { port: this.getAppPort(appId) };
    }

    const port = await this.allocatePort();
    const server = http.createServer(async (req, res) => {
      const unsafePath = req.url.split("?")[0].split("#")[0];
      const decoded = decodeURIComponent(unsafePath).replace(/^\/+/, "");
      const ext = path.extname(decoded);
      let absolutePath = decoded
        ? path.resolve(rootDir, decoded)
        : path.resolve(rootDir, "index.html");

      this.logger.info(`app-server:${req.method} ${req.url} → ${absolutePath}`);

      if (!absolutePath.startsWith(path.resolve(rootDir))) {
        res.writeHead(403, { "content-type": "application/json" });
        res.end(JSON.stringify({ error: "forbidden" }));
        this.logger.info(`app-server:403 ${absolutePath}`);
        return;
      }

      let stat = await fsp.stat(absolutePath).catch(() => null);

      if (!stat && decoded.includes("/")) {
        const stripped = decoded.replace(/^[^/]+\//, "");
        const altPath = path.resolve(rootDir, stripped);
        this.logger.info(`app-server:retry1 ${absolutePath} → ${altPath}`);
        if (altPath.startsWith(path.resolve(rootDir))) {
          stat = await fsp.stat(altPath).catch(() => null);
          if (stat) absolutePath = altPath;
        }
      }

      if (!stat) {
        for (const prefix of ["dist", "build", "out", "www", "public"]) {
          const prefixed = path.resolve(rootDir, prefix, decoded);
          this.logger.info(`app-server:retry2 ${absolutePath} → ${prefixed}`);
          if (!prefixed.startsWith(path.resolve(rootDir))) continue;
          stat = await fsp.stat(prefixed).catch(() => null);
          if (stat) { absolutePath = prefixed; break; }
        }
      }

      if (stat && stat.isFile()) {
        this.serveFile(res, absolutePath);
        return;
      }

      if (spaFallback && !ext) {
        const indexPath = path.resolve(rootDir, "index.html");
        const indexStat = await fsp.stat(indexPath).catch(() => null);
        if (indexStat && indexStat.isFile()) {
          this.serveFile(res, indexPath);
          return;
        }
      }

      res.writeHead(404, { "content-type": "application/json" });
      res.end(JSON.stringify({ error: "not_found" }));
      this.logger.info(`app-server:404 ${absolutePath}`);
    });

    await new Promise((resolve, reject) => {
      server.listen(port, "127.0.0.1", resolve);
      server.once("error", reject);
    });
    this.logger.info(`App server started for ${appId} on port ${port}`);

    this.servers.set(port, { appId, server, rootDir });
    return { port };
  }

  serveFile(res, absolutePath) {
    const ext = path.extname(absolutePath).toLowerCase();
    const contentType = MIME_TYPES[ext] || "application/octet-stream";
    const isHtml = ext === ".html";

    res.writeHead(200, {
      "content-type": contentType,
      "cache-control": isHtml ? "no-store" : "public, max-age=3600",
      "access-control-allow-origin": "*",
      "content-security-policy": "default-src 'self'; script-src 'self' 'unsafe-inline' 'unsafe-eval' https:; style-src 'self' 'unsafe-inline' https:; img-src * data: blob:; connect-src *; font-src * data:; frame-ancestors 'none'; base-uri 'self';"
    });
    this.logger.info(`app-server:200 ${absolutePath} (${contentType})`);
    fs.createReadStream(absolutePath).pipe(res);
  }

  stopApp(appId) {
    for (const [port, entry] of this.servers) {
      if (entry.appId === appId) {
        entry.server.close(() => {
          this.logger.info(`App server stopped for ${appId} (port ${port})`);
        });
        this.servers.delete(port);
        return { stopped: true, port };
      }
    }
    return { stopped: false };
  }

  getAppPort(appId) {
    for (const [port, entry] of this.servers) {
      if (entry.appId === appId) return port;
    }
    return null;
  }

  getAppStatus(appId) {
    const port = this.getAppPort(appId);
    if (!port) return { running: false };
    return { running: true, port };
  }

  stopAll() {
    for (const [port, entry] of this.servers) {
      entry.server.close();
      this.logger.info(`App server stopped for ${entry.appId} (port ${port})`);
    }
    this.servers.clear();
  }
}

module.exports = AppStaticServer;
