'use strict';
const { createDashboardSnapshot } = require('./snapshots');

class MetricsAggregator {
  constructor(unifier) {
    this.unifier = unifier;
    this._history = [];
    this._maxHistory = 100;
  }

  async getDashboardSnapshot() {
    const snapshotData = {
      timestamp: Date.now(),
      health: null,
      ram: null,
      usb: null,
      gpu: null,
      workers: null,
      adaptive: null,
      events: [],
    };

    if (this.unifier.healthMonitor) {
      snapshotData.health = this.unifier.healthMonitor.getSystemHealthSnapshot();
    }

    const vrm = this.unifier.registry.get('vrm');
    if (vrm && vrm.client && vrm.client.isOnline()) {
      try {
        const raw = await vrm.client.snapshot();
        snapshotData.ram = { available: true, data: raw };
      } catch {
        snapshotData.ram = { available: true, error: 'snapshot failed' };
      }
    } else {
      snapshotData.ram = { available: false, status: vrm ? vrm.status : 'offline' };
    }

    const vum = this.unifier.registry.get('vum');
    if (vum && vum.client && vum.client.isOnline()) {
      try {
        const raw = await vum.client.metricsSnapshot();
        snapshotData.usb = { available: true, data: raw };
      } catch {
        snapshotData.usb = { available: true, error: 'metrics failed' };
      }
    } else {
      snapshotData.usb = { available: false, status: vum ? vum.status : 'offline' };
    }

    const vgm = this.unifier.registry.get('vgm');
    if (vgm && vgm.client && vgm.client.isOnline()) {
      try {
        const raw = await vgm.client.metricsSnapshot();
        snapshotData.gpu = { available: true, data: raw };
      } catch {
        snapshotData.gpu = { available: true, error: 'metrics failed' };
      }
    } else {
      snapshotData.gpu = { available: false, status: vgm ? vgm.status : 'offline' };
    }

    const dwp = this.unifier.registry.get('dwp');
    if (dwp && dwp.client) {
      try {
        const raw = await dwp.client.metrics();
        snapshotData.workers = { available: true, data: raw };
      } catch {
        snapshotData.workers = { available: true, error: 'metrics failed' };
      }
    } else {
      snapshotData.workers = { available: false, status: dwp ? dwp.status : 'offline' };
    }

    const asc = this.unifier.registry.get('asc');
    if (asc && asc.client) {
      try {
        const raw = await asc.client.explain();
        snapshotData.adaptive = { available: true, data: raw };
      } catch {
        snapshotData.adaptive = { available: false, error: 'explain failed' };
      }
    } else {
      snapshotData.adaptive = { available: false, status: asc ? asc.status : 'offline' };
    }

    if (this.unifier.eventBus) {
      snapshotData.events = this.unifier.eventBus.history(20);
    }

    const snapshot = createDashboardSnapshot(snapshotData);

    this._history.push(snapshot);
    if (this._history.length > this._maxHistory) {
      this._history.shift();
    }

    return snapshot;
  }

  getSnapshotHistory(limit) {
    const take = typeof limit === 'number' && limit > 0 ? limit : 10;
    return this._history.slice(-take);
  }
}

module.exports = { MetricsAggregator };
