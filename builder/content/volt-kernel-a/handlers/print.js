function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "print.list":
      return { type: "print.list.result", data: await kernel.print.list() };
    case "print.add":
      return { type: "print.add.result", data: await kernel.print.add(payload) };
    case "print.remove":
      return { type: "print.remove.result", data: await kernel.print.remove(payload.printerId) };
    case "print.submit":
      return { type: "print.submit.result", data: await kernel.print.submit(payload) };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
