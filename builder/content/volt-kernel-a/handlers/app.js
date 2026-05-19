function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "app.listInstalled":
      return { type: "app.listInstalled.result", data: await kernel.appStore.listInstalled() };
    case "app.clone":
      return { type: "app.clone.result", data: await kernel.appStore.clone(payload) };
    case "app.build":
      return { type: "app.build.result", data: await kernel.appStore.build(payload) };
    case "app.update":
      return { type: "app.update.result", data: await kernel.appStore.update(payload) };
    case "app.remove":
      return { type: "app.remove.result", data: await kernel.appStore.remove(payload) };
    case "app.start":
      return { type: "app.start.result", data: await kernel.appStore.startApp(payload) };
    case "app.stop":
      return { type: "app.stop.result", data: await kernel.appStore.stopApp(payload) };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
