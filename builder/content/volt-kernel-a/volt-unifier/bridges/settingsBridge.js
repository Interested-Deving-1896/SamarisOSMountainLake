'use strict';
const { ServiceBridge } = require('./serviceBridge');
const { EVENT_TYPES } = require('../constants');

class SettingsBridge extends ServiceBridge {
  constructor(unifier) {
    super(unifier);
    this._moduleStatuses = {};
  }

  getModuleStatuses() {
    const statuses = {};
    for (const entry of this.unifier.registry.getAll()) {
      const cfg = this.unifier.config.modules[entry.id];
      statuses[entry.id] = {
        status: entry.status,
        enabled: cfg ? cfg.enabled !== false : true,
        degradedReason: entry.health.degradedReason,
        lastError: entry.health.lastError,
        errorCount: entry.health.errorCount,
        reconnectCount: entry.health.reconnectCount,
        lastHeartbeatAt: entry.health.lastHeartbeatAt,
      };
    }
    return statuses;
  }

  async getAscExplain() {
    const asc = this.unifier.registry.get('asc');
    if (!asc || !asc.client) throw new Error('ASC client not available');
    return asc.client.explain();
  }

  async getDwpMetrics() {
    const dwp = this.unifier.registry.get('dwp');
    if (!dwp || !dwp.client) throw new Error('DWP client not available');
    return dwp.client.metrics();
  }

  start() {
    this._subscribe(EVENT_TYPES.MODULE_HEALTH, (event) => {
      this._moduleStatuses = event.payload;
    });
  }

  destroy() {
    super.destroy();
  }
}

module.exports = { SettingsBridge };
