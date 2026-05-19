const fs = require("node:fs/promises");
const path = require("node:path");

class PermissionManager {
  constructor(logger) {
    this.logger = logger;
    this.permissions = new Map();
    this.stateFile = path.resolve(__dirname, "../../.volt/system/permissions.json");
    this.loaded = false;
  }

  async ensureLoaded() {
    if (this.loaded) return;
    this.loaded = true;
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      const parsed = JSON.parse(raw);
      for (const [appId, actions] of Object.entries(parsed || {})) {
        const current = this.permissions.get(appId) || {};
        this.permissions.set(appId, { ...actions, ...current });
      }
    } catch {}
  }

  async persist() {
    await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
    const payload = Object.fromEntries(this.permissions.entries());
    await fs.writeFile(this.stateFile, JSON.stringify(payload, null, 2), "utf8");
  }

  seed(appId, permissions) {
    const current = this.permissions.get(appId) || {};
    const next = { ...current };
    for (const permission of permissions) {
      next[permission] = true;
    }
    this.permissions.set(appId, next);
  }

  can(appId, action) {
    this.logger.info("permission:check", { appId, action });
    const appPermissions = this.permissions.get(appId);
    if (!appPermissions) return false;

    // Exact match
    if (action in appPermissions) return Boolean(appPermissions[action]);

    // Wildcard namespace match: "fs.*" matches "fs.list", "fs.read", etc.
    const parts = action.split(".");
    for (let i = parts.length; i > 0; i--) {
      const pattern = parts.slice(0, i).join(".") + ".*";
      if (pattern in appPermissions) return Boolean(appPermissions[pattern]);
    }

    // Global wildcard
    if ("*" in appPermissions) return Boolean(appPermissions["*"]);

    return false;
  }

  list(appId) {
    const appPermissions = this.permissions.get(appId) || {};
    return Object.entries(appPermissions).map(([action, allowed]) => ({ action, allowed: Boolean(allowed) }));
  }

  listAll() {
    return Array.from(this.permissions.entries()).map(([appId, actions]) => ({
      appId,
      permissions: Object.entries(actions).map(([action, allowed]) => ({ action, allowed: Boolean(allowed) }))
    }));
  }

  async set(appId, action, allowed) {
    await this.ensureLoaded();
    const current = this.permissions.get(appId) || {};
    this.permissions.set(appId, {
      ...current,
      [action]: Boolean(allowed)
    });
    await this.persist();
    return this.list(appId);
  }
}

module.exports = PermissionManager;
