'use strict';
const { ServiceBridge } = require('./serviceBridge');

class FinderBridge extends ServiceBridge {
  constructor(unifier) {
    super(unifier);
  }

  async readFile(path) {
    const vum = this.unifier.registry.get('vum');
    if (!vum || !vum.client) throw new Error('VUM client not available');
    return vum.client.read(path);
  }

  async writeFile(path, data) {
    const vum = this.unifier.registry.get('vum');
    if (!vum || !vum.client) throw new Error('VUM client not available');
    return vum.client.write(path, data);
  }

  async eject(device) {
    const vum = this.unifier.registry.get('vum');
    if (!vum || !vum.client) throw new Error('VUM client not available');
    return vum.client.eject(device);
  }

  getStatus() {
    const vum = this.unifier.registry.get('vum');
    if (!vum || !vum.client) {
      return { moduleId: 'vum', status: 'offline' };
    }
    return {
      moduleId: 'vum',
      status: vum.client.status || vum.status,
      degradedReason: vum.client.degradedReason || vum.health.degradedReason,
      lastError: vum.client.lastError || vum.health.lastError,
    };
  }

  init() {
  }

  destroy() {
    super.destroy();
  }
}

module.exports = { FinderBridge };
