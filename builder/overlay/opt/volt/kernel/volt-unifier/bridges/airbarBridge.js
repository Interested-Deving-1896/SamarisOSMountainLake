'use strict';
const { ServiceBridge } = require('./serviceBridge');
const { EVENT_TYPES } = require('../constants');

class AirBarBridge extends ServiceBridge {
  constructor(unifier) {
    super(unifier);
    this._lastRamPressure = null;
    this._lastUsbDurability = null;
    this._lastGpuThermal = null;
    this._lastHealth = null;
    this._onEvent = null;
  }

  setOnEvent(callback) {
    if (typeof callback === 'function') {
      this._onEvent = callback;
    }
  }

  start() {
    this._subscribe(EVENT_TYPES.RAM_PRESSURE, (event) => {
      this._lastRamPressure = event.payload;
      this._dispatch('ram_pressure', {
        level: event.payload ? event.payload.level || event.payload.pressure : null,
        severity: event.severity,
        timestamp: event.timestamp,
      });
    });

    this._subscribe(EVENT_TYPES.USB_DURABILITY, (event) => {
      this._lastUsbDurability = event.payload;
      this._dispatch('usb_durability', {
        dirtyWrites: event.payload ? event.payload.dirtyWrites : null,
        severity: event.severity,
        timestamp: event.timestamp,
      });
    });

    this._subscribe(EVENT_TYPES.GPU_THERMAL, (event) => {
      this._lastGpuThermal = event.payload;
      this._dispatch('gpu_thermal', {
        temperature: event.payload ? event.payload.temperature : null,
        severity: event.severity,
        timestamp: event.timestamp,
      });
    });

    this._subscribe(EVENT_TYPES.MODULE_HEALTH, (event) => {
      this._lastHealth = event.payload;
      this._dispatch('module_health', {
        overallStatus: event.payload ? event.payload.overallStatus : null,
        warnings: event.payload ? event.payload.warnings : [],
        criticalIssues: event.payload ? event.payload.criticalIssues : [],
        severity: event.severity,
        timestamp: event.timestamp,
      });
    });
  }

  destroy() {
    super.destroy();
  }

  _dispatch(type, data) {
    if (this._onEvent) {
      this._onEvent({ type, data });
    }
  }
}

module.exports = { AirBarBridge };
