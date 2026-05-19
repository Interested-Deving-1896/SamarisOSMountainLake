function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel, context) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "user.create":
      return {
        type: "user.create.result",
        data: await kernel.user.create(payload.username, payload.displayName, payload.password),
      };
    case "user.login": {
      const userData = await kernel.user.login(payload.username, payload.password);
      if (userData && !userData.guest) {
        kernel.fileSystem.setActiveUser(userData.username);
      }
      return { type: "user.login.result", data: userData };
    }
    case "user.list":
      return { type: "user.list.result", data: await kernel.user.list() };
    case "user.active":
      return { type: "user.active.result", data: kernel.user.getActiveUser() };
    case "user.delete":
      await kernel.user.delete(payload.username);
      return { type: "user.delete.result", data: { ok: true } };
    case "user.update":
      return {
        type: "user.update.result",
        data: await kernel.user.updateProfile(payload.username, payload),
      };
    case "user.logout":
      kernel.user.clearSession();
      return { type: "user.logout.result", data: { ok: true } };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
