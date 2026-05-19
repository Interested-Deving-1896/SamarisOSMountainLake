const path = require("node:path");
const fs = require("node:fs/promises");
const os = require("node:os");
const { EventEmitter } = require("node:events");

let tempCounter = 0;

class MockLogger {
  info() {}
  warn() {}
  error() {}
  debug() {}
}

class MockEventBus {
  constructor() {
    this.listeners = new Map();
  }
  on(event, handler) {
    if (!this.listeners.has(event)) this.listeners.set(event, new Set());
    this.listeners.get(event).add(handler);
    return () => this.listeners.get(event)?.delete(handler);
  }
  emit(event, data) {
    const handlers = this.listeners.get(event);
    if (handlers) for (const h of handlers) h(data);
  }
  subscribe(listener) {
    return () => {};
  }
}

class MockUserService {
  constructor() {
    this._activeUser = { username: "testuser", displayName: "Test User" };
  }
  getActiveUser() { return this._activeUser; }
  getVaultIdentity() { return { username: "testuser", secret: "test-password" }; }
  resolveHome(username) { return path.join(os.tmpdir(), `samaris-audit-home-${username}`); }
  resolveUserPath(vp) { return vp; }
}

class MockKernelB {
  available() { return false; }
  async call() { throw new Error("kernel_b_not_available"); }
}

async function createTempFileSystem() {
  const root = path.join(os.tmpdir(), `samaris-audit-fs-${Date.now()}-${tempCounter++}`);
  const userRoot = path.join(root, "user");
  const volumeRoot = path.join(root, "volumes");
  await fs.mkdir(userRoot, { recursive: true });
  await fs.mkdir(volumeRoot, { recursive: true });
  const folders = ["Desktop", "Documents", "Downloads", "Music", "Pictures", "Photos", "Videos", "Applications", "AppData", "Trash", ".samaris"];
  await Promise.all(folders.map((f) => fs.mkdir(path.join(userRoot, f), { recursive: true })));
  return { root, userRoot, volumeRoot };
}

async function destroyTempFileSystem(root) {
  await fs.rm(root, { recursive: true, force: true }).catch(() => {});
}

module.exports = {
  MockLogger,
  MockEventBus,
  MockUserService,
  MockKernelB,
  createTempFileSystem,
  destroyTempFileSystem,
};
