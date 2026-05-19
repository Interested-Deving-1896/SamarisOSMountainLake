const { runServiceTest } = require("../engine/serviceRunner");
const { measureAsync } = require("../engine/benchmark");
const { MockLogger } = require("../engine/mockFactory");

async function auditUnifierConstants(logger, kernelRoot) {
  const results = [];

  results.push(await runServiceTest("Unifier Constants", "load constants", () => {
    require(`${kernelRoot}/volt-unifier/constants`);
  }));

  results.push(await runServiceTest("Unifier Errors", "load errors", () => {
    require(`${kernelRoot}/volt-unifier/errors`);
  }));

  return results;
}

async function auditSbpMessage(logger, kernelRoot) {
  const results = [];

  const { SbpMessage } = require(`${kernelRoot}/volt-unifier/sbp/message`);

  results.push(await runServiceTest("SBP Message", "create", () => {
    SbpMessage.create(0x01, 0, Buffer.from("hello"));
  }));

  results.push(await runServiceTest("SBP Message", "encode then decode", () => {
    const msg = SbpMessage.create(0x01, 0, Buffer.from("test-payload"));
    const buf = msg.encode();
    SbpMessage.fromBuffer(buf);
  }));

  results.push(await runServiceTest("SBP Message", "1000 encode/decode cycle", () => {
    for (let i = 0; i < 1000; i++) {
      const msg = SbpMessage.create(0x01, 0, Buffer.from(`msg-${i}`));
      SbpMessage.fromBuffer(msg.encode());
    }
  }));

  results.push(await runServiceTest("SBP Message", "64KB payload round-trip", () => {
    const payload = Buffer.from("x".repeat(65536));
    const msg = SbpMessage.create(0x01, 0, payload);
    const decoded = SbpMessage.fromBuffer(msg.encode());
    if (decoded.payload.length !== 65536) throw new Error("payload_mismatch");
  }));

  results.push(await runServiceTest("SBP Message", "isRequest / isResponse", () => {
    const req = SbpMessage.create(0x01, 0x01, Buffer.from("req"));
    req.isRequest();
    req.isResponse();
  }));

  return results;
}

async function auditSbpRouter(logger, kernelRoot) {
  const results = [];

  const { SbpRouter } = require(`${kernelRoot}/volt-unifier/sbp/router`);
  const unifier = { getModule() { return null; } };
  const router = new SbpRouter(unifier);

  results.push(await runServiceTest("SBP Router", "instantiation", () => {
    new SbpRouter(unifier);
  }));

  results.push(await runServiceTest("SBP Router", "subscribe + handleIncoming", () => {
    const sub = router.subscribe("test-module", 0x08, () => {});
    sub();
  }));

  return results;
}

async function auditModuleRegistry(logger, kernelRoot) {
  const results = [];

  const { ModuleRegistry } = require(`${kernelRoot}/volt-unifier/registry/moduleRegistry`);

  results.push(await runServiceTest("ModuleRegistry", "register + get", () => {
    const reg = new ModuleRegistry(null);
    reg.register("test", { endpoint: "tcp://127.0.0.1:9000" });
    reg.get("test");
  }));

  results.push(await runServiceTest("ModuleRegistry", "updateStatus", () => {
    const reg = new ModuleRegistry(null);
    reg.register("test", { endpoint: "tcp://127.0.0.1:9000" });
    reg.updateStatus("test", "online", { lastHeartbeatAt: Date.now() });
  }));

  results.push(await runServiceTest("ModuleRegistry", "getAll + count", () => {
    const reg = new ModuleRegistry(null);
    reg.register("a", { endpoint: "tcp://127.0.0.1:9000" });
    reg.register("b", { endpoint: "tcp://127.0.0.1:9001" });
    reg.getAll();
  }));

  results.push(await runServiceTest("ModuleRegistry", "register 100 modules and enumerate", () => {
    const reg = new ModuleRegistry(null);
    for (let i = 0; i < 100; i++) reg.register(`mod-${i}`, { endpoint: `tcp://127.0.0.1:${9000 + i}` });
    const all = reg.getAll();
    if (typeof all.size === "number" ? all.size !== 100 : all.length !== 100) throw new Error("registry_count_mismatch");
  }));

  return results;
}

