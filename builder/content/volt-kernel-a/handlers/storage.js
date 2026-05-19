function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "storage.status":
      return { type: "storage.status.result", data: await kernel.storage.status() };
    case "storage.setupFirstBoot":
      return { type: "storage.setupFirstBoot.result", data: await kernel.storage.setupFirstBoot(payload) };
    case "storage.unlockUser":
      return { type: "storage.unlockUser.result", data: await kernel.storage.unlockUserStorage(payload.password || "") };
    case "storage.devices":
      return { type: "storage.devices.result", data: await kernel.storage.listExternalDevices() };
    case "storage.mount":
      return { type: "storage.mount.result", data: await kernel.storage.mountExternal(payload.path || "") };
    case "storage.unmount":
      return { type: "storage.unmount.result", data: await kernel.storage.unmountExternal(payload.path || "") };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
