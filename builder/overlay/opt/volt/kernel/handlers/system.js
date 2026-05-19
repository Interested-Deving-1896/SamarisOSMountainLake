function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  switch (message.type) {
    case "system.ping":
      return { type: "system.pong", data: "ok" };
    case "system.info":
      return {
        type: "system.info.result",
        data: {
          name: "Samaris Kernel V3",
          scheduler: kernel.scheduler.snapshot(),
          uptimeMs: kernel.scheduler.startedAt ? Date.now() - kernel.scheduler.startedAt : 0
        }
      };
    case "system.state":
      return {
        type: "system.state.result",
        data: kernel.getPublicState()
      };
    case "system.tick":
      return {
        type: "system.tick.result",
        data: kernel.scheduler.tick(payloadOf(message).reason || "manual")
      };
    case "system.metrics":
      return {
        type: "system.metrics.result",
        data: await kernel.systemMetrics.getMetrics()
      };
    case "system.checkUpdate":
      return {
        type: "system.checkUpdate.result",
        data: {
          currentVersion: "v1.0.0-alpha",
          latestVersion: null,
          upToDate: true,
          checkedAt: new Date().toISOString()
        }
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
