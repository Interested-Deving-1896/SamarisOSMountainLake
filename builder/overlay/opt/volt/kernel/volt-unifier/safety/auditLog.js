'use strict';

const MAX_ENTRIES = 1000;

class AuditLog {
  constructor() {
    this._entries = [];
  }

  record(action, moduleId, allowed, reason, source) {
    const entry = {
      timestamp: Date.now(),
      action,
      moduleId,
      allowed,
      reason,
      source: source || null,
    };

    this._entries.push(entry);
    if (this._entries.length > MAX_ENTRIES) {
      this._entries.shift();
    }

    return entry;
  }

  recent(count) {
    const take = typeof count === 'number' && count > 0 ? count : 50;
    return this._entries.slice(-take);
  }

  clear() {
    this._entries = [];
  }

  count() {
    return this._entries.length;
  }
}

module.exports = { AuditLog };
