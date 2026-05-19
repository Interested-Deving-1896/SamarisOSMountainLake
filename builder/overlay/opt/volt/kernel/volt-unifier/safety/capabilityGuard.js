'use strict';
const { CapabilityMissingError, PermissionDeniedError } = require('../errors');
const { SENSITIVE_COMMANDS } = require('../constants');

class CapabilityGuard {
  static requireCapability(capabilities, required) {
    if (!capabilities) throw new CapabilityMissingError('unknown', required);
    if (!capabilities.features || !Array.isArray(capabilities.features)) {
      throw new CapabilityMissingError('unknown', required);
    }
    const hasWildcard = capabilities.features.some(f => f.endsWith('.*') && required.startsWith(f.slice(0, -2)));
    const hasExact = capabilities.features.includes(required);
    if (!hasExact && !hasWildcard) throw new CapabilityMissingError('unknown', required);
    return true;
  }

  static isSensitiveCommand(command) {
    return SENSITIVE_COMMANDS.includes(command);
  }

  static checkPermission(capabilities, moduleId, command) {
    if (this.isSensitiveCommand(command)) {
      this.requireCapability(capabilities, command.toLowerCase());
    }
    return true;
  }
}

module.exports = { CapabilityGuard };
