class WindowManager {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.windows = [];
  }

  openWindow(input = {}) {
    const window = {
      id: input.id || `window-${this.windows.length + 1}`,
      appId: input.appId || "unknown.app",
      title: input.title || "Untitled",
      x: input.x ?? 120,
      y: input.y ?? 96,
      w: input.w ?? 840,
      h: input.h ?? 520,
      focused: true,
      z: this.windows.length + 1
    };
    this.windows = this.windows.map((entry) => ({ ...entry, focused: false }));
    this.windows.push(window);
    this.eventBus.emit("window:focused", window);
    return window;
  }

  list() {
    return [...this.windows];
  }

  focus(id) {
    let focused = null;
    const topZ = this.windows.length + 1;
    this.windows = this.windows.map((entry) => {
      const next = { ...entry, focused: entry.id === id, z: entry.id === id ? topZ : entry.z };
      if (next.focused) focused = next;
      return next;
    });
    if (focused) {
      this.eventBus.emit("window:focused", focused);
    }
    return focused;
  }

  replaceAll(nextWindows = []) {
    this.windows = [...nextWindows];
    return this.list();
  }
}

module.exports = WindowManager;
