async function handle(message, kernel) {
  const { powerService } = kernel;
  switch (message.type) {
    case "power.shutdown":
      return { type: "power.shutdown.result", data: await powerService.shutdown() };
    case "power.restart":
      return { type: "power.restart.result", data: await powerService.restart() };
    case "power.sleep":
      return { type: "power.sleep.result", data: await powerService.sleep() };
    case "power.lock":
      return { type: "power.lock.result", data: await powerService.lock() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
