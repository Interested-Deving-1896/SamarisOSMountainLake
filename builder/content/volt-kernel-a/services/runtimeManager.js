const Runtime = require("../models/Runtime");

class RuntimeManager {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.runtimes = new Map();
  }

  startRuntime(input = {}) {
    const runtime = new Runtime({
      id: input.id || `runtime-${this.runtimes.size + 1}`,
      kind: input.kind || "appRuntime",
      target: input.target || null
    });
    this.runtimes.set(runtime.id, runtime);
    this.eventBus.emit("runtime:started", runtime);
    return runtime;
  }

  list() {
    return [...this.runtimes.values()];
  }

  stopRuntime(id) {
    const runtime = this.runtimes.get(id) || null;
    if (!runtime) return null;
    runtime.state = "stopped";
    return runtime;
  }

  browserStatus() {
    return {
      connected: true,
      engine: "chromium",
      tabs: []
    };
  }

  navigate(url) {
    this.logger.info("browser:navigate", url);
    this.eventBus.emit("browser:navigated", { url });
    return { ok: true, url };
  }
}

module.exports = RuntimeManager;
