'use strict';

class DashboardFeed {
  constructor() {
    this._listeners = new Map();
    this._nextId = 1;
  }

  subscribe(ws) {
    if (!ws || typeof ws.send !== 'function') {
      throw new Error('WebSocket must expose a send() method');
    }
    const id = `feed-${this._nextId++}`;
    this._listeners.set(id, ws);

    const unsubscribe = () => {
      this._listeners.delete(id);
    };

    return unsubscribe;
  }

  broadcast(data) {
    const payload = typeof data === 'string' ? data : JSON.stringify(data);
    for (const [id, ws] of this._listeners) {
      try {
        ws.send(payload);
      } catch (err) {
        this._listeners.delete(id);
      }
    }
  }

  listenerCount() {
    return this._listeners.size;
  }
}

module.exports = { DashboardFeed };
