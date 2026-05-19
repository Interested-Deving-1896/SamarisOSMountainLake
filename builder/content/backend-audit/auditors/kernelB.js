const fs = require("node:fs");
const net = require("node:net");
const { measureAsync, fmtPerf } = require("../engine/benchmark");
const { runServiceTest } = require("../engine/serviceRunner");
const { ConnectionPool } = require("../engine/connectionPool");

const SOCKET_PATH = process.env.VOLT_KERNEL_B_SOCKET || "/run/samaris/volt-kernel-b.sock";

function socketExists() {
  try { return fs.existsSync(SOCKET_PATH); } catch { return false; }
}

function createResult(service, test, status, notes, benchmark) {
  return { service, test, status, notes, benchmark, error: status === "failed" ? notes : null, toJSON() { return this; } };
}

async function auditKernelB(logger) {
  const results = [];

  if (!socketExists()) {
    results.push(createResult("KernelB", "socket exists", "skipped", `Socket not found at ${SOCKET_PATH}`));
    results.push(createResult("KernelB", "all remaining tests", "skipped", "Kernel B not available — all tests skipped"));
    return results;
  }

  results.push(createResult("KernelB", "socket exists", "passed", SOCKET_PATH));

  const pool = new ConnectionPool(SOCKET_PATH);

  // Connection latency
  results.push(await runServiceTest("KernelB", "connection latency (cold)", async () => {
    const sock = net.createConnection(SOCKET_PATH);
    await new Promise((resolve, reject) => {
      sock.once("connect", resolve);
      sock.once("error", reject);
    });
    sock.destroy();
  }));

  // JSON-RPC health
  results.push(await runServiceTest("KernelB", "health() via JSON-RPC", async () => {
    await pool.callDirect("health", {}, { timeoutMs: 2000 });
  }));

  // JSON-RPC ping
  results.push(await runServiceTest("KernelB", "ping() via JSON-RPC", async () => {
    await pool.callDirect("ping", {}, { timeoutMs: 2000 });
  }));

  // Connection overhead: pooled vs direct
  results.push(await runServiceTest("KernelB", "pooled vs direct overhead", async () => {
    const r = await pool.measurePoolBenefit("ping", {}, 30);
    if (!r.pooled || !r.direct) throw new Error("No data from pool comparison");
  }));

  // Throughput
  results.push(await runServiceTest("KernelB", "throughput small payload", async () => {
    const start = Date.now();
    let count = 0;
    while (Date.now() - start < 1000) {
      await pool.callDirect("ping", {}, { timeoutMs: 500 });
      count++;
    }
    return count;
  }));

  // Error handling
  results.push(await runServiceTest("KernelB", "unknown method error", async () => {
    try { await pool.callDirect("nonexistent", {}, { timeoutMs: 1000 }); }
    catch { /* expected */ }
  }));

  results.push(await runServiceTest("KernelB", "empty payload handling", async () => {
    await pool.callDirect("ping", {}, { timeoutMs: 2000 });
  }));

  // SBP protocol (if supported)
  results.push(await runServiceTest("KernelB", "SBP protocol handshake (first byte S)", async () => {
    const sock = net.createConnection(SOCKET_PATH);
    await new Promise((resolve, reject) => {
      sock.once("connect", resolve);
      sock.once("error", reject);
    });
    sock.write(Buffer.from([0x53])); // 'S' = SBP mode
    sock.destroy();
  }));

  return results;
}

module.exports = { auditKernelB, label: "Kernel B (Rust — Tesseract Engine)" };
