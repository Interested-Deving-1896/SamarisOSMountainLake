function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "media.musicLibrary":
      return {
        type: "media.musicLibrary.result",
        data: await kernel.media.listMusicLibrary(payload)
      };
    case "media.videoLibrary":
      return {
        type: "media.videoLibrary.result",
        data: await kernel.media.listVideoLibrary(payload)
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
