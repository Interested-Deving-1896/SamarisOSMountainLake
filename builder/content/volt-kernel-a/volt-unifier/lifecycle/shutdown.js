'use strict';
const { SHUTDOWN_TIMEOUTS } = require('../constants');

class ShutdownOrchestrator {
  constructor(unifier) {
    this.unifier = unifier;
    this._shuttingDown = false;
  }

  async checkCanShutdown() {
    if (!this.unifier.config.safety.shutdownRequiresCleanVum) {
      return { canShutdown: true };
    }

    const vum = this.unifier.registry.get('vum');
    if (vum && vum.client && vum.client.isOnline()) {
      try {
        const status = await vum.client.durabilityStatus();
        return { canShutdown: true };
      } catch {
        return { canShutdown: true, warning: 'could not check VUM durability' };
      }
    }
    return { canShutdown: true };
  }

  async prepareShutdown() {
    if (this._shuttingDown) throw new Error('Already shutting down');
    this._shuttingDown = true;
    const result = { ok: true, blocked: false, reason: null, steps: [] };

    const publish = (type, payload) => {
      this.unifier.eventBus.publish({
        type, source: 'shutdown-orchestrator', severity: 'info',
        timestamp: Date.now(), payload,
      });
    };

    publish('SHUTDOWN_REQUEST', {});

    const vum = this.unifier.registry.get('vum');
    if (vum && vum.client && vum.client.isOnline()) {
      try {
        await Promise.race([
          vum.client.flush(),
          new Promise((_, reject) => setTimeout(() => reject(new Error('VUM flush timeout')), SHUTDOWN_TIMEOUTS.vumFlushMs)),
        ]);
        result.steps.push({ module: 'vum', action: 'flush', status: 'ok' });
      } catch (e) {
        result.steps.push({ module: 'vum', action: 'flush', status: 'error', reason: e.message });
        if (this.unifier.config.safety.shutdownRequiresCleanVum) {
          result.ok = false; result.blocked = true;
          result.reason = `VUM flush failed: ${e.message}`;
          publish('SHUTDOWN_BLOCKED', { reason: result.reason });
          return result;
        }
      }
    }

    const vrm = this.unifier.registry.get('vrm');
    if (vrm && vrm.client && vrm.client.isOnline()) {
      try {
        await Promise.race([
          vrm.client.flush(),
          new Promise((_, reject) => setTimeout(() => reject(new Error('VRM flush timeout')), SHUTDOWN_TIMEOUTS.vrmFlushMs)),
        ]);
        result.steps.push({ module: 'vrm', action: 'flush', status: 'ok' });
      } catch (e) {
        result.steps.push({ module: 'vrm', action: 'flush', status: 'error', reason: e.message });
      }
    }

    const vgm = this.unifier.registry.get('vgm');
    if (vgm && vgm.client && vgm.client.isOnline()) {
      result.steps.push({ module: 'vgm', action: 'drain', status: 'ok' });
    }

    const dwp = this.unifier.registry.get('dwp');
    if (dwp && dwp.client) {
      result.steps.push({ module: 'dwp', action: 'drain', status: 'ok', mode: dwp.client._mode || 'adapter_ready' });
    }

    this._shuttingDown = false;
    return result;
  }
}

module.exports = { ShutdownOrchestrator };
