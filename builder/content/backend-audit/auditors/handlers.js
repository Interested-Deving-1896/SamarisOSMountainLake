const { runHandlerTest } = require("../engine/handlerRunner");
const { MockLogger, MockEventBus, MockUserService, MockKernelB } = require("../engine/mockFactory");
const { createTempFileSystem, destroyTempFileSystem } = require("../engine/mockFactory");

async function buildKernelStub(logger, kernelRoot) {
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const Auth = require(`${kernelRoot}/core/auth`);
  const PM = require(`${kernelRoot}/services/permissionManager`);
  const EventBus = require(`${kernelRoot}/core/eventBus`);
  const tempFs = await createTempFileSystem();
  const pm = new PM(logger);
  pm.seed("test-app", ["*"]);
  const auth = new Auth(logger);
  const fsService = new FS(logger, new MockEventBus(), new MockUserService(), new MockKernelB());
  fsService.userRootPath = tempFs.userRoot;
  fsService.virtualRoots["/User"] = tempFs.userRoot;
  const bus = new EventBus(logger);
  bus.emit = () => {};

  const kernel = {
    auth, permissionManager: pm, fileSystem: fsService, eventBus: bus,
    scheduler: { snapshot() { return {}; }, startedAt: Date.now(), tick() { return 0; } },
    apps: [], devices: [], session: { lastBootAt: new Date().toISOString() },
    getPublicState() { return { processes: [], windows: [], runtimes: [], devices: [], session: {} }; },
    audio: { getStatus() { return {}; }, setVolume() {}, setOutput() {} },
    battery: { getStatus() { return {}; } },
    archive: { async listContents() { return { ok: true, entries: [] }; }, async extract() { return { ok: true }; } },
    appStore: { async listInstalled() { return []; }, async clone() { return { ok: true }; }, async build() { return { ok: true }; } },
    appStaticServer: { stopAll() {}, getAppPort() { return null; }, startApp() { return { port: 19000 }; }, stopApp() { return { stopped: true }; } },
    user: new (require(`${kernelRoot}/services/userService`))(logger),
    vault: new (require(`${kernelRoot}/services/vaultService`))(logger, new MockUserService(), new MockKernelB()),
    connectivity: new (require(`${kernelRoot}/services/connectivityService`))(logger, bus, new MockUserService()),
    network: { async list() { return []; }, setConfig() {} },
    browser: { status() { return {}; }, resolveBinary() { return {}; }, sessions: new Map() },
    wine: { async status() { return { installed: false }; } },
    orbit: { async status() { return {}; } },
    disk: { async getStorage() { return {}; }, async listDisks() { return []; } },
    firewall: new (require(`${kernelRoot}/services/firewallService`))(logger),
    print: new (require(`${kernelRoot}/services/printService`))(logger, fsService),
    encryption: { async status() { return {}; } },
    search: { async query() { return []; } },
    powerService: { shutdown() {}, restart() {}, sleep() {}, lock() {} },
    systemMetrics: { getMetrics() { return {}; } },
    devState: { getResetState() { return {}; } },
    storage: { async loadState() { return {}; }, async status() { return {}; } },
    media: { async listMusicLibrary() { return []; }, async listVideoLibrary() { return []; } },
    onboarding: { async get() { return {}; }, async patch() {} },
    stt: { async transcribe() { return { text: "" }; } },
    tts: { async speak() { return { ok: true }; } },
    kernelB: new MockKernelB(),
    runtimeManager: { list() { return []; }, startRuntime() {} },
    processManager: { list() { return []; }, createProcess() {} },
    windowManager: { list() { return []; }, openWindow() {} },
    sessionFeatures: {},
    tempFs,
  };
  return kernel;
}

async function auditSystemHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/system`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("system", "ping", handler, { type: "system.ping", appId: "test-app" }, kernel));
  results.push(await runHandlerTest("system", "info", handler, { type: "system.info", appId: "test-app" }, kernel));
  results.push(await runHandlerTest("system", "state", handler, { type: "system.state", appId: "test-app" }, kernel));
  results.push(await runHandlerTest("system", "metrics", handler, { type: "system.metrics", appId: "test-app" }, kernel));
  await destroyTempFileSystem(kernel.tempFs.root);
  return results;
}

async function auditFsHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/fs`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("fs", "write", handler, { type: "fs.write", data: { path: "/User/Documents/handler-fs-test.txt", content: "hi" }, appId: "test-app" }, kernel));
  results.push(await runHandlerTest("fs", "read", handler, { type: "fs.read", data: { path: "/User/Documents/handler-fs-test.txt" }, appId: "test-app" }, kernel));
  results.push(await runHandlerTest("fs", "list", handler, { type: "fs.list", data: { path: "/User" }, appId: "test-app" }, kernel));
  results.push(await runHandlerTest("fs", "mkdir", handler, { type: "fs.mkdir", data: { path: "/User/Documents/new-dir" }, appId: "test-app" }, kernel));
  results.push(await runHandlerTest("fs", "write", handler, { type: "fs.write", data: { path: "/User/Documents/h.txt", content: "hi" }, appId: "test-app" }, kernel));
  await destroyTempFileSystem(kernel.tempFs.root);
  return results;
}

