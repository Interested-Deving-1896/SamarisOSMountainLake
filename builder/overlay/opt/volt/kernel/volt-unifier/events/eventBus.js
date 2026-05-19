'use strict';

const { EVENT_TYPES, EVENT_ROUTING } = require('../constants');
const { isVoltEvent } = require('./eventTypes');

const DEFAULT_ROUTING_TABLE = { ...EVENT_ROUTING };

class SystemEventBus {
  constructor(kernelEventBus) {
    if (!kernelEventBus || typeof kernelEventBus.emit !== 'function') {
      throw new Error('kernelEventBus must expose an emit() method');
    }

    this._bus = kernelEventBus;
    this._subscriptions = new Map();
    this._history = [];
    this._maxHistory = 500;
    this._routingTable = { ...DEFAULT_ROUTING_TABLE };
  }

  publish(event) {
    if (!isVoltEvent(event)) {
      throw new Error('Cannot publish: invalid VoltEvent object');
    }

    this._history.push(event);
    if (this._history.length > this._maxHistory) {
      this._history.shift();
    }

    const routes = this._routingTable[event.type] || [];
    this._bus.emit(`unifier:${event.type}`, event);

    for (const route of routes) {
      this._bus.emit(`unifier:${event.type}:${route}`, event);
    }

    const subs = this._subscriptions.get(event.type);
    if (subs) {
      for (const sub of subs) {
        try {
          sub.handler(event);
        } catch (err) {
          if (this._subscriptions.has('__error')) {
            const errorSubs = this._subscriptions.get('__error');
            for (const es of errorSubs) {
              try {
                es.handler({ event, error: err, subscriberId: sub.id });
              } catch (_) {
              }
            }
          }
        }
      }
    }
  }

  subscribe(type, handler) {
    if (!type || typeof type !== 'string') {
      throw new Error('Subscription type must be a non-empty string');
    }
    if (typeof handler !== 'function') {
      throw new Error('Subscription handler must be a function');
    }

    const id = `sub-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

    if (!this._subscriptions.has(type)) {
      this._subscriptions.set(type, []);
    }
    this._subscriptions.get(type).push({ id, handler });

    return id;
  }

  subscribeOnce(type, handler) {
    const wrapped = (event) => {
      this.unsubscribe(id);
      handler(event);
    };
    const id = this.subscribe(type, wrapped);
    return id;
  }

  unsubscribe(id) {
    if (!id || typeof id !== 'string') {
      return false;
    }

    for (const [, subs] of this._subscriptions) {
      const idx = subs.findIndex(s => s.id === id);
      if (idx >= 0) {
        subs.splice(idx, 1);
        return true;
      }
    }
    return false;
  }

  history(limit) {
    const take = typeof limit === 'number' && limit > 0 ? limit : 50;
    return this._history.slice(-take);
  }

  historyByType(type, limit) {
    const take = typeof limit === 'number' && limit > 0 ? limit : 50;
    return this._history
      .filter(e => e.type === type)
      .slice(-take);
  }

  clear() {
    this._history = [];
  }

  setRouting(eventType, routes) {
    if (!eventType || typeof eventType !== 'string') {
      throw new Error('Routing eventType must be a non-empty string');
    }
    if (!Array.isArray(routes)) {
      throw new Error('Routes must be an array of strings');
    }
    this._routingTable[eventType] = Array.from(routes);
  }

  getRouting(eventType) {
    return this._routingTable[eventType] || [];
  }

  subscriberCount(type) {
    const subs = this._subscriptions.get(type);
    return subs ? subs.length : 0;
  }
}

module.exports = { SystemEventBus };
