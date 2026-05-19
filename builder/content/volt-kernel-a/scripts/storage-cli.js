#!/usr/bin/env node
const fs = require("node:fs");
const path = require("node:path");
const FileSystemService = require("../services/fileSystem");
const StorageService = require("../services/storageService");

async function readStdin() {
  const chunks = [];
  for await (const chunk of process.stdin) {
    chunks.push(Buffer.from(chunk));
  }
  return Buffer.concat(chunks).toString("utf8").trim();
}

async function main() {
  const command = process.argv[2] || "status";
  const logger = {
    info: (...args) => process.stderr.write(`[storage-cli] ${args.map(String).join(" ")}\n`),
    warn: (...args) => process.stderr.write(`[storage-cli] ${args.map(String).join(" ")}\n`),
    error: (...args) => process.stderr.write(`[storage-cli] ${args.map(String).join(" ")}\n`)
  };
  const eventBus = { emit() {} };
  const fileSystem = new FileSystemService(logger, eventBus);
  const storage = new StorageService(logger, eventBus, fileSystem);
  await storage.init();

  let result;
  switch (command) {
    case "status":
      result = await storage.status();
      break;
    case "setup":
      result = await storage.setupFirstBoot({ password: await readStdin() });
      break;
    case "unlock":
      result = await storage.unlockUserStorage(await readStdin());
      break;
    case "devices":
      result = await storage.listExternalDevices();
      break;
    case "mount":
      result = await storage.mountExternal(process.argv[3] || "");
      break;
    case "unmount":
      result = await storage.unmountExternal(process.argv[3] || "");
      break;
    default:
      process.stderr.write(`Unknown command: ${command}\n`);
      process.exit(1);
  }

  process.stdout.write(`${JSON.stringify(result, null, 2)}\n`);
}

main().catch((error) => {
  process.stderr.write(`${error instanceof Error ? error.stack || error.message : String(error)}\n`);
  process.exit(1);
});
