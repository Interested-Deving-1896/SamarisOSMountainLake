'use strict';
const { ServiceHealthClient } = require('./serviceHealthClient');
const { DwpLocalFallback } = require('./dwpLocalFallback');

class DwpClient extends ServiceHealthClient {
  constructor(unifier) {
    super(unifier, 'dwp', unifier.config.modules.dwp);
    this.fallback = new DwpLocalFallback(unifier);
    this._mode = 'adapter_ready';
  }

  async submitJob(job) {
    if (this.isOnline()) return this.request(0x50, Buffer.from(JSON.stringify(job)));
    return this.fallback.submitJob(job);
  }

  async setDesktopPressure(pressure) {
    if (this.isOnline()) return this.request(0x51, Buffer.from(JSON.stringify({ pressure })));
    this.fallback.setDesktopPressure(pressure);
    return { ok: true, mode: 'local_fallback' };
  }

  async requestOrbitBurst(request) {
    if (this.isOnline()) return this.request(0x52, Buffer.from(JSON.stringify(request)));
    return this.fallback.requestOrbitBurst(request);
  }

  async metrics() {
    if (this.isOnline()) {
      const raw = await this.request(0x53, Buffer.alloc(0));
      return JSON.parse(raw.toString());
    }
    return this.fallback.metrics();
  }

  getStatus() {
    return {
      moduleId: 'dwp',
      mode: this._mode,
      status: this.status,
      usingLocalFallback: !this.isOnline(),
      fallbackMetrics: this.fallback.getSnapshot(),
      degradedReason: this.degradedReason,
    };
  }
}

module.exports = { DwpClient };
