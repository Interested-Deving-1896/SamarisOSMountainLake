'use strict';
const { ServiceBridge } = require('./serviceBridge');
const { EVENT_TYPES } = require('../constants');

const ORBIT_APPS = [
  { id: 'orbit-llm',  name: 'Orbit LLM',  quota_mb: 2048, priority: 'critical' },
  { id: 'orbit-stt',  name: 'Orbit STT',  quota_mb: 512,  priority: 'high' },
  { id: 'orbit-tts',  name: 'Orbit TTS',  quota_mb: 512,  priority: 'normal' },
];

class OrbitBridge extends ServiceBridge {
  constructor(unifier) {
    super(unifier);
    this._registered = false;
    this._burstActive = false;
    this._lastGpuSnapshot = null;
    this._lastThermalEvent = null;
  }

  async init() {
    await this._registerWithVrm();
    this._setupDwp();
    this._subscribeEvents();
  }

  async _registerWithVrm() {
    const vrm = this._getClient('vrm');
    if (!vrm) return;

    for (const app of ORBIT_APPS) {
      try {
        await vrm.registerApp(app);
      } catch {
        // VRM may already have these apps pre-registered
      }
    }
    this._registered = true;
  }

  _setupDwp() {
    const dwp = this._getClient('dwp');
    if (!dwp) return;
    // DWP already has orbit configured as critical in config
    // We can trigger bursts when inference starts
  }

  _subscribeEvents() {
    this._subscribe(EVENT_TYPES.GPU_THERMAL, (event) => {
      this._lastThermalEvent = event;
    });
  }

  async notifyInferenceStart(mode) {
    const dwp = this._getClient('dwp');
    if (dwp) {
      try {
        await dwp.requestOrbitBurst({ mode, priority: 'critical' });
        this._burstActive = true;
      } catch {
        // burst not available, continue without
      }
    }
  }

  async notifyInferenceEnd() {
    this._burstActive = false;
    // DWP burst cooldown is handled automatically
  }

  async logGpuMetrics() {
    const vgm = this._getClient('vgm');
    if (!vgm) return null;
    try {
      const raw = await vgm.metricsSnapshot();
      this._lastGpuSnapshot = raw;
      return raw;
    } catch {
      return this._lastGpuSnapshot;
    }
  }

  getStatus() {
    return {
      registered: this._registered,
      burstActive: this._burstActive,
      lastGpuSnapshot: this._lastGpuSnapshot,
      lastThermalEvent: this._lastThermalEvent,
    };
  }

  _getClient(moduleId) {
    const entry = this.unifier?.registry?.get(moduleId);
    return entry?.client || null;
  }

  start() {
    void this.init();
  }

  destroy() {
    super.destroy();
  }
}

module.exports = { OrbitBridge };
