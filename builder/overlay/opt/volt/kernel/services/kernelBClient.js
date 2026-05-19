const fs = require("node:fs");
const net = require("node:net");

const DEFAULT_SOCKET = "/run/samaris/volt-kernel-b.sock";
const POOL_SIZE = Math.max(1, parseInt(process.env.VOLT_KERNEL_B_POOL_SIZE || "4"));
const MAX_PENDING = 256;
const CB_THRESHOLD = 3;
const CB_RESET_MS = 10000;

const OPS = {
  GPU_RENDER: 0x01, GPU_COMPUTE: 0x02, CPU_RESERVE: 0x03, CPU_RELEASE: 0x04,
  CPU_EXEC: 0x05, MEM_ALLOC: 0x06, MEM_FREE: 0x07, STREAM_VIDEO: 0x08,
  STREAM_AUDIO: 0x09, QUERY_CORES: 0x0A, QUERY_GPU: 0x0B, HEARTBEAT: 0x0C,
  THERMAL_STATUS: 0x0F, CONTEXT_CREATE: 0x30, CONTEXT_SHARE: 0x31,
};

function sbpEncode(opcode, payloadStr) {
  const payloadBuf = Buffer.from(payloadStr, "utf8");
  const h = Buffer.alloc(16);
  h.writeUInt16LE(0x4F56, 0);
  h[2] = 0x05;
  h[3] = opcode;
  h.writeUInt16LE(1, 4);
  h[6] = 2;
  h.writeUInt32LE(1, 7);
  h.writeUInt32LE(payloadBuf.length, 11);
  let cs = 0;
  for (let i = 0; i < 15; i++) cs ^= h[i];
  h[15] = cs;
  return Buffer.concat([h, payloadBuf]);
}

function sbpEncodeQuery(kind) {
  const payload = JSON.stringify({ kind });
  return sbpEncode(kind === "cores" ? OPS.QUERY_CORES : kind === "gpu" ? OPS.QUERY_GPU : OPS.THERMAL_STATUS, payload);
}

class PooledConnection {
  constructor(client, index) {
    this.client = client;
    this.index = index;
    this.socket = null;
    this.buffer = "";
    this.sbpBuffer = Buffer.alloc(0);
    this.alive = false;
  }

  async connect() {
    if (this.socket && this.alive) return;
    if (this.socket) { try { this.socket.destroy(); } catch {} this.socket = null; }
    return new Promise((resolve) => {
      const sock = net.createConnection(this.client.socketPath);
      sock.once("connect", () => {
        this.socket = sock;
        this.alive = true;
        this.buffer = "";
        this.sbpBuffer = Buffer.alloc(0);
        sock.write(this.client._sbpEnabled ? "S" : "J");
        sock.on("data", (chunk) => { this.client._onData(this, chunk); });
        sock.on("error", (err) => { this.client._onError(this, err); });
        sock.on("close", () => { this.client._onClose(this); });
        resolve();
      });
      sock.once("error", () => resolve());
      setTimeout(() => { if (!this.alive) { sock.destroy(); resolve(); } }, 2000);
    });
  }

  send(data) {
    if (this.socket && this.alive) { this.socket.write(data); return true; }
    return false;
  }

  close() {
    if (this.socket) { try { this.socket.destroy(); } catch {} this.socket = null; }
    this.alive = false;
  }
}

class KernelBClient {
  constructor(logger, socketPath = process.env.VOLT_KERNEL_B_SOCKET || DEFAULT_SOCKET) {
    this.logger = logger;
    this.socketPath = socketPath;
    this.requestSeq = 0;
    this._pending = new Map();
    this._destroyed = false;
    this._sbpEnabled = false;
    this._sbpCapable = null;
    this._poolIndex = 0;
    this._circuitFailures = 0;
    this._circuitOpen = false;
    this._cbResetTimer = null;
    this._pool = [];
    for (let i = 0; i < POOL_SIZE; i++) this._pool.push(new PooledConnection(this, i));
  }

  available() {
    if (this._circuitOpen) return false;
    try { return fs.existsSync(this.socketPath); } catch { return false; }
  }

  async probeSbp() {
    if (this._sbpCapable !== null) return this._sbpCapable;
    return new Promise((resolve) => {
      const sock = net.createConnection(this.socketPath);
      const timer = setTimeout(() => { sock.destroy(); resolve(false); }, 500);
      sock.once("connect", () => {
        sock.write("S");
        sock.write(sbpEncodeQuery("cores"));
        let buf = Buffer.alloc(0);
        sock.on("data", (chunk) => {
          buf = Buffer.concat([buf, chunk]);
          const r = this._decodeSbpFrame(buf);
          if (r) { clearTimeout(timer); sock.destroy(); this._sbpCapable = true; resolve(true); }
        });
        setTimeout(() => { sock.destroy(); if (!this._sbpCapable) { this._sbpCapable = false; resolve(false); } }, 500);
      });
      sock.once("error", () => { clearTimeout(timer); resolve(false); });
    });
  }

  _decodeSbpFrame(buf) {
    if (buf.length < 16) return null;
    const magic = buf.readUInt16LE(0);
    if (magic !== 0x4F56) return null;
    if (buf[2] !== 0x05) return null;
    const payloadLen = buf.readUInt32LE(11);
    let cs = 0;
    for (let i = 0; i < 15; i++) cs ^= buf[i];
    if (buf[15] !== cs) return null;
    const total = 16 + payloadLen;
    if (buf.length < total) return null;
    return { opcode: buf[3], payload: buf.slice(16, total).toString("utf8") };
  }

