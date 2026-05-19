'use strict';
const fs = require('node:fs');

class BootClient {
  constructor(unifier) {
    this.unifier = unifier;
    this.moduleId = 'volt-boot';
    this.status = 'online';
    this.bootStartedAt = Date.now();
  }

  isAscComplete() { return fs.existsSync('/run/volt-asc.complete'); }
  isKernelBStarted() { return fs.existsSync('/run/volt-kernel-b.started'); }

  ascGeneratedConfig() {
    try { return fs.readFileSync('/run/samaris/adaptive.generated.toml', 'utf-8'); }
    catch { return null; }
  }
}

module.exports = { BootClient };
