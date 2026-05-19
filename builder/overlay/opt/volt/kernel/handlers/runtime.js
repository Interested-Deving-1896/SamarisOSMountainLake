function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "runtime.list":
      return { type: "runtime.list.result", data: kernel.runtimeManager.list() };
    case "runtime.start":
      return { type: "runtime.start.result", data: kernel.runtimeManager.startRuntime(payload) };
    case "runtime.stop":
      return { type: "runtime.stop.result", data: kernel.runtimeManager.stopRuntime(payload.id) };
    case "browser.status":
      return { type: "browser.status.result", data: await kernel.browser.status() };
    case "browser.navigate":
      return { type: "browser.navigate.result", data: await kernel.browser.launch(payload.url || "about:blank") };
    case "window.list":
      return { type: "window.list.result", data: kernel.windowManager.list() };
    case "window.open":
      return { type: "window.open.result", data: kernel.windowManager.openWindow(payload) };
    case "window.focus":
      return { type: "window.focus.result", data: kernel.windowManager.focus(payload.id) };
    case "permission.check":
      return {
        type: "permission.check.result",
        data: {
          allowed: kernel.permissionManager.can(payload.appId || "unknown.app", payload.action || "unknown.action")
        }
      };
    case "permission.list":
      return {
        type: "permission.list.result",
        data: kernel.permissionManager.list(payload.appId || "volt.desktop")
      };
    case "permission.listAll":
      return {
        type: "permission.listAll.result",
        data: { error: "permission_denied" }
      };
    case "permission.set":
      return {
        type: "permission.set.result",
        data: { error: "permission_denied" }
      };
    case "event.emit":
      kernel.eventBus.emit(payload.name || "event:custom", payload.payload || {});
      return { type: "event.emit.result", data: { ok: true } };
    case "event.history":
      return { type: "event.history.result", data: kernel.eventBus.listHistory() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
