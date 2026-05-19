'use strict';

const { CapabilityMissingError, PermissionDeniedError } = require('../errors');
const { SENSITIVE_COMMANDS, OPOCODES } = require('../constants');

const SENSITIVE_OPCODES = new Set();
for (const name of SENSITIVE_COMMANDS) {
  if (typeof OPOCODES[name] === 'number') {
    SENSITIVE_OPCODES.add(OPOCODES[name]);
  }
}

function checkCapability(moduleCapabilities, requiredCapability) {
  if (!Array.isArray(moduleCapabilities)) {
    return false;
  }
  return moduleCapabilities.includes(requiredCapability);
}

function requireCapability(moduleCapabilities, requiredCapability) {
  if (!checkCapability(moduleCapabilities, requiredCapability)) {
    const moduleId = (moduleCapabilities && moduleCapabilities.module) || 'unknown';
    throw new CapabilityMissingError(moduleId, requiredCapability);
  }
}

function isSensitiveCommand(opcodeName) {
  return SENSITIVE_COMMANDS.includes(opcodeName);
}

function isSensitiveOpcode(opcode) {
  return SENSITIVE_OPCODES.has(opcode);
}

function requirePermission(moduleId, operation) {
  if (SENSITIVE_COMMANDS.includes(operation)) {
    throw new PermissionDeniedError(moduleId, operation);
  }
}

module.exports = {
  checkCapability,
  requireCapability,
  isSensitiveCommand,
  isSensitiveOpcode,
  requirePermission,
};
