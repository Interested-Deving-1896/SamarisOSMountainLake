const { profileMemory, profileMemoryAsync, measureCpuTime } = require("../engine/profiler");
const { MockLogger, MockUserService, MockKernelB, MockEventBus } = require("../engine/mockFactory");

function createResult(service, test, status, notes, data) {
  return { service, test, status, notes, data, error: status === "failed" ? notes : null, toJSON() { return this; } };
}

async function auditProfiling(logger, kernelRoot) {
  const results = [];

  // Memory footprint of key services
  const serviceConstructors = [
    ["Auth", () => new (require(`${kernelRoot}/core/auth`))(new MockLogger())],
    ["EventBus", () => new (require(`${kernelRoot}/core/eventBus`))(new MockLogger())],
    ["PermissionManager", () => new (require(`${kernelRoot}/services/permissionManager`))(new MockLogger())],
    ["VaultService", () => new (require(`${kernelRoot}/services/vaultService`))(new MockLogger(), new MockUserService(), new MockKernelB())],
    ["FileSystemService", () => new (require(`${kernelRoot}/services/fileSystem`))(new MockLogger(), new MockEventBus(), new MockUserService(), new MockKernelB())],
  ];

  for (const [name, factory] of serviceConstructors) {
    try {
      const mem = await profileMemory(factory, name, { iterations: 100 });
      results.push(createResult("Profiling", `${name} instantiation memory`, "passed",
        `${(mem.perCall.heapUsed / 1024).toFixed(1)} KB/op · ${(mem.totalDiff.heapUsed / 1024).toFixed(1)} KB total`,
        mem));
    } catch (err) {
      results.push(createResult("Profiling", `${name} instantiation memory`, "failed", err.message));
    }
  }

  // CPU time for hot-path operations
  const Auth = require(`${kernelRoot}/core/auth`);
  const PM = require(`${kernelRoot}/services/permissionManager`);
  const pm = new PM(new MockLogger());
  pm.seed("app", ["*"]);
  const auth = new Auth(new MockLogger());
  const authKernel = { permissionManager: pm };

  const cpuAuth = measureCpuTime(() => { auth.authorize({ type: "system.ping", appId: "app" }, authKernel); }, 10000);
  results.push(createResult("Profiling", "Auth authorize CPU time", "passed",
    `user: ${(cpuAuth.user / 1000).toFixed(2)} μs · system: ${(cpuAuth.system / 1000).toFixed(2)} μs`, cpuAuth));

  const Vault = require(`${kernelRoot}/services/vaultService`);
  const vault = new Vault(new MockLogger(), new MockUserService(), new MockKernelB());
  const cpuEncrypt = measureCpuTime(() => { vault.encryptString("test", "password"); }, 100);
  results.push(createResult("Profiling", "Vault encrypt CPU time", "passed",
    `user: ${(cpuEncrypt.user / 1000).toFixed(2)} μs · system: ${(cpuEncrypt.system / 1000).toFixed(2)} μs`, cpuEncrypt));

  return results;
}

module.exports = { auditProfiling, label: "CPU & Memory Profiling" };
