'use strict';
const { ServiceHealthClient } = require('./serviceHealthClient');
const { OPOCODES } = require('../constants');

class VumClient extends ServiceHealthClient {
  constructor(unifier) {
    super(unifier, 'vum', unifier.config.modules.vum);
  }

  async status() {
    return this.request(OPOCODES.USB_STATUS, Buffer.alloc(0));
  }

  async read(path) {
    return this.request(OPOCODES.USB_READ, Buffer.from(String(path)));
  }

  async write(path, data) {
    const payload = Buffer.concat([
      Buffer.from(String(path) + '\0'),
      Buffer.from(data),
    ]);
    return this.request(OPOCODES.USB_WRITE, payload);
  }

  async flush() {
    return this.request(OPOCODES.USB_FLUSH, Buffer.alloc(0));
  }

  async cacheStatus() {
    return this.request(OPOCODES.USB_CACHE_STATUS, Buffer.alloc(0));
  }

  async prefetch(path) {
    return this.request(OPOCODES.USB_PREFETCH, Buffer.from(String(path)));
  }

  async eject() {
    return this.request(OPOCODES.USB_EJECT, Buffer.alloc(0));
  }

  async mount() {
    return this.request(OPOCODES.USB_MOUNT, Buffer.alloc(0));
  }

  async unmount() {
    return this.request(OPOCODES.USB_UNMOUNT, Buffer.alloc(0));
  }

  async journalStatus() {
    return this.request(OPOCODES.USB_JOURNAL_STATUS, Buffer.alloc(0));
  }

  async recoveryRun() {
    return this.request(OPOCODES.USB_RECOVERY_RUN, Buffer.alloc(0));
  }

  async durabilityStatus() {
    return this.request(OPOCODES.USB_DURABILITY_STATUS, Buffer.alloc(0));
  }

  async metricsSnapshot() {
    return this.request(OPOCODES.USB_METRICS_SNAPSHOT, Buffer.alloc(0));
  }
}

module.exports = { VumClient };
