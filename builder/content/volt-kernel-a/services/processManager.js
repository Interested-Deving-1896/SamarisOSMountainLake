const Process = require("../models/Process");

class ProcessManager {
  constructor(logger, eventBus, scheduler) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.scheduler = scheduler;
    this.nextPid = 1000;
    this.processes = new Map();
  }

  createProcess(input = {}) {
    const process = new Process({
      pid: this.nextPid += 1,
      appId: input.appId || "unknown.app",
      runtime: input.runtime || "app",
      priority: input.priority || "normal",
      permissions: input.permissions || []
    });
    this.processes.set(process.pid, process);
    this.eventBus.emit("process:created", process);
    return process;
  }

  list() {
    return [...this.processes.values()];
  }

  find(pid) {
    return this.processes.get(Number(pid)) || null;
  }

  updateState(pid, state) {
    const process = this.find(pid);
    if (!process) return null;
    process.state = state;
    this.scheduler.tick(`process:${state}`);
    if (state === "terminated") {
      this.eventBus.emit("process:killed", process);
    }
    return process;
  }

  pause(pid) {
    return this.updateState(pid, "paused");
  }

  resume(pid) {
    return this.updateState(pid, "running");
  }

  kill(pid) {
    return this.updateState(pid, "terminated");
  }
}

module.exports = ProcessManager;
