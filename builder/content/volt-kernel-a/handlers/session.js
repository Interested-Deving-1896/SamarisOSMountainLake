function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "session.get":
      return { type: "session.get.result", data: { ...kernel.session } };
    case "session.save":
      kernel.session = {
        ...kernel.session,
        lastSavedAt: new Date().toISOString(),
        snapshot: payload.snapshot || kernel.getPublicState()
      };
      return { type: "session.save.result", data: { ok: true, session: kernel.session } };
    case "session.restore":
      kernel.session = {
        ...kernel.session,
        restored: true,
        restoredAt: new Date().toISOString()
      };
      return { type: "session.restore.result", data: { ok: true, session: kernel.session } };
    case "session.security.get":
      return { type: "session.security.get.result", data: await kernel.sessionFeatures.get() };
    case "session.security.set":
      return { type: "session.security.set.result", data: await kernel.sessionFeatures.set(payload) };
    case "session.lock":
      return { type: "session.lock.result", data: await kernel.sessionFeatures.lock() };
    case "session.unlock": {
      const unlockResult = await kernel.sessionFeatures.unlock(payload);
      if (unlockResult.ok && kernel.user && kernel.user.getActiveUser()) {
        const active = kernel.user.getActiveUser();
        if (active && !active.guest) {
          kernel.fileSystem.setActiveUser(active.username);
        }
      }
      return { type: "session.unlock.result", data: unlockResult };
    }
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
