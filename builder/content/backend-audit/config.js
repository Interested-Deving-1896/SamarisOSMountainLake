const KERNEL_ROOT = require("node:path").resolve(__dirname, "..", "volt-kernel-a");

module.exports = {
  kernelRoot: KERNEL_ROOT,

  defaultIterations: 100,
  defaultWarmup: 10,

  slowOpsIterations: 50,
  slowOpsWarmup: 5,
  fastOpsIterations: 500,
  fastOpsWarmup: 50,

  timeoutPerTest: 30000,
  concurrentChecks: 1000,
  fsTestSizes: { small: 1024, medium: 1024 * 1024, large: 10 * 1024 * 1024 },
  mockTempRoot: require("node:path").join(require("node:os").tmpdir(), "samaris-backend-audit"),

  enableOptionalServices: true,
  skipExternalBinaryServices: true,
};
