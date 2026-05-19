function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);
  switch (message.type) {
    case "search.query":
      return { type: "search.query.result", data: await kernel.search.query(payload, kernel) };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
