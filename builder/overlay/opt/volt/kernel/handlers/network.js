function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "network.list":
      return {
        type: "network.list.result",
        data: await kernel.network.list()
      };
    case "network.setConfig":
      return {
        type: "network.setConfig.result",
        data: await kernel.network.setConfig(payload)
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