async function auditAudioHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/audio`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("audio", "status", handler, { type: "audio.status", appId: "test-app" }, kernel));
  results.push(await runHandlerTest("audio", "setVolume", handler, { type: "audio.volume", data: { volume: 0.5 }, appId: "test-app" }, kernel));
  return results;
}

async function auditBatteryHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/battery`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("battery", "status", handler, { type: "battery.status", appId: "test-app" }, kernel));
  return results;
}

async function auditArchiveHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/archive`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("archive", "list", handler, { type: "archive.list", data: { archivePath: "/test.zip" }, appId: "test-app" }, kernel));
  return results;
}

async function auditMediaHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/media`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("media", "musicLibrary", handler, { type: "media.musicLibrary", appId: "test-app" }, kernel));
  results.push(await runHandlerTest("media", "videoLibrary", handler, { type: "media.videoLibrary", appId: "test-app" }, kernel));
  return results;
}

async function auditUserHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/user`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("user", "list", handler, { type: "user.list", appId: "test-app" }, kernel));
  return results;
}

async function auditSessionHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/session`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("session", "get", handler, { type: "session.get", appId: "test-app" }, kernel));
  return results;
}

async function auditDeviceHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/device`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("device", "status", handler, { type: "device.status", appId: "test-app" }, kernel));
  return results;
}

async function auditDisplayHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/display`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("display", "status", handler, { type: "display.status", appId: "test-app" }, kernel));
  return results;
}

async function auditPowerHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/power`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("power", "status", handler, { type: "power.status", appId: "test-app" }, kernel));
  return results;
}

async function auditNetworkHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/network`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("network", "list", handler, { type: "network.list", appId: "test-app" }, kernel));
  return results;
}

async function auditAppHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/app`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("app", "listInstalled", handler, { type: "app.listInstalled", appId: "test-app" }, kernel));
  return results;
}

async function auditRuntimeHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/runtime`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("runtime", "list", handler, { type: "runtime.list", appId: "test-app" }, kernel));
  return results;
}

async function auditProcessHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/process`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("process", "list", handler, { type: "process.list", appId: "test-app" }, kernel));
  return results;
}

async function auditSearchHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/search`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("search", "query", handler, { type: "search.query", data: { query: "test" }, appId: "test-app" }, kernel));
  return results;
}

async function auditFirewallHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/firewall`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("firewall", "list", handler, { type: "firewall.list", appId: "test-app" }, kernel));
  return results;
}

async function auditDiskHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/disk`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("disk", "status", handler, { type: "disk.status", appId: "test-app" }, kernel));
  return results;
}

async function auditStorageHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/storage`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("storage", "status", handler, { type: "storage.status", appId: "test-app" }, kernel));
  return results;
}

async function auditPrintHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/print`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("print", "list", handler, { type: "print.list", appId: "test-app" }, kernel));
  return results;
}

async function auditOnboardingHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/onboarding`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("onboarding", "get", handler, { type: "onboarding.get", appId: "test-app" }, kernel));
  return results;
}

async function auditEncryptionHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/encryption`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("encryption", "status", handler, { type: "encryption.status", appId: "test-app" }, kernel));
  return results;
}

async function auditTTSHandler(logger, kernelRoot) {
  const results = [];
  try {
    const handler = require(`${kernelRoot}/handlers/tts`);
    const kernel = await buildKernelStub(logger, kernelRoot);
    results.push(await runHandlerTest("tts", "speak", handler, { type: "tts.speak", data: { text: "hello" }, appId: "test-app" }, kernel));
  } catch {
    const { HandlerTestResult } = require("../engine/handlerRunner");
    results.push(new HandlerTestResult("tts", "speak").skipped("Handler module not found"));
  }
  return results;
}

async function auditSTTHandler(logger, kernelRoot) {
  const results = [];
  try {
    const handler = require(`${kernelRoot}/handlers/stt`);
    const kernel = await buildKernelStub(logger, kernelRoot);
    results.push(await runHandlerTest("stt", "transcribe", handler, { type: "stt.transcribe", data: { audio: "" }, appId: "test-app" }, kernel));
  } catch {
    const { HandlerTestResult } = require("../engine/handlerRunner");
    results.push(new HandlerTestResult("stt", "transcribe").skipped("Handler module not found"));
  }
  return results;
}

async function auditWineHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/wine`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("wine", "status", handler, { type: "wine.status", appId: "test-app" }, kernel));
  return results;
}

async function auditOrbitHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/orbit`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("orbit", "status", handler, { type: "orbit.status", appId: "test-app" }, kernel));
  return results;
}

async function auditBrowserHandler(logger, kernelRoot) {
  const results = [];
  const handler = require(`${kernelRoot}/handlers/runtime`);
  const kernel = await buildKernelStub(logger, kernelRoot);
  results.push(await runHandlerTest("browser", "status", handler, { type: "browser.status", appId: "test-app" }, kernel));
  return results;
}

module.exports = {
  auditSystemHandler, auditFsHandler, auditAudioHandler, auditBatteryHandler,
  auditArchiveHandler, auditMediaHandler, auditUserHandler, auditSessionHandler,
  auditDeviceHandler, auditDisplayHandler, auditPowerHandler, auditNetworkHandler,
  auditAppHandler, auditRuntimeHandler, auditProcessHandler, auditSearchHandler,
  auditFirewallHandler, auditDiskHandler, auditStorageHandler, auditPrintHandler,
  auditOnboardingHandler, auditEncryptionHandler, auditTTSHandler, auditSTTHandler,
  auditWineHandler, auditOrbitHandler, auditBrowserHandler,
  label: "Handlers",
};
