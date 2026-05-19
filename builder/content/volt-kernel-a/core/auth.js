class Auth {
  constructor(logger) {
    this.logger = logger;
  }

  authorize(message, kernel) {
    const type = typeof message?.type === "string" ? message.type : "";
    const namespace = type.split(".")[0] || "unknown";

    // Require explicit appId — no default to super-admin
    const appId = message?.appId;
    if (!appId || typeof appId !== "string") {
      this.logger.warn("auth:deny", { type, reason: "missing_appId" });
      return false;
    }

    const allowed = kernel.permissionManager.can(appId, type);
    if (allowed) return true;

    // Namespace-level check
    if (namespace !== appId) {
      const nsAllowed = kernel.permissionManager.can(appId, `${namespace}.*`);
      if (nsAllowed) return true;
    }

    this.logger.warn("auth:deny", { type, appId });
    return false;
  }
}

module.exports = Auth;
