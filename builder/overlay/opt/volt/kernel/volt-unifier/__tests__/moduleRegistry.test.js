'use strict';
const { describe, it } = require('node:test');
const assert = require('node:assert');
const { MODULE_STATUSES } = require('../constants');

function createMockUnifier() {
  return {};
}

describe('ModuleRegistry', () => {
  it('registers a module with default state', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    const entry = reg.register('vrm', { protocol: 'SBP_MEM' });

    assert.strictEqual(entry.id, 'vrm');
    assert.strictEqual(entry.status, 'offline');
    assert.strictEqual(entry.health.status, 'offline');
    assert.strictEqual(entry.health.lastHeartbeatAt, null);
    assert.strictEqual(entry.health.latencyMs, null);
    assert.strictEqual(entry.health.errorCount, 0);
    assert.strictEqual(entry.health.reconnectCount, 0);
    assert.strictEqual(entry.health.lastError, null);
    assert.strictEqual(entry.health.degradedReason, null);
    assert.strictEqual(entry.capabilities, null);
    assert.strictEqual(entry.client, null);
    assert.strictEqual(entry.connectedAt, null);
    assert.deepStrictEqual(entry.config, { protocol: 'SBP_MEM' });
  });

  it('rejects duplicate module registration', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    assert.throws(
      () => reg.register('vrm', {}),
      /already registered/
    );
  });

  it('requires non-empty moduleId', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    assert.throws(
      () => reg.register('', {}),
      /non-empty string/
    );
    assert.throws(
      () => reg.register(null, {}),
      /non-empty string/
    );
  });

  it('returns getOnline modules', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('kernel-b', {});
    reg.register('vrm', {});
    reg.register('vum', {});
    reg.register('vgm', {});

    reg.updateStatus('kernel-b', 'online');
    reg.updateStatus('vrm', 'online');
    reg.updateStatus('vum', 'degraded');

    const online = reg.getOnline();
    assert.strictEqual(online.length, 2);
    assert.ok(online.every(e => e.status === 'online'));
  });

  it('getByStatus validates status', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    const result = reg.getByStatus('nonexistent');
    assert.deepStrictEqual(result, []);
  });

  it('handles status transitions correctly', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    reg.updateStatus('vrm', 'online');
    assert.strictEqual(reg.get('vrm').status, 'online');
    assert.strictEqual(reg.get('vrm').health.status, 'online');

    reg.updateStatus('vrm', 'degraded', {
      degradedReason: 'high latency',
      latencyMs: 500,
    });
    assert.strictEqual(reg.get('vrm').status, 'degraded');
    assert.strictEqual(reg.get('vrm').health.degradedReason, 'high latency');
    assert.strictEqual(reg.get('vrm').health.latencyMs, 500);
  });

  it('transitions to degraded when degradedReason is set with non-degraded status', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    reg.updateStatus('vrm', 'online');
    reg.updateStatus('vrm', 'online', { degradedReason: 'something wrong' });
    assert.strictEqual(reg.get('vrm').status, 'degraded');
  });

  it('sets connectedAt when transitioning to online', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    assert.strictEqual(reg.get('vrm').connectedAt, null);
    reg.updateStatus('vrm', 'online');
    assert.ok(reg.get('vrm').connectedAt > 0);
  });

  it('updateHealth updates individual health fields', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});

    reg.updateHealth('vrm', {
      errorCount: 3,
      reconnectCount: 1,
      lastError: 'connection refused',
    });

    assert.strictEqual(reg.get('vrm').health.errorCount, 3);
    assert.strictEqual(reg.get('vrm').health.reconnectCount, 1);
    assert.strictEqual(reg.get('vrm').health.lastError, 'connection refused');
  });

  it('updateHealth sets status and syncs to entry', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});

    reg.updateHealth('vrm', { status: 'online' });
    assert.strictEqual(reg.get('vrm').status, 'online');
    assert.strictEqual(reg.get('vrm').health.status, 'online');
  });

  it('updateHealth ignores invalid status values', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    reg.updateHealth('vrm', { status: 'invalid_status' });
    assert.notStrictEqual(reg.get('vrm').health.status, 'invalid_status');
  });

  it('updateHealth returns false for non-existent module', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    assert.strictEqual(reg.updateHealth('nonexistent', { status: 'online' }), false);
  });

  it('updateStatus rejects invalid status', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    assert.throws(
      () => reg.updateStatus('vrm', 'invalid_status'),
      /Invalid module status/
    );
  });

  it('updateStatus returns false for non-existent module', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    assert.strictEqual(reg.updateStatus('nonexistent', 'online'), false);
  });

  it('counts registered modules', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    assert.strictEqual(reg.count(), 0);
    reg.register('kernel-b', {});
    assert.strictEqual(reg.count(), 1);
    reg.register('vrm', {});
    assert.strictEqual(reg.count(), 2);
  });

  it('has returns correct boolean', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    assert.strictEqual(reg.has('vrm'), true);
    assert.strictEqual(reg.has('nonexistent'), false);
  });

  it('get returns null for unregistered', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    assert.strictEqual(reg.get('nonexistent'), null);
  });

  it('getAll returns all entries', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('kernel-b', {});
    reg.register('vrm', {});
    const all = reg.getAll();
    assert.strictEqual(all.length, 2);
  });

  it('remove deletes a module', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    assert.strictEqual(reg.has('vrm'), true);
    assert.strictEqual(reg.remove('vrm'), true);
    assert.strictEqual(reg.has('vrm'), false);
    assert.strictEqual(reg.remove('vrm'), false);
  });

  it('updateStatus with extra capabilities updates entry', () => {
    const { ModuleRegistry } = require('../registry/moduleRegistry');
    const reg = new ModuleRegistry(createMockUnifier());
    reg.register('vrm', {});
    reg.updateStatus('vrm', 'online', {
      capabilities: ['memory-management', 'pressure-events'],
      client: { name: 'mock' },
    });
    assert.deepStrictEqual(reg.get('vrm').capabilities, ['memory-management', 'pressure-events']);
    assert.deepStrictEqual(reg.get('vrm').health.capabilities, ['memory-management', 'pressure-events']);
    assert.deepStrictEqual(reg.get('vrm').client, { name: 'mock' });
  });

  it('status enum covers all expected values', () => {
    const expected = ['offline', 'starting', 'connecting', 'online', 'degraded', 'recovering', 'error', 'fatal'];
    assert.deepStrictEqual([...MODULE_STATUSES], expected);
  });
});
