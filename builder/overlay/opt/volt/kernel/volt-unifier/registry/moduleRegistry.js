'use strict';

const { MODULE_IDS, MODULE_STATUSES, MODULE_PROTOCOLS } = require('../constants');

class ModuleRegistry {
  constructor(unifier) {
    this._unifier = unifier;
    this._modules = new Map();
  }

  register(moduleId, config) {
    if (!moduleId || typeof moduleId !== 'string') {
      throw new Error('moduleId must be a non-empty string');
    }

    if (this._modules.has(moduleId)) {
      throw new Error(`Module ${moduleId} is already registered`);
    }

    const entry = {
      id: moduleId,
      config: config || {},
      status: 'offline',
      health: {
        status: 'offline',
        lastHeartbeatAt: null,
        latencyMs: null,
        capabilities: [],
        errorCount: 0,
        reconnectCount: 0,
        lastError: null,
        degradedReason: null,
      },
      capabilities: null,
      client: null,
      connectedAt: null,
    };

    this._modules.set(moduleId, entry);
    return entry;
  }

  get(moduleId) {
    if (!this._modules.has(moduleId)) {
      return null;
    }
    return this._modules.get(moduleId);
  }

  has(moduleId) {
    return this._modules.has(moduleId);
  }

  getAll() {
    return Array.from(this._modules.values());
  }

  getOnline() {
    return this.getByStatus('online');
  }

  getByStatus(status) {
    if (!MODULE_STATUSES.includes(status)) {
      return [];
    }
    return Array.from(this._modules.values())
      .filter(entry => entry.status === status);
  }

  updateStatus(moduleId, status, extra) {
    const entry = this._modules.get(moduleId);
    if (!entry) {
      return false;
    }

    if (!MODULE_STATUSES.includes(status)) {
      throw new Error(`Invalid module status: ${status}`);
    }

    entry.status = status;
    entry.health.status = status;

    if (status === 'online') {
      entry.connectedAt = entry.connectedAt || Date.now();
    }

    if (extra && typeof extra === 'object') {
      if (extra.lastError !== undefined) {
        entry.health.lastError = extra.lastError;
      }
      if (extra.degradedReason !== undefined) {
        entry.health.degradedReason = extra.degradedReason;
        if (status !== 'degraded' && extra.degradedReason) {
          entry.status = 'degraded';
          entry.health.status = 'degraded';
        }
      }
      if (extra.latencyMs !== undefined) {
        entry.health.latencyMs = extra.latencyMs;
      }
      if (extra.lastHeartbeatAt !== undefined) {
        entry.health.lastHeartbeatAt = extra.lastHeartbeatAt;
      }
      if (extra.errorCount !== undefined) {
        entry.health.errorCount = extra.errorCount;
      }
      if (extra.reconnectCount !== undefined) {
        entry.health.reconnectCount = extra.reconnectCount;
      }
      if (extra.capabilities !== undefined) {
        entry.capabilities = extra.capabilities;
        entry.health.capabilities = extra.capabilities;
      }
      if (extra.client !== undefined) {
        entry.client = extra.client;
      }
    }

    return true;
  }

  updateHealth(moduleId, health) {
    const entry = this._modules.get(moduleId);
    if (!entry) {
      return false;
    }

    if (!health || typeof health !== 'object') {
      return false;
    }

    const validKeys = [
      'status', 'lastHeartbeatAt', 'latencyMs', 'capabilities',
      'errorCount', 'reconnectCount', 'lastError', 'degradedReason',
    ];

    for (const key of validKeys) {
      if (health[key] !== undefined) {
        if (key === 'status' && !MODULE_STATUSES.includes(health[key])) {
          continue;
        }
        entry.health[key] = health[key];
        if (key === 'status') {
          entry.status = health[key];
        }
      }
    }

    if (health.lastHeartbeatAt) {
      entry.health.lastHeartbeatAt = health.lastHeartbeatAt;
    }

    return true;
  }

  remove(moduleId) {
    return this._modules.delete(moduleId);
  }

  count() {
    return this._modules.size;
  }
}

module.exports = { ModuleRegistry };
