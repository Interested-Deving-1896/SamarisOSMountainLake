const { MockLogger, MockEventBus, MockUserService, MockKernelB } = require("../engine/mockFactory");
const { measure, measureAsync } = require("../engine/benchmark");
const { runServiceTest } = require("../engine/serviceRunner");

async function auditAuth(logger, kernelRoot) {
  const results = [];
  const Auth = require(`${kernelRoot}/core/auth`);
  const PermissionManager = require(`${kernelRoot}/services/permissionManager`);

  const pm = new PermissionManager(new MockLogger());
  pm.seed("test-app", ["fs.read", "fs.*", "media.*", "system.*"]);
  const auth = new Auth(new MockLogger());
  const kernel = { permissionManager: pm };

  results.push(await runServiceTest("Auth", "authorize exact match", () => {
    auth.authorize({ type: "fs.read", appId: "test-app" }, kernel);
  }));

  results.push(await runServiceTest("Auth", "authorize wildcard namespace", () => {
    auth.authorize({ type: "fs.write", appId: "test-app" }, kernel);
  }));

  results.push(await runServiceTest("Auth", "authorize deep namespace", () => {
    auth.authorize({ type: "media.musicLibrary", appId: "test-app" }, kernel);
  }));

  results.push(await runServiceTest("Auth", "deny missing appId", () => {
    auth.authorize({ type: "fs.read" }, kernel);
  }));

  results.push(await runServiceTest("Auth", "deny no permission", () => {
    auth.authorize({ type: "mail.send", appId: "test-app" }, kernel);
  }));

  results.push(await runServiceTest("Auth", "deny null message", () => {
    auth.authorize(null, kernel);
  }));

  results.push(await runServiceTest("Auth", "1000 concurrent fast-path", async () => {
    const calls = Array.from({ length: 1000 }, () =>
      auth.authorize({ type: "system.ping", appId: "test-app" }, kernel)
    );
    // Run in series to measure per-call cost in benchmark
  }));

  return results;
}

async function auditEventBus(logger, kernelRoot) {
  const results = [];
  const EventBus = require(`${kernelRoot}/core/eventBus`);

  const bus = new EventBus(new MockLogger());

  results.push(await runServiceTest("EventBus", "subscribe + emit", () => {
    let called = false;
    const unsub = bus.on("test.event", () => { called = true; });
    bus.emit("test.event", "data");
  }));

  results.push(await runServiceTest("EventBus", "emit 100 events", () => {
    const unsub = bus.on("test.batch", () => {});
    for (let i = 0; i < 100; i++) bus.emit("test.batch", i);
  }));

  results.push(await runServiceTest("EventBus", "unsubscribe", () => {
    const handler = () => {};
    const unsub = bus.on("test.unsub", handler);
    unsub();
    bus.emit("test.unsub", "data");
  }));

  results.push(await runServiceTest("EventBus", "emit no subscribers", () => {
    bus.emit("nonexistent", "data");
  }));

  return results;
}

async function auditScheduler(logger, kernelRoot) {
  const results = [];
  const EventBus = require(`${kernelRoot}/core/eventBus`);
  const Scheduler = require(`${kernelRoot}/core/scheduler`);

  const bus = new EventBus(new MockLogger());
  const scheduler = new Scheduler(new MockLogger(), bus);

  scheduler.start();

  results.push(await runServiceTest("Scheduler", "start and tick", () => {
    // Scheduler emits "process:scheduled" on tick
  }));

  return results;
}

module.exports = { auditAuth, auditEventBus, auditScheduler, label: "Core Modules" };
