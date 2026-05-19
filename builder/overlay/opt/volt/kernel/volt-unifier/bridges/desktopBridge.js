'use strict';
const { ServiceBridge } = require('./serviceBridge');
const { EVENT_TYPES } = require('../constants');

class DesktopBridge extends ServiceBridge {
  constructor(unifier) {
    super(unifier);
    this._healthMonitor = unifier.healthMonitor;
  }

  async onFramePressure(pressure) {
    const dwp = this.unifier.registry.get('dwp');
    if (dwp && dwp.client && dwp.client.setDesktopPressure) {
      await dwp.client.setDesktopPressure(pressure);
    }
    this.unifier.eventBus.publish({
      type: EVENT_TYPES.GPU_FRAME_PRESSURE,
      source: 'desktop-bridge',
      severity: 'info',
      timestamp: Date.now(),
      payload: { pressure },
    });
  }

  getHealthSnapshot() {
    if (this._healthMonitor) {
      return this._healthMonitor.getSystemHealthSnapshot();
    }
    return null;
  }

  start() {
    this._subscribe(EVENT_TYPES.MODULE_HEALTH, (event) => {
      this._lastHealthEvent = event;
    });
    this._subscribe(EVENT_TYPES.BOOT_READY, (event) => {
      this._bootReady = true;
    });
  }

  destroy() {
    super.destroy();
  }
}

module.exports = { DesktopBridge };
