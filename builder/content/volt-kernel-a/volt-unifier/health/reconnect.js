'use strict';
const { RECONNECT_BACKOFF } = require('../constants');

class ReconnectPolicy {
  constructor() {
    this._attempts = new Map();
  }

  getDelay(moduleId) {
    const count = this._attempts.get(moduleId) || 0;
    const delay = RECONNECT_BACKOFF.initialMs * Math.pow(RECONNECT_BACKOFF.factor, count);
    const capped = Math.min(delay, RECONNECT_BACKOFF.maxMs);
    const jitter = capped * RECONNECT_BACKOFF.jitter * (Math.random() * 2 - 1);
    return Math.max(0, Math.round(capped + jitter));
  }

  recordAttempt(moduleId) {
    const count = (this._attempts.get(moduleId) || 0) + 1;
    this._attempts.set(moduleId, count);
    if (count >= RECONNECT_BACKOFF.maxAttempts) {
      return false;
    }
    return true;
  }

  reset(moduleId) {
    this._attempts.delete(moduleId);
  }
}

module.exports = { ReconnectPolicy };
