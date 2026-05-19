'use strict';
const { execFile } = require('node:child_process');
const { EventEmitter } = require('node:events');
const fs = require('node:fs');

class AscClient extends EventEmitter {
  constructor(unifier) {
    super();
    this.on('error', () => {});
    this.unifier = unifier;
    this.moduleId = 'asc';
    this.config = (unifier && unifier.config && unifier.config.modules && unifier.config.modules.asc) || {};
    this.status = 'offline';
    this.lastError = null;
    this.degradedReason = null;
    this.errorCount = 0;
    this.reconnectCount = 0;
    this.lastHeartbeatAt = null;
    this.generatedConfig = this.config.generatedConfig || '/run/samaris/adaptive.generated.toml';
    this._binary = this._resolveBinary();
  }

  _resolveBinary() {
    if (process.env.VOLT_ASC_BIN) return process.env.VOLT_ASC_BIN;
    if (this.config.binary) {
      if (this.config.binary.startsWith('/')) return this.config.binary;
      return this.config.binary;
    }
    const arch = process.arch === 'arm64' ? 'aarch64' : 'x86_64';
    return `/opt/volt/bin/volt-asc-${arch}`;
  }

  async connect() { return this.refresh(); }
  async disconnect() { this.status = 'offline'; }
  isOnline() { return this.status === 'online'; }

  async refresh() {
    if (fs.existsSync(this.generatedConfig)) {
      this.status = 'online';
      this.lastHeartbeatAt = Date.now();
      this.lastError = null;
      this.degradedReason = null;
      return { ok: true, generatedConfig: this.generatedConfig };
    }

    this.status = 'degraded';
    this.degradedReason = `generated config missing: ${this.generatedConfig}`;
    return { ok: false, error: this.degradedReason };
  }

  async request(opcode, _payload, _timeoutMs = 2000) {
    const result = await this.refresh();
    return Buffer.from(JSON.stringify({
      ok: result.ok,
      moduleId: this.moduleId,
      status: this.status,
      generatedConfig: this.generatedConfig,
      opcode,
      timestamp: Date.now(),
    }));
  }

  async probe() { return this._run(['probe']); }
  async generate() { return this._run(['generate']); }
  async generateAndWrite(path) {
    return this._run(path ? ['--write', path, 'write'] : ['write']);
  }
  async explain() { return this._run(['explain']); }
  async check() { return this._run(['check']); }

  async _run(args) {
    const filtered = args.filter(Boolean);
    return new Promise((resolve) => {
      execFile(this._binary, filtered, { timeout: 15000 }, (error, stdout, stderr) => {
        if (error) {
          this.status = 'degraded';
          this.lastError = error.message;
          this.degradedReason = error.message;
          this.errorCount++;
          this.emit('error', error);
          resolve({ ok: false, error: error.message, stdout, stderr });
        } else {
          this.status = 'online';
          this.lastHeartbeatAt = Date.now();
          this.lastError = null;
          this.degradedReason = null;
          this.emit('result', { stdout, stderr });
          resolve({ ok: true, stdout, stderr });
        }
      });
    });
  }
}

module.exports = { AscClient };
