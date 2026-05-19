'use strict';
const { ServiceBridge } = require('./serviceBridge');
const { EVENT_TYPES } = require('../constants');

class DevToolsBridge extends ServiceBridge {
  constructor(unifier) {
    super(unifier);
    this._events = [];
    this._maxEvents = 1000;
  }

  getDashboardSnapshot() {
    const allEntries = this.unifier.registry.getAll();
    const metrics = {};

    for (const entry of allEntries) {
      if (entry.client && typeof entry.client.metricsSnapshot === 'function') {
        metrics[entry.id] = { available: true };
      } else {
        metrics[entry.id] = { available: false, status: entry.status };
      }
    }

    const snapshot = {
      timestamp: Date.now(),
      modules: metrics,
      overallStatus: this._getOverallStatus(),
      eventCount: this._events.length,
    };

    return snapshot;
  }

  getEventHistory(limit) {
    const take = typeof limit === 'number' && limit > 0 ? limit : 50;
    return this._events.slice(-take);
  }

  getHealthOverview() {
    const modules = this.unifier.registry.getAll();
    const overview = {};

    for (const entry of modules) {
      overview[entry.id] = {
        status: entry.health.status,
        lastHeartbeatAt: entry.health.lastHeartbeatAt,
        latencyMs: entry.health.latencyMs,
        errorCount: entry.health.errorCount,
        reconnectCount: entry.health.reconnectCount,
        lastError: entry.health.lastError,
        degradedReason: entry.health.degradedReason,
      };
    }

    return overview;
  }

  start() {
    const types = Object.values(EVENT_TYPES);
    for (const type of types) {
      this._subscribe(type, (event) => {
        this._events.push(event);
        if (this._events.length > this._maxEvents) {
          this._events.shift();
        }
      });
    }
  }

  destroy() {
    super.destroy();
  }

  _getOverallStatus() {
    const modules = this.unifier.registry.getAll();
    let hasFatal = false;
    let hasError = false;
    let hasDegraded = false;

    for (const entry of modules) {
      if (entry.status === 'fatal') hasFatal = true;
      else if (entry.status === 'error') hasError = true;
      else if (entry.status === 'degraded') hasDegraded = true;
    }

    if (hasFatal) return 'fatal';
    if (hasError) return 'error';
    if (hasDegraded) return 'degraded';
    return 'online';
  }
}

module.exports = { DevToolsBridge };
