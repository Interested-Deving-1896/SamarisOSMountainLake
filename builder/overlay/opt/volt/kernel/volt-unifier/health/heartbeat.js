'use strict';
const { OPOCODES, HEARTBEAT_DEFAULTS } = require('../constants');

const HEARTBEAT_OPCODES = {
  'kernel-b': OPOCODES.HEARTBEAT,
  vrm: OPOCODES.RAM_HEARTBEAT,
  vum: OPOCODES.USB_HEARTBEAT,
};

class HeartbeatManager {
  constructor(unifier) {
    this.unifier = unifier;
    this._timers = new Map();
  }

  start() {
    for (const [moduleId, config] of Object.entries(this.unifier.config.modules)) {
      if (config.enabled === false) continue;
      const heartbeatMs = config.heartbeatMs || HEARTBEAT_DEFAULTS[moduleId] || 5000;
      const timer = setInterval(() => this._sendHeartbeat(moduleId), heartbeatMs);
      this._timers.set(moduleId, timer);
    }
  }

  stop() {
    for (const [moduleId, timer] of this._timers) {
      clearInterval(timer);
    }
    this._timers.clear();
  }

  async _sendHeartbeat(moduleId) {
    const entry = this.unifier.registry.get(moduleId);
    if (!entry || !entry.client) return;

    const opcode = HEARTBEAT_OPCODES[moduleId];

    try {
      if (entry.client.refresh) {
        await entry.client.refresh();
      }
      if (opcode && entry.client.isOnline && entry.client.isOnline()) {
        await entry.client.request(opcode, Buffer.alloc(0), 2000);
      }
      const now = Date.now();
      this.unifier.registry.updateStatus(moduleId, entry.client.status || 'online', {
        lastHeartbeatAt: now,
        lastError: entry.client.lastError,
        degradedReason: entry.client.degradedReason,
        errorCount: entry.client.errorCount,
        reconnectCount: entry.client.reconnectCount,
      });
      if (entry.client) {
        entry.client.lastHeartbeatAt = now;
      }
    } catch (err) {
      if (entry.client) {
        const now = Date.now();
        entry.client.lastHeartbeatAt = now;
        this.unifier.registry.updateStatus(moduleId, entry.client.status || 'degraded', {
          lastHeartbeatAt: now,
          lastError: err.message,
          degradedReason: entry.client.degradedReason || err.message,
          errorCount: (entry.client.errorCount || entry.health.errorCount || 0) + 1,
          reconnectCount: entry.client.reconnectCount,
        });
      }
    }
  }
}

module.exports = { HeartbeatManager };
