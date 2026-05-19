function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel, context = {}) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "fs.read":
      return {
        type: "fs.read.result",
        data: await kernel.fileSystem.read(payload.path || "/")
      };
    case "fs.write":
      return {
        type: "fs.write.result",
        data: await kernel.fileSystem.write(payload.path || "/tmp/kbt-file.txt", String(payload.content || ""))
      };
    case "fs.writeBase64":
      return {
        type: "fs.writeBase64.result",
        data: await kernel.fileSystem.writeBase64(payload.path || "/tmp/kbt-file.bin", String(payload.base64 || ""))
      };
    case "fs.readDataUrl":
      return {
        type: "fs.readDataUrl.result",
        data: await kernel.fileSystem.readDataUrl(payload.path || "/")
      };
    case "fs.list":
      return {
        type: "fs.list.result",
        data: await kernel.fileSystem.list(payload.path || "/")
      };
    case "fs.mkdir":
      return {
        type: "fs.mkdir.result",
        data: await kernel.fileSystem.mkdir(payload.path || "/tmp/kbt-folder")
      };
    case "fs.rename":
      return {
        type: "fs.rename.result",
        data: await kernel.fileSystem.rename(payload.from, payload.to)
      };
    case "fs.copy":
      return {
        type: "fs.copy.result",
        data: await kernel.fileSystem.copy(payload.from, payload.to)
      };
    case "fs.delete":
      return {
        type: "fs.delete.result",
        data: await kernel.fileSystem.remove(payload.path, Boolean(payload.recursive))
      };
    case "fs.watch":
      return {
        type: "fs.watch.result",
        data: await kernel.fileSystem.watch(payload.path || "/", context)
      };
    case "fs.unwatch":
      return {
        type: "fs.unwatch.result",
        data: await kernel.fileSystem.unwatch(payload.path || "/")
      };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = {
  handle
};
