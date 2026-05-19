'use strict';

class VoltUnifierError extends Error {
  constructor(code, message, options = {}) {
    super(message);
    this.name = 'VoltUnifierError';
    this.code = code;
    this.module = options.module || null;
    this.cause = options.cause || null;
    this.recoverable = options.recoverable !== false;
  }
}

class ModuleOfflineError extends VoltUnifierError {
  constructor(moduleId, message) {
    super('MODULE_OFFLINE', message || `Module ${moduleId} is offline`, {
      module: moduleId,
      recoverable: true,
    });
    this.name = 'ModuleOfflineError';
  }
}

class ModuleTimeoutError extends VoltUnifierError {
  constructor(moduleId, timeoutMs) {
    super('MODULE_TIMEOUT', `Module ${moduleId} timed out after ${timeoutMs}ms`, {
      module: moduleId,
      recoverable: true,
    });
    this.name = 'ModuleTimeoutError';
    this.timeoutMs = timeoutMs;
  }
}

class PermissionDeniedError extends VoltUnifierError {
  constructor(moduleId, command) {
    super('PERMISSION_DENIED', `Permission denied: ${command} on ${moduleId}`, {
      module: moduleId,
      recoverable: false,
    });
    this.name = 'PermissionDeniedError';
    this.command = command;
  }
}

class InvalidSbpMessageError extends VoltUnifierError {
  constructor(reason) {
    super('INVALID_SBP_MESSAGE', `Invalid SBP message: ${reason}`, {
      recoverable: false,
    });
    this.name = 'InvalidSbpMessageError';
  }
}

class UnsupportedOpcodeError extends VoltUnifierError {
  constructor(moduleId, opcode) {
    super('UNSUPPORTED_OPCODE', `Opcode ${opcode} not supported by ${moduleId}`, {
      module: moduleId,
      recoverable: false,
    });
    this.name = 'UnsupportedOpcodeError';
  }
}

class TransportError extends VoltUnifierError {
  constructor(moduleId, message, cause) {
    super('TRANSPORT_ERROR', message, {
      module: moduleId,
      cause,
      recoverable: true,
    });
    this.name = 'TransportError';
  }
}

class ReconnectFailedError extends VoltUnifierError {
  constructor(moduleId, attempts) {
    super('RECONNECT_FAILED', `Failed to reconnect ${moduleId} after ${attempts} attempts`, {
      module: moduleId,
      recoverable: false,
    });
    this.name = 'ReconnectFailedError';
    this.attempts = attempts;
  }
}

class ShutdownBlockedError extends VoltUnifierError {
  constructor(reason) {
    super('SHUTDOWN_BLOCKED', `Shutdown blocked: ${reason}`, {
      recoverable: true,
    });
    this.name = 'ShutdownBlockedError';
  }
}

class AscFailedError extends VoltUnifierError {
  constructor(message, code) {
    super('ASC_FAILED', `ASC failed: ${message}`, {
      recoverable: true,
    });
    this.name = 'AscFailedError';
    this.exitCode = code;
  }
}

class CapabilityMissingError extends VoltUnifierError {
  constructor(moduleId, capability) {
    super('CAPABILITY_MISSING', `Module ${moduleId} missing capability: ${capability}`, {
      module: moduleId,
      recoverable: false,
    });
    this.name = 'CapabilityMissingError';
  }
}

class UnsafeDebugBindError extends VoltUnifierError {
  constructor(bindAddress) {
    super('UNSAFE_DEBUG_BIND', `Refusing to bind debug server to ${bindAddress} (must be 127.0.0.1)`, {
      recoverable: false,
    });
    this.name = 'UnsafeDebugBindError';
  }
}

module.exports = {
  VoltUnifierError,
  ModuleOfflineError,
  ModuleTimeoutError,
  PermissionDeniedError,
  InvalidSbpMessageError,
  UnsupportedOpcodeError,
  TransportError,
  ReconnectFailedError,
  ShutdownBlockedError,
  AscFailedError,
  CapabilityMissingError,
  UnsafeDebugBindError,
};
