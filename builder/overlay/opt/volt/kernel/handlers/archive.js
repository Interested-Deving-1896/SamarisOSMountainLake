function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "archive.extract":
      return {
        type: "archive.extract.result",
        data: await kernel.archive.extract(payload.archivePath || "", payload.destDir || "")
      };
    case "archive.list":
      return {
        type: "archive.list.result",
        data: await kernel.archive.listContents(payload.archivePath || "")
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
