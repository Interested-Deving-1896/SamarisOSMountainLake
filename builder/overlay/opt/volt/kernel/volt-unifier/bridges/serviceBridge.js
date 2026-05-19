'use strict';

class ServiceBridge {
  constructor(unifier) {
    if (new.target === ServiceBridge) {
      throw new Error('ServiceBridge is abstract and cannot be instantiated directly');
    }
    this.unifier = unifier;
    this._subscriptionIds = [];
  }

  init() {
    throw new Error('init() must be implemented by subclass');
  }

  destroy() {
    for (const id of this._subscriptionIds) {
      this.unifier.eventBus.unsubscribe(id);
    }
    this._subscriptionIds = [];
  }

  _subscribe(eventType, handler) {
    const id = this.unifier.eventBus.subscribe(eventType, handler);
    this._subscriptionIds.push(id);
    return id;
  }
}

module.exports = { ServiceBridge };
