function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel, context = {}) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "orbit.status":
      return {
        type: "orbit.status.result",
        data: await kernel.orbit.status()
      };
    case "orbit.generate":
      return {
        type: "orbit.generate.result",
        data: await kernel.orbit.generate(
          {
            prompt: String(payload.prompt || ""),
            modeId: payload.modeId || "general",
            strategy: payload.strategy || "self-consistency",
            requestId: message.requestId
          },
          context
        )
      };
    case "orbit.shutdown":
      return {
        type: "orbit.shutdown.result",
        data: kernel.orbit.stop()
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
