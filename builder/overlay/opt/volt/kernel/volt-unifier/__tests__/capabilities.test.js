'use strict';
const { describe, it } = require('node:test');
const assert = require('node:assert');

describe('CapabilityGuard', () => {
  it('allowed capability passes', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: ['memory-management', 'gpu-render', 'storage'],
      permissions: ['memory.*'],
    };
    assert.strictEqual(CapabilityGuard.requireCapability(caps, 'memory-management'), true);
    assert.strictEqual(CapabilityGuard.requireCapability(caps, 'gpu-render'), true);
  });

  it('missing capability throws CapabilityMissingError', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: ['memory-management'],
      permissions: [],
    };
    assert.throws(
      () => CapabilityGuard.requireCapability(caps, 'gpu-render'),
      /missing capability/
    );
  });

  it('null capabilities throws', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    assert.throws(
      () => CapabilityGuard.requireCapability(null, 'anything'),
      /missing capability/
    );
  });

  it('missing features array throws', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    assert.throws(
      () => CapabilityGuard.requireCapability({}, 'anything'),
      /missing capability/
    );
  });

  it('wildcard matching works', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: ['memory.*'],
      permissions: [],
    };
    assert.strictEqual(CapabilityGuard.requireCapability(caps, 'memory.allocate'), true);
    assert.strictEqual(CapabilityGuard.requireCapability(caps, 'memory.free'), true);
    assert.strictEqual(CapabilityGuard.requireCapability(caps, 'memory-management'), true);
  });

  it('wildcard only matches prefix', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: ['memory.*'],
      permissions: [],
    };
    assert.throws(
      () => CapabilityGuard.requireCapability(caps, 'storage.write'),
      /missing capability/
    );
  });

  it('sensitive commands detected', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('RAM_SET_QUOTA'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('USB_EJECT'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('USB_FLUSH'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('GPU_SWITCH_DEVICE'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('GPU_EVICT_RESOURCE'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('ASC_GENERATE'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('ASC_WRITE'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('SHUTDOWN_REQUEST'), true);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('SHUTDOWN_PREPARE'), true);
  });

  it('non-sensitive command is not detected', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('CPU_EXEC'), false);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('HEARTBEAT'), false);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('USB_READ'), false);
    assert.strictEqual(CapabilityGuard.isSensitiveCommand('GPU_RENDER'), false);
  });

  it('checkPermission requires capability for sensitive command', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: ['gpu-render'],
      permissions: [],
    };
    assert.throws(
      () => CapabilityGuard.checkPermission(caps, 'test', 'USB_EJECT'),
      /missing capability/
    );
  });

  it('checkPermission passes for non-sensitive command without capabilities', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: [],
      permissions: [],
    };
    assert.strictEqual(CapabilityGuard.checkPermission(caps, 'test', 'CPU_EXEC'), true);
  });

  it('checkPermission passes for sensitive command with matching capability', () => {
    const { CapabilityGuard } = require('../safety/capabilityGuard');
    const caps = {
      features: ['usb_eject'],
      permissions: [],
    };
    assert.strictEqual(CapabilityGuard.checkPermission(caps, 'test', 'USB_EJECT'), true);
  });
});
