function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "process.list":
      return { type: "process.list.result", data: kernel.processManager.list() };
    case "process.create":
      return { type: "process.create.result", data: kernel.processManager.createProcess(payload) };
    case "process.pause":
      return { type: "process.pause.result", data: kernel.processManager.pause(payload.pid) };
    case "process.resume":
      return { type: "process.resume.result", data: kernel.processManager.resume(payload.pid) };
    case "process.kill":
      return { type: "process.kill.result", data: kernel.processManager.kill(payload.pid) };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
