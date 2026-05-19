class Scheduler {
  constructor(logger, eventBus) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.startedAt = null;
    this.tickCount = 0;
  }

  start() {
    this.startedAt = Date.now();
    this.logger.info("scheduler:start");
  }

  tick(reason = "manual") {
    this.tickCount += 1;
    this.eventBus.emit("process:scheduled", {
      tick: this.tickCount,
      reason
    });
    return {
      tick: this.tickCount,
      reason
    };
  }

  snapshot() {
    return {
      startedAt: this.startedAt,
      tickCount: this.tickCount
    };
  }
}

module.exports = Scheduler;
