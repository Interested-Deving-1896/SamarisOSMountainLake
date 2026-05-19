'use strict';
const { UnsafeDebugBindError } = require('../errors');

class LocalOnlyGuard {
  static validateBindAddress(address) {
    if (address === '0.0.0.0') throw new UnsafeDebugBindError(address);
    if (address !== '127.0.0.1' && address !== 'localhost') {
      throw new UnsafeDebugBindError(address);
    }
    return true;
  }

  static validateOrigin(origin) {
    if (!origin) return false;
    const allowed = ['127.0.0.1:3000', 'localhost:3000', '127.0.0.1:5173', 'localhost:5173', 'file://'];
    return allowed.some(a => origin.includes(a));
  }

  static isPublicBind(address) {
    return address === '0.0.0.0' || address === '::';
  }
}

module.exports = { LocalOnlyGuard };
