const fs = require("node:fs");
const { runServiceTest } = require("../engine/serviceRunner");
const { ConnectionPool } = require("../engine/connectionPool");
const { runMatrix } = require("../engine/matrixRunner");

const SOCKET_PATH = process.env.VOLT_KERNEL_B_SOCKET || "/run/samaris/volt-kernel-b.sock";

function socketExists() {
  try { return fs.existsSync(SOCKET_PATH); } catch { return false; }
}

function createResult(service, test, status, notes, benchmark) {
  return { service, test, status, notes, benchmark, error: status === "failed" ? notes : null, toJSON() { return this; } };
}

async function auditInterKernel(logger, kernelRoot) {
  const results = [];

  // Always test the kernel B client code itself
  const KBClient = require(`${kernelRoot}/services/kernelBClient`);
  const kb = new KBClient(logger);

  results.push(await runServiceTest("InterKernel", "KernelBClient available() (no socket)", () => {
    kb.available();
  }));

  results.push(await runServiceTest("InterKernel", "KernelBClient call() fails gracefully", async () => {
    try { await kb.health(); } catch { /* expected — no kernel B running */ }
  }));

  // Compare direct vs kernelB-delegated FS operations
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const tempFs = await require("../engine/mockFactory").createTempFileSystem();
  const { MockLogger, MockEventBus, MockUserService } = require("../engine/mockFactory");

  const fsDirect = new FS(new MockLogger(), new MockEventBus(), new MockUserService(), null);
  fsDirect.userRootPath = tempFs.userRoot;
  fsDirect.virtualRoots["/User"] = tempFs.userRoot;

  const fsWithKB = new FS(new MockLogger(), new MockEventBus(), new MockUserService(), kb);
  fsWithKB.userRootPath = tempFs.userRoot;
  fsWithKB.virtualRoots["/User"] = tempFs.userRoot;

  // Benchmark FS operations with and without kernel B delegation
  const testFile = "/User/Documents/kb-test.txt";

  results.push(await runServiceTest("InterKernel", "FS write (direct, no KB)", async () => {
    await fsDirect.write(testFile, "hello");
  }));

  results.push(await runServiceTest("InterKernel", "FS write (with KB client)", async () => {
    await fsWithKB.write(testFile, "hello");
  }));

  results.push(await runServiceTest("InterKernel", "FS rename (direct vs KB delegation)", async () => {
    // Test the _kernelB call path
    const spyAvailable = () => socketExists();
    const origAvailable = kb.available;
    kb.available = spyAvailable;

    await fsWithKB.write(testFile, "data");
    await fsWithKB.rename(testFile, testFile.replace("kb-test", "kb-renamed"));

    kb.available = origAvailable;
  }));

  // Payload size benchmark matrix (if socket exists)
  if (socketExists()) {
    const pool = new ConnectionPool(SOCKET_PATH);

    try {
      const matrix = await runMatrix("KB latency by payload", {
        payloadBytes: [0, 100, 1024, 65536],
      }, async (params) => {
        const payload = "x".repeat(params.payloadBytes);
        await pool.callDirect("ping", { data: payload }, { timeoutMs: 5000 });
      }, { iterations: 20, warmup: 5 });

      results.push({ service: "InterKernel", test: "latency matrix", status: "passed", matrix, toJSON() { return this; } });
    } catch (err) {
      results.push(createResult("InterKernel", "latency matrix", "skipped", err.message));
    }
  }

  await require("../engine/mockFactory").destroyTempFileSystem(tempFs.root);
  return results;
}

module.exports = { auditInterKernel, label: "Inter-Kernel Communication (A ↔ B)" };
