function createRouter(kernel) {
  const _loaded = {};
  const _handlers = {
    fs: "../handlers/fs",
    media: "../handlers/media",
    app: "../handlers/app",
    encryption: "../handlers/encryption",
    firewall: "../handlers/firewall",
    process: "../handlers/process",
    print: "../handlers/print",
    runtime: "../handlers/runtime",
    orbit: "../handlers/orbit",
    wine: "../handlers/wine",
    mail: "../handlers/mail",
    network: "../handlers/network",
    audio: "../handlers/audio",
    battery: "../handlers/battery",
    search: "../handlers/search",
    browser: "../handlers/runtime",
    window: "../handlers/runtime",
    permission: "../handlers/runtime",
    event: "../handlers/runtime",
    system: "../handlers/system",
    device: "../handlers/device",
    session: "../handlers/session",
    onboarding: "../handlers/onboarding",
    storage: "../handlers/storage",
    power: "../handlers/power",
    user: "../handlers/user",
    disk: "../handlers/disk",
    archive: "../handlers/archive",
    stt: "../handlers/stt",
    tts: "../handlers/tts",
    display: "../handlers/display"
  };

  function _load(namespace) {
    if (!_loaded[namespace]) {
      const modPath = _handlers[namespace];
      if (!modPath) return null;
      _loaded[namespace] = require(modPath);
    }
    return _loaded[namespace];
  }

  return {
    async route(message, context = {}) {
      const type = typeof message?.type === "string" ? message.type : "";
      const namespace = type.split(".")[0];
      const handler = _load(namespace);

      if (!handler) {
        return { type: "error", data: "unknown_type" };
      }

      const allowed = kernel.auth.authorize(message, kernel);
      if (!allowed) {
        return { type: "error", data: "permission_denied" };
      }

      return handler.handle(message, kernel, context);
    }
  };
}

module.exports = {
  createRouter
};
