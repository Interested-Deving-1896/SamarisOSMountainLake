class IPC {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
  }

  send(channel, payload = {}) {
    this.logger.info("ipc:send", channel, payload);
    this.eventBus.emit(`ipc:${channel}`, payload);
    return { ok: true, channel, payload };
  }

  request(channel, payload = {}) {
    this.logger.info("ipc:request", channel, payload);
    return { ok: true, channel, payload, mocked: true };
  }
}

module.exports = IPC;
