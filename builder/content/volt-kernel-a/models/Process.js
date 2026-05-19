class Process {
  constructor({
    pid,
    appId,
    runtime = "app",
    state = "running",
    priority = "normal",
    cpu = 0,
    memory = 0,
    permissions = []
  }) {
    this.pid = pid;
    this.appId = appId;
    this.runtime = runtime;
    this.state = state;
    this.priority = priority;
    this.cpu = cpu;
    this.memory = memory;
    this.permissions = permissions;
  }
}

module.exports = Process;
