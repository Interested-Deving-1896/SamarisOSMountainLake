'use strict';
const { EVENT_TYPES } = require('../constants');
const { createEvent } = require('../events/eventTypes');

class ReadinessState {
  constructor(unifier) {
    this.unifier = unifier;
    this._ready = false;
    this._readyCallbacks = [];
    this._subscriptionIds = [];
  }

  markReady() {
    if (this._ready) return;
    this._ready = true;
    this.unifier.eventBus.publish(
      createEvent(EVENT_TYPES.BOOT_READY, 'readiness-state', { ready: true, timestamp: Date.now() }, 'info')
    );
    for (const cb of this._readyCallbacks) {
      try {
        cb();
      } catch (_) {
      }
    }
    this._readyCallbacks = [];
  }

  isReady() {
    if (this._ready) return true;

    const modules = this.unifier.registry.getAll();
    for (const entry of modules) {
      const cfg = this.unifier.config.modules[entry.id];
      if (cfg && cfg.enabled === false) continue;
      if (entry.status !== 'online' && entry.status !== 'degraded') {
        return false;
      }
    }

    this.markReady();
    return true;
  }

  onReady(handler) {
    if (typeof handler !== 'function') {
      throw new Error('handler must be a function');
    }
    if (this._ready) {
      process.nextTick(handler);
      return;
    }
    this._readyCallbacks.push(handler);
  }
}

module.exports = { ReadinessState };
