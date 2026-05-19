'use strict';

const { execFile } = require('node:child_process');
const { promisify } = require('node:util');
const EventEmitter = require('node:events');

const execFileAsync = promisify(execFile);

const DEFAULT_SERVICES = Object.freeze({
  vrm: 'volt-ram-manager.service',
  vum: 'volt-usb-manager.service',
  vgm: 'volt-gpu-manager.service',
  dwp: 'volt-worker-pool.service',
});

class ServiceHealthClient extends EventEmitter {
  constructor(unifier, moduleId, endpointConfig = {}) {
    super();
    this.unifier = unifier;
    this.moduleId = moduleId;
    this.config = endpointConfig;
    this.service = endpointConfig.service || DEFAULT_SERVICES[moduleId] || `${moduleId}.service`;
    this.status = 'offline';
    this.lastHeartbeatAt = null;
    this.errorCount = 0;
    this.reconnectCount = 0;
    this.lastError = null;
    this.degradedReason = null;
  }

  async connect() {
    await this.refresh();
  }

  async disconnect() {
    this.status = 'offline';
  }

  isOnline() {
    return this.status === 'online';
  }

  async refresh() {
    try {
      const { stdout } = await execFileAsync('systemctl', ['is-active', this.service], { timeout: 2000 });
      const state = stdout.trim();
      if (state === 'active') {
        this.status = 'online';
        this.lastHeartbeatAt = Date.now();
        this.lastError = null;
        this.degradedReason = null;
        return;
      }
      this.status = 'offline';
      this.degradedReason = `systemd state: ${state || 'unknown'}`;
    } catch (err) {
      const state = String(err.stdout || '').trim();
      this.status = 'offline';
      this.lastError = err.message;
      this.degradedReason = state ? `systemd state: ${state}` : err.message;
      this.errorCount++;
    }
  }

  async request(opcode, _payload, _timeoutMs = 2000) {
    await this.refresh();
    if (!this.isOnline()) {
      throw new Error(`${this.moduleId} is ${this.status}: ${this.degradedReason || this.lastError || 'not active'}`);
    }
    return Buffer.from(JSON.stringify({
      ok: true,
      moduleId: this.moduleId,
      service: this.service,
      status: this.status,
      opcode,
      timestamp: Date.now(),
    }));
  }
}

module.exports = { ServiceHealthClient };
