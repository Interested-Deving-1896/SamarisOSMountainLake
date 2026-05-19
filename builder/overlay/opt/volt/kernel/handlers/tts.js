function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "tts.speak":
      return {
        type: "tts.speak.result",
        data: await kernel.tts.speak(payload)
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
