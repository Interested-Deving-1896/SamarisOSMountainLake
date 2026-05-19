function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "disk.status":
      return { type: "disk.status.result", data: await kernel.disk.getStorage() };
    case "disk.list":
      return { type: "disk.list.result", data: await kernel.disk.listDisks() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
