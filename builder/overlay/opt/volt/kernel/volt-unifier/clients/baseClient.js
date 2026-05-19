'use strict';
const net = require('node:net');
const events = require('node:events');

class BaseModuleClient extends events.EventEmitter {
  constructor(unifier, moduleId, endpointConfig) {
    super();
    this.unifier = unifier;
    this.moduleId = moduleId;
    this.config = endpointConfig;
    this.status = 'offline';
    this.socket = null;
    this.buffer = Buffer.alloc(0);
    this.lastHeartbeatAt = 0;
    this.errorCount = 0;
    this.reconnectCount = 0;
    this.lastError = null;
    this.degradedReason = null;
    this.capabilities = {};
    this._pendingRequests = new Map();
    this._destroyed = false;
  }

  async connect() {
    if (this._destroyed) return;
    if (!this.config.socket) {
      this.status = 'degraded';
      this.degradedReason = 'no socket configured';
      return;
    }
    return new Promise((resolve, reject) => {
      const sock = new net.Socket();
      sock.on('connect', () => {
        this.status = 'online';
        this.socket = sock;
        this.lastHeartbeatAt = Date.now();
        this.reconnectCount = 0;
        this.emit('connected');
        resolve();
      });
      sock.on('data', (data) => {
        this.buffer = Buffer.concat([this.buffer, data]);
        this._processBuffer();
      });
      sock.on('close', () => {
        this.socket = null;
        if (!this._destroyed) {
          this.status = 'offline';
          this.emit('disconnected');
          this._scheduleReconnect();
        }
      });
      sock.on('error', (err) => {
        this.lastError = err.message;
        this.errorCount++;
        if (!this._destroyed && this.status !== 'connecting') {
          this.status = 'error';
          this.emit('error', err);
        }
        reject(err);
      });
      sock.connect(this.config.socket);
      this.status = 'connecting';
    });
  }

  async disconnect() {
    this._destroyed = true;
    if (this.socket) {
      this.socket.destroy();
      this.socket = null;
    }
    this.status = 'offline';
    this._rejectAllPending(new Error('disconnected'));
  }

  async reconnect() {
    this.reconnectCount++;
    this.status = 'recovering';
    this.emit('reconnecting', this.reconnectCount);
    try {
      await this.connect();
      this.emit('reconnected');
      return true;
    } catch (e) {
      this.lastError = e.message;
      return false;
    }
  }

  async send(opcode, payload) {
    if (this.status !== 'online') throw new Error(`Module ${this.moduleId} is ${this.status}`);
    const { SbpMessage } = require('../sbp/message');
    const msg = SbpMessage.create(opcode, 0x01, payload);
    this.socket.write(msg.encode());
  }

  async request(opcode, payload, timeoutMs = 5000) {
    if (this.status !== 'online') throw new Error(`Module ${this.moduleId} is ${this.status}`);
    const { SbpMessage } = require('../sbp/message');
    const requestId = BigInt(Date.now()) << 16n | BigInt(Math.floor(Math.random() * 65536));
    const msg = SbpMessage.create(opcode, 0x01, payload, requestId);
    this.socket.write(msg.encode());
    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this._pendingRequests.delete(requestId);
        reject(new Error(`Request ${requestId} timed out after ${timeoutMs}ms`));
      }, timeoutMs);
      this._pendingRequests.set(requestId.toString(), { resolve, reject, timer });
    });
  }

  markDegraded(reason) {
    this.status = 'degraded';
    this.degradedReason = reason;
    this.emit('degraded', reason);
  }

  markFatal(error) {
    this.status = 'fatal';
    this.lastError = error.message || String(error);
    this.emit('fatal', error);
  }

  isOnline() { return this.status === 'online'; }
  isDegraded() { return this.status === 'degraded' || this.status === 'recovering'; }

  _processBuffer() {
    const { SbpMessage } = require('../sbp/message');
    while (this.buffer.length >= 32) {
      const result = SbpMessage.fromBuffer(this.buffer);
      if (!result) break;
      const { message, consumed } = result;
      this.buffer = this.buffer.slice(consumed);
      this._handleMessage(message);
    }
  }

  _handleMessage(msg) {
    const reqId = msg.header.requestId.toString();
    const pending = this._pendingRequests.get(reqId);
    if (pending) {
      clearTimeout(pending.timer);
      this._pendingRequests.delete(reqId);
      if (msg.header.flags & 0x04) {
        pending.reject(new Error(`SBP error: ${msg.payload.toString()}`));
      } else {
        pending.resolve(msg.payload);
      }
    }
    if (msg.header.flags & 0x08) {
      this.emit('event', msg);
      if (this.unifier && this.unifier.eventBus) {
        const { normalizeSbpEvent } = require('../events/normalizer');
        const voltEvent = normalizeSbpEvent(this.moduleId, msg.header.opcode, msg.payload);
        if (voltEvent) this.unifier.eventBus.publish(voltEvent);
      }
    }
  }

  _scheduleReconnect() {
    const delay = Math.min(500 * Math.pow(2, this.reconnectCount), 30000);
    setTimeout(() => { if (!this._destroyed) this.reconnect(); }, delay);
  }

  _rejectAllPending(error) {
    for (const [id, pending] of this._pendingRequests) {
      clearTimeout(pending.timer);
      pending.reject(error);
    }
    this._pendingRequests.clear();
  }
}

module.exports = { BaseModuleClient };
