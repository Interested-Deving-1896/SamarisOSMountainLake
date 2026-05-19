function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "audio.status":
      return { type: "audio.status.result", data: await kernel.audio.getStatus() };
    case "audio.volume":
      return { type: "audio.volume.result", data: await kernel.audio.setVolume(payload) };
    case "audio.listOutputs":
      return { type: "audio.listOutputs.result", data: await kernel.audio.getStatus() };
    case "audio.setOutput":
      return { type: "audio.setOutput.result", data: await kernel.audio.setOutput(payload) };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
