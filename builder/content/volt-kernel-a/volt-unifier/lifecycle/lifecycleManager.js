'use strict';

const VALID_STATES = Object.freeze(['uninitialized', 'starting', 'running', 'stopping', 'stopped']);

const ALLOWED_TRANSITIONS = Object.freeze({
  uninitialized: ['starting'],
  starting: ['running', 'stopping'],
  running: ['stopping'],
  stopping: ['stopped'],
  stopped: [],
});

class LifecycleManager {
  constructor() {
    this._state = 'uninitialized';
    this._listeners = new Map();
  }

  start() {
    this._transition('starting');
    return this;
  }

  stop() {
    this._transition('stopping');
    return this;
  }

  markRunning() {
    this._transition('running');
    return this;
  }

  markStopped() {
    this._transition('stopped');
    return this;
  }

  getState() {
    return this._state;
  }

  isRunning() {
    return this._state === 'running';
  }

  isStarting() {
    return this._state === 'starting';
  }

  isStopped() {
    return this._state === 'stopped';
  }

  on(state, handler) {
    if (!VALID_STATES.includes(state)) {
      throw new Error(`Invalid lifecycle state: ${state}`);
    }
    if (typeof handler !== 'function') {
      throw new Error('Handler must be a function');
    }
    if (!this._listeners.has(state)) {
      this._listeners.set(state, []);
    }
    this._listeners.get(state).push(handler);
    return this;
  }

  _transition(target) {
    const current = this._state;
    const allowed = ALLOWED_TRANSITIONS[current];

    if (!allowed || !allowed.includes(target)) {
      throw new Error(
        `Invalid lifecycle transition: ${current} -> ${target}`
      );
    }

    this._state = target;
    this._emit(target, current);
  }

  _emit(state, previous) {
    const handlers = this._listeners.get(state);
    if (handlers) {
      for (const handler of handlers) {
        try {
          handler(state, previous);
        } catch (_) {
        }
      }
    }
  }
}

module.exports = { LifecycleManager };
