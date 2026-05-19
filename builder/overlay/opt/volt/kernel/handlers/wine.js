function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel, context = {}) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "wine.status":
      return { type: "wine.status.result", data: await kernel.wine.status() };
    case "wine.launch":
      return {
        type: "wine.launch.result",
        data: await kernel.wine.launchExe(String(payload.exePath || ""), payload.options || {}, context)
      };
    case "wine.config":
      return {
        type: "wine.config.result",
        data: await kernel.wine.launchConfig(payload.options || {}, context)
      };
    case "wine.stop":
      return {
        type: "wine.stop.result",
        data: await kernel.wine.stopSession(String(payload.sessionId || ""))
      };
    case "wine.logs":
      return {
        type: "wine.logs.result",
        data: await kernel.wine.getSessionLogs(String(payload.sessionId || ""))
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
