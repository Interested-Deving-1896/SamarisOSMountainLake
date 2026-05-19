class EventBus {
  constructor(logger) {
    this.logger = logger;
    this.listeners = new Map();
    this.history = [];
  }

  on(eventName, listener) {
    const current = this.listeners.get(eventName) || [];
    current.push(listener);
    this.listeners.set(eventName, current);
    return () => {
      const next = (this.listeners.get(eventName) || []).filter((entry) => entry !== listener);
      this.listeners.set(eventName, next);
    };
  }

  emit(eventName, payload = {}) {
    this.logger.info("event:emit", eventName, payload);
    this.history.push({ event: eventName, payload, at: new Date().toISOString() });
    if (this.history.length > 100) {
      this.history.shift();
    }
    for (const listener of this.listeners.get(eventName) || []) {
      try {
        listener(payload);
      } catch (error) {
        this.logger.error("event:listener_error", error && error.stack ? error.stack : String(error));
      }
    }
  }

  listHistory() {
    return [...this.history];
  }
}

module.exports = EventBus;
