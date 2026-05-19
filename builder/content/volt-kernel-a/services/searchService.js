const fs = require("node:fs/promises");
const path = require("node:path");

class SearchService {
  constructor(logger, fileSystem) {
    this.logger = logger;
    this.fileSystem = fileSystem;
  }

  async walk(virtualPath, depth = 0, maxDepth = 3) {
    if (depth > maxDepth) return [];
    const listing = await this.fileSystem.list(virtualPath);
    const results = [];
    for (const node of listing.nodes) {
      const nextPath = `${virtualPath.replace(/\/+$/, "")}/${node.name}`.replace(/\/+/g, "/");
      results.push({ ...node, path: nextPath });
      if (node.kind === "dir") {
        results.push(...(await this.walk(nextPath, depth + 1, maxDepth)));
      }
    }
    return results;
  }

  async query(payload = {}, kernel) {
    const term = String(payload.term || "").trim().toLowerCase();
    if (payload.scope === "files") {
      return await this.queryFiles(term);
    }
    const hits = [];

    for (const app of kernel.apps) {
      if (!term || app.name.toLowerCase().includes(term) || app.id.toLowerCase().includes(term)) {
        hits.push({
          kind: "app",
          id: app.id,
          title: app.name,
          subtitle: "Application"
        });
      }
    }

    const settingsEntries = [
      { id: "settings.appearance", title: "Appearance", subtitle: "Settings" },
      { id: "settings.network", title: "Network", subtitle: "Settings" },
      { id: "settings.security", title: "Security", subtitle: "Settings" }
    ];
    for (const entry of settingsEntries) {
      if (!term || entry.title.toLowerCase().includes(term) || entry.id.toLowerCase().includes(term)) {
        hits.push({ kind: "settings", ...entry });
      }
    }

    try {
      const files = await this.walk("/User", 2, 3);
      for (const entry of files) {
        if (!term || entry.name.toLowerCase().includes(term)) {
          hits.push({
            kind: "file",
            id: entry.path,
            title: entry.name,
            subtitle: entry.path
          });
        }
      }
    } catch {}

    return hits.slice(0, 40);
  }

  async queryFiles(term) {
    const normalized = String(term || "").trim().toLowerCase();
    if (!normalized) return [];

    const roots = ["/User/Desktop", "/User/Documents", "/User/Downloads", "/User/Photos", "/User/Music", "/User/Videos", "/User"];
    const seen = new Set();
    const hits = [];

    for (const root of roots) {
      let entries = [];
      try {
        entries = await this.walk(root, 0, 6);
      } catch {
        continue;
      }

      for (const entry of entries) {
        if (hits.length >= 80) break;
        if (seen.has(entry.path)) continue;
        if (!entry.name.toLowerCase().includes(normalized)) continue;
        seen.add(entry.path);
        hits.push({
          kind: "file",
          id: entry.path,
          title: entry.name,
          subtitle: entry.path,
          path: entry.path,
          fileKind: entry.kind,
          modifiedAt: entry.modifiedAt || null,
          size: entry.size || null
        });
      }
    }

    return hits;
  }
}

module.exports = SearchService;
