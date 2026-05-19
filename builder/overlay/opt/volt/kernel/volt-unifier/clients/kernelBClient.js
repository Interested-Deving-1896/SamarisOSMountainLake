'use strict';
const fs = require('node:fs');
const net = require('node:net');
const EventEmitter = require('node:events');

class KernelBClient extends EventEmitter {
  constructor(unifier) {
    super();
    this.unifier = unifier;
    this.moduleId = 'kernel-b';
    this.config = unifier.config.modules['kernel-b'];
    this.socketPath = this.config.socket || '/run/samaris/volt-kernel-b.sock';
    this.status = 'offline';
    this.socket = null;
    this.buffer = '';
    this.lastHeartbeatAt = null;
    this.errorCount = 0;
    this.reconnectCount = 0;
    this.lastError = null;
    this.degradedReason = null;
    this._pending = new Map();
    this._requestSeq = 0;
    this._destroyed = false;
  }

  async connect() {
    if (this._destroyed) return;
    if (!fs.existsSync(this.socketPath)) {
      this.status = 'offline';
      this.lastError = `socket missing: ${this.socketPath}`;
      throw new Error(this.lastError);
    }

    return new Promise((resolve, reject) => {
      const sock = net.createConnection(this.socketPath);
      let settled = false;

      const fail = (err) => {
        this.lastError = err.message;
        this.errorCount++;
        this.status = 'error';
        if (!settled) {
          settled = true;
          reject(err);
        }
      };

      sock.on('connect', () => {
        sock.write('J');
        this.socket = sock;
        this.status = 'online';
        this.lastHeartbeatAt = Date.now();
        this.lastError = null;
        this.degradedReason = null;
        if (!settled) {
          settled = true;
          resolve();
        }
      });

      sock.on('data', (chunk) => this._handleData(chunk));
      sock.on('error', fail);
      sock.on('close', () => {
        this.socket = null;
        if (!this._destroyed) {
          this.status = 'offline';
          this._rejectAll(new Error('kernel-b socket closed'));
        }
      });
    });
  }

  async disconnect() {
    this._destroyed = true;
    if (this.socket) {
      this.socket.destroy();
      this.socket = null;
    }
    this.status = 'offline';
    this._rejectAll(new Error('kernel-b disconnected'));
  }

  isOnline() {
    return this.status === 'online' && !!this.socket;
  }

  async request(_opcode, _payload, timeoutMs = 2000) {
    return Buffer.from(JSON.stringify(await this.call('health', {}, timeoutMs)));
  }

  async call(method, params = {}, timeoutMs = 2000) {
    if (!this.isOnline()) {
      await this.connect();
    }
    const id = `unifier-${Date.now()}-${++this._requestSeq}`;
    const request = JSON.stringify({ jsonrpc: '2.0', id, method, params }) + '\n';
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this._pending.delete(id);
        reject(new Error(`kernel-b ${method} timed out after ${timeoutMs}ms`));
      }, timeoutMs);
      this._pending.set(id, { resolve, reject, timer });
      this.socket.write(request);
    });
  }

  async queryCores() { return this.call('query_cores', {}); }
  async queryGpu() { return this.call('query_gpu', {}); }
  async heartbeat() { return this.call('heartbeat', {}); }
  async thermalStatus() { return this.call('thermal_status', {}); }
  async metrics() { return this.call('metrics', {}); }
  async safetyStatus() { return this.call('safety_status', {}); }
  async systemStatus() { return this.call('system_status', {}); }
  async auditLog() { return this.call('audit_log', {}); }
  async sysInfo() { return this.call('sys_info', {}); }
  async execCpu(payload) { return this.call('cpu_exec', { data: payload.toString('base64') }); }
  async renderGpu(payload) { return this.call('gpu_render', { data: payload.toString('base64') }); }

  _handleData(chunk) {
    this.buffer += chunk.toString('utf8');
    let index;
    while ((index = this.buffer.indexOf('\n')) >= 0) {
      const line = this.buffer.slice(0, index).trim();
      this.buffer = this.buffer.slice(index + 1);
      if (!line) continue;
      let response;
      try {
        response = JSON.parse(line);
      } catch (err) {
        this.lastError = err.message;
        continue;
      }
      const pending = this._pending.get(response.id);
      if (!pending) continue;
      clearTimeout(pending.timer);
      this._pending.delete(response.id);
      if (response.error) {
        const err = new Error(response.error.message || 'kernel-b error');
        err.code = response.error.code || 'KERNEL_B_ERROR';
        pending.reject(err);
      } else {
        this.lastHeartbeatAt = Date.now();
        pending.resolve(response.result);
      }
    }
  }

  _rejectAll(err) {
    for (const pending of this._pending.values()) {
      clearTimeout(pending.timer);
      pending.reject(err);
    }
    this._pending.clear();
  }
}

module.exports = { KernelBClient };
