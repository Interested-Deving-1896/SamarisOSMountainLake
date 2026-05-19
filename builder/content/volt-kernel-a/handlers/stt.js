function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "stt.transcribe":
      return {
        type: "stt.transcribe.result",
        data: await kernel.stt.transcribe(payload)
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