async function auditEventBus(logger, kernelRoot) {
  const results = [];

  const { SystemEventBus } = require(`${kernelRoot}/volt-unifier/events/eventBus`);
  const bus = new SystemEventBus({ emit() {} });
  const { createEvent } = require(`${kernelRoot}/volt-unifier/events/eventTypes`);

  results.push(await runServiceTest("Unifier EventBus", "publish + subscribe", () => {
    let called = false;
    const id = bus.subscribe("test", (msg) => { called = true; });
    bus.publish(createEvent("test", "audit-module", { data: 1 }));
    bus.unsubscribe(id);
  }));

  results.push(await runServiceTest("Unifier EventBus", "publish no subscribers", () => {
    bus.publish(createEvent("test-nosub", "audit-module", { data: 2 }));
  }));

  results.push(await runServiceTest("Unifier EventBus", "publish 100 events", () => {
    for (let i = 0; i < 100; i++) {
      bus.publish(createEvent(`evt-${i}`, "audit-module", { index: i }));
    }
  }));

  return results;
}

async function auditCapabilityGuard(logger, kernelRoot) {
  const results = [];

  const { CapabilityGuard } = require(`${kernelRoot}/volt-unifier/safety/capabilityGuard`);

  results.push(await runServiceTest("CapabilityGuard", "isSensitiveCommand", () => {
    CapabilityGuard.isSensitiveCommand("shutdown");
  }));

  results.push(await runServiceTest("CapabilityGuard", "requireCapability match", () => {
    CapabilityGuard.requireCapability({ features: ["fs.read"] }, "fs.read");
  }));

  results.push(await runServiceTest("CapabilityGuard", "requireCapability wildcard", () => {
    CapabilityGuard.requireCapability({ features: ["fs.*"] }, "fs.read");
  }));

  return results;
}

async function auditLifecycle(logger, kernelRoot) {
  const results = [];

  results.push(await runServiceTest("Unifier Lifecycle", "ReadinessState", () => {
    const { ReadinessState } = require(`${kernelRoot}/volt-unifier/lifecycle/readiness`);
    new ReadinessState();
  }));

  results.push(await runServiceTest("Unifier Lifecycle", "ShutdownOrchestrator", () => {
    const { ShutdownOrchestrator } = require(`${kernelRoot}/volt-unifier/lifecycle/shutdown`);
    const orch = new ShutdownOrchestrator({});
  }));

  return results;
}

async function auditBridges(logger, kernelRoot) {
  const results = [];

  const mockUnifier = {
    eventBus: { publish() {}, subscribe() { return () => {}; } },
    registry: { get() { return null; } },
    audit: { record() {} },
  };

  results.push(await runServiceTest("Unifier Bridges", "DesktopBridge instantiation", () => {
    const { DesktopBridge } = require(`${kernelRoot}/volt-unifier/bridges/desktopBridge`);
    new DesktopBridge(mockUnifier);
  }));

  results.push(await runServiceTest("Unifier Bridges", "FinderBridge instantiation", () => {
    const { FinderBridge } = require(`${kernelRoot}/volt-unifier/bridges/finderBridge`);
    new FinderBridge(mockUnifier);
  }));

  return results;
}

async function auditHealthMonitor(logger, kernelRoot) {
  const results = [];

  const mockUnifier = {
    registry: { getAll() { return new Map(); }, get() { return null; } },
    eventBus: { publish() {}, subscribe() { return () => {}; } },
    audit: { record() {} },
    config: { health: { heartbeatIntervalMs: 5000, failureThreshold: 3 } },
  };

  results.push(await runServiceTest("Unifier Health", "HealthMonitor instantiation", () => {
    const { HealthMonitor } = require(`${kernelRoot}/volt-unifier/health/healthMonitor`);
    new HealthMonitor(mockUnifier);
  }));

  results.push(await runServiceTest("Unifier Health", "ReconnectPolicy", () => {
    const { ReconnectPolicy } = require(`${kernelRoot}/volt-unifier/health/reconnect`);
    const policy = new ReconnectPolicy();
    policy.getDelay(0);
    policy.getDelay(3);
  }));

  return results;
}

async function auditMetrics(logger, kernelRoot) {
  const results = [];

  results.push(await runServiceTest("Unifier Metrics", "MetricsAggregator", () => {
    const { MetricsAggregator } = require(`${kernelRoot}/volt-unifier/metrics/aggregator`);
    new MetricsAggregator({ registry: { getAll() { return new Map(); } } });
  }));

  return results;
}

module.exports = {
  auditUnifierConstants, auditSbpMessage, auditSbpRouter,
  auditModuleRegistry, auditEventBus, auditCapabilityGuard,
  auditLifecycle, auditBridges, auditHealthMonitor, auditMetrics,
  label: "Volt-Unifier",
};
