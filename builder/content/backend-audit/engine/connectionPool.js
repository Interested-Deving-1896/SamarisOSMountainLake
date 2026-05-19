const net = require("node:net");

class ConnectionPool {
  constructor(path, opts = {}) {
    this.path = path;
    this.maxSize = opts.maxSize || 10;
    this.timeout = opts.timeout || 5000;
    this.pool = [];
    this.active = new Set();
    this.requestId = 0;
    this._pending = new Map();
    this._buffer = "";
    this._socket = null;
  }

  async borrow() {
    if (this.pool.length > 0) {
      const conn = this.pool.pop();
      this.active.add(conn);
      return conn;
    }
    const conn = await this._connect();
    this.active.add(conn);
    return conn;
  }

  release(conn) {
    this.active.delete(conn);
    if (this.pool.length < this.maxSize) {
      this.pool.push(conn);
    } else {
      conn.socket.destroy();
    }
  }

  _connect() {
    return new Promise((resolve, reject) => {
      const socket = net.createConnection(this.path);
      const timer = setTimeout(() => {
        socket.destroy();
        reject(new Error("connection_timeout"));
      }, this.timeout);

      socket.once("connect", () => {
        clearTimeout(timer);
        socket.write("J");
        resolve({ socket, id: Date.now() });
      });
      socket.once("error", (err) => { clearTimeout(timer); reject(err); });
    });
  }

  async call(method, params = {}, opts = {}) {
    const conn = await this.borrow();
    const id = ++this.requestId;
    const payload = JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n";

    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this._pending.delete(id);
        this.release(conn);
        reject(new Error("timeout"));
      }, opts.timeoutMs || 15000);

      this._pending.set(id, { resolve, reject, timer, conn });

      conn.socket.write(payload);
    });
  }

  // Per-request socket (no pooling)
  async callDirect(method, params = {}, opts = {}) {
    const id = ++this.requestId;
    const payload = JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n";

    return new Promise((resolve, reject) => {
      const socket = net.createConnection(this.path);
      let buffer = "";
      let settled = false;

      const finish = (err, val) => {
        if (settled) return;
        settled = true;
        clearTimeout(timer);
        socket.destroy();
        if (err) reject(err);
        else resolve(val);
      };

      const timer = setTimeout(() => finish(new Error("timeout"), null), opts.timeoutMs || 15000);

      socket.on("connect", () => {
        socket.write("J");
        socket.write(payload);
      });

      socket.on("data", (chunk) => {
        buffer += chunk.toString("utf8");
        let idx;
        while ((idx = buffer.indexOf("\n")) >= 0) {
          const line = buffer.slice(0, idx).trim();
          buffer = buffer.slice(idx + 1);
          if (!line) continue;
          try {
            const resp = JSON.parse(line);
            if (resp.id !== id) continue;
            if (resp.error) finish(new Error(resp.error.message || "rpc_error"), null);
            else finish(null, resp.result);
          } catch (e) { finish(e, null); }
        }
      });
      socket.on("error", (err) => finish(err, null));
      socket.on("close", () => finish(new Error("closed"), null));
    });
  }

  async measurePoolBenefit(method, params, iterations = 50) {
    // Warmup
    for (let i = 0; i < 5; i++) {
      try { await this.call(method, params, { timeoutMs: 2000 }); } catch {}
    }

    const pooledSamples = [];
    for (let i = 0; i < iterations; i++) {
      const start = Number(process.hrtime.bigint());
      try { await this.call(method, params, { timeoutMs: 2000 }); } catch {}
      pooledSamples.push(Number(process.hrtime.bigint()) - start);
    }

    const directSamples = [];
    for (let i = 0; i < iterations; i++) {
      const start = Number(process.hrtime.bigint());
      try { await this.callDirect(method, params, { timeoutMs: 2000 }); } catch {}
      directSamples.push(Number(process.hrtime.bigint()) - start);
    }

    const { computeStats } = require("./benchmark");
    return { pooled: computeStats(pooledSamples), direct: computeStats(directSamples) };
  }
}

module.exports = { ConnectionPool };
