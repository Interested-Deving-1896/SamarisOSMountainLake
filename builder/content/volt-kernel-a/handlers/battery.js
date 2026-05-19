async function handle(message, kernel) {
  switch (message.type) {
    case "battery.status":
      return { type: "battery.status.result", data: await kernel.battery.getStatus() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
