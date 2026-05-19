'use strict';
const { ServiceHealthClient } = require('./serviceHealthClient');
const { OPOCODES } = require('../constants');

class VrmClient extends ServiceHealthClient {
  constructor(unifier) {
    super(unifier, 'vrm', unifier.config.modules.vrm);
  }

  async status() {
    return this.request(OPOCODES.RAM_STATUS, Buffer.alloc(0));
  }

  async flush() {
    return this.request(OPOCODES.RAM_FLUSH, Buffer.alloc(0));
  }

  async registerApp(app) {
    return this.request(OPOCODES.RAM_REGISTER_APP, Buffer.from(JSON.stringify(app)));
  }

  async unregisterApp(appId) {
    return this.request(OPOCODES.RAM_UNREGISTER_APP, Buffer.from(String(appId)));
  }

  async setQuota(appId, quota) {
    return this.request(OPOCODES.RAM_SET_QUOTA, Buffer.from(JSON.stringify({ appId, quota })));
  }

  async appStatus(appId) {
    return this.request(OPOCODES.RAM_APP_STATUS, Buffer.from(String(appId)));
  }

  async compressApp(appId) {
    return this.request(OPOCODES.RAM_COMPRESS_APP, Buffer.from(String(appId)));
  }

  async releaseCache() {
    return this.request(OPOCODES.RAM_RELEASE_CACHE, Buffer.alloc(0));
  }

  async snapshot() {
    return this.request(OPOCODES.RAM_SNAPSHOT, Buffer.alloc(0));
  }

  async subscribePressureEvents() {
    return this.request(OPOCODES.RAM_SUBSCRIBE_EVENTS, Buffer.alloc(0));
  }
}

module.exports = { VrmClient };