  async _ensurePool() {
    if (this._destroyed) return;
    const connectPromises = this._pool.map((c) => c.alive ? Promise.resolve() : c.connect());
    await Promise.all(connectPromises);
    if (this._sbpCapable === null && this._pool.some((c) => c.alive)) {
      try { const supported = await this.probeSbp();
        if (supported && !this._sbpEnabled) {
          this._sbpEnabled = true;
          await Promise.all(this._pool.map((c) => { c.close(); return c.connect(); }));
        }
      } catch {}
    }
    if (this._pool.some((c) => c.alive)) this._circuitSuccess();
  }

  call(method, params = {}, options = {}) {
    const timeoutMs = options.timeoutMs || 5000;
    const id = `node-${Date.now()}-${++this.requestSeq}`;
    return new Promise((resolve, reject) => {
      if (this._circuitOpen) { reject(new Error("kernel_b_circuit_open")); return; }
      if (this._pending.size >= MAX_PENDING) { reject(new Error("kernel_b_overloaded")); return; }
      const timer = setTimeout(() => {
        this._pending.delete(id); this._circuitFail(); reject(new Error("kernel_b_timeout"));
      }, timeoutMs);
      this._pending.set(id, { resolve, reject, timer });
      this._ensurePool().then(() => {
        const conn = this._nextConnection();
        if (!conn || !conn.send(this._encode(id, method, params))) {
          clearTimeout(timer); this._pending.delete(id); this._circuitFail();
          reject(new Error("kernel_b_no_connection"));
        }
      }).catch((err) => { clearTimeout(timer); this._pending.delete(id); this._circuitFail(); reject(err); });
    });
  }

  async health() { return this.call("health", {}, { timeoutMs: 500 }); }
  async queryCores() { return this.call("query_cores", {}); }
  async queryGpu() { return this.call("query_gpu", {}); }
  async thermalStatus() { return this.call("thermal_status", {}); }
  async sysInfo() { return this.call("sys_info", {}); }
  async metrics() { return this.call("metrics", {}); }
  async safetyStatus() { return this.call("safety_status", {}); }
  async systemStatus() { return this.call("system_status", {}); }
  async auditLog() { return this.call("audit_log", {}); }

  _encode(id, method, params) {
    const json = JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n";
    if (!this._sbpEnabled) return json;
    const opcodeMap = {
      health: 0x0C, ping: 0x0C, query_cores: 0x0A, query_gpu: 0x0B,
      thermal_status: 0x0F, sys_info: 0x0A,
    };
    const opcode = opcodeMap[method] || 0x0A;
    return sbpEncode(opcode, json);
  }

  _nextConnection() {
    const alive = this._pool.filter((c) => c.alive && !c.busy);
    if (alive.length === 0) {
      const any = this._pool.find((c) => c.alive);
      return any || null;
    }
    this._poolIndex = (this._poolIndex + 1) % alive.length;
    return alive[this._poolIndex];
  }

  _onData(conn, chunk) {
    if (this._sbpEnabled) {
      conn.sbpBuffer = Buffer.concat([conn.sbpBuffer, chunk]);
      while (true) {
        const decoded = this._decodeSbpFrame(conn.sbpBuffer);
        if (!decoded) break;
        conn.sbpBuffer = conn.sbpBuffer.slice(16 + decoded.payload.length);
        try { const resp = JSON.parse(decoded.payload); this._resolvePending(resp); }
        catch { try { const parsed = JSON.parse(decoded.payload); this._resolvePending(parsed); } catch {} }
      }
    } else {
      conn.buffer += chunk.toString("utf8");
      let idx;
      while ((idx = conn.buffer.indexOf("\n")) >= 0) {
        const line = conn.buffer.slice(0, idx).trim();
        conn.buffer = conn.buffer.slice(idx + 1);
        if (!line) continue;
        try { const resp = JSON.parse(line); this._resolvePending(resp); } catch {}
      }
    }
  }

  _resolvePending(response) {
    const pending = this._pending.get(response.id);
    if (!pending) return;
    clearTimeout(pending.timer);
    this._pending.delete(response.id);
    this._circuitSuccess();
    if (response.error) {
      const err = new Error(response.error.message || "kernel_b_error");
      err.code = response.error.code || "KERNEL_B_ERROR";
      pending.reject(err);
    } else {
      pending.resolve(response.result);
    }
  }

  _onError(conn) { conn.alive = false; this._circuitFail(); }

  _onClose(conn) {
    conn.alive = false;
    setTimeout(() => { if (!this._destroyed) conn.connect().catch(() => {}); }, 2000);
  }

  _circuitFail() {
    this._circuitFailures++;
    if (this._circuitFailures >= CB_THRESHOLD) {
      this._circuitOpen = true;
      this._failAllPending(new Error("kernel_b_circuit_open"));
      if (this._cbResetTimer) clearTimeout(this._cbResetTimer);
      this._cbResetTimer = setTimeout(() => { this._circuitOpen = false; this._circuitFailures = 0; }, CB_RESET_MS);
    }
  }

  _circuitSuccess() { this._circuitFailures = 0; this._circuitOpen = false; }

  _failAllPending(error) {
    for (const [, p] of this._pending) { clearTimeout(p.timer); p.reject(error); }
    this._pending.clear();
  }

  destroy() {
    this._destroyed = true;
    if (this._cbResetTimer) clearTimeout(this._cbResetTimer);
    this._failAllPending(new Error("kernel_b_destroyed"));
    for (const conn of this._pool) conn.close();
  }
}

module.exports = KernelBClient;
