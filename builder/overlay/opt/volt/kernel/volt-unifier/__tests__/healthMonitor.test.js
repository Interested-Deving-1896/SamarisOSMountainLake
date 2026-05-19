'use strict';
const { describe, it, before, after } = require('node:test');
const assert = require('node:assert');
const { EventEmitter } = require('node:events');

describe('HealthMonitor', () => {
  function makeMockUnifier(moduleEntries) {
    const modules = new Map();
    for (const e of moduleEntries) {
      modules.set(e.id, {
        id: e.id,
        config: e.config || {},
        status: e.status || 'offline',
        health: {
          status: e.status || 'offline',
          lastHeartbeatAt: e.lastHeartbeatAt || null,
          latencyMs: null,
          capabilities: [],
          errorCount: 0,
          reconnectCount: 0,
          lastError: e.lastError || null,
          degradedReason: e.degradedReason || null,
        },
        capabilities: null,
        client: null,
        connectedAt: null,
      });
    }
    return {
      registry: {
        getAll() { return Array.from(modules.values()); },
        get(id) { return modules.get(id) || null; },
        updateStatus(id, status, extra) {
          const entry = modules.get(id);
          if (!entry) return false;
          entry.status = status;
          entry.health.status = status;
          if (extra) {
            if (extra.degradedReason !== undefined) {
              entry.health.degradedReason = extra.degradedReason;
            }
            if (extra.lastError !== undefined) {
              entry.health.lastError = extra.lastError;
            }
          }
          return true;
        },
      },
      config: {
        modules: Object.fromEntries(
          moduleEntries.map(e => [e.id, e.config || { enabled: true, heartbeatMs: e.heartbeatMs || 5000 }])
        ),
      },
      eventBus: {
        publish() {},
      },
    };
  }

  it('heartbeat online keeps module healthy', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'online', lastHeartbeatAt: Date.now(), heartbeatMs: 5000 },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'online');
    assert.strictEqual(snapshot.warnings.length, 0);
    assert.strictEqual(snapshot.criticalIssues.length, 0);
    assert.strictEqual(snapshot.vrm.status, 'online');
  });

  it('heartbeat timeout marks degraded', () => {
    const unifier = makeMockUnifier([
      {
        id: 'vrm', status: 'online', lastHeartbeatAt: Date.now() - 60000,
        heartbeatMs: 1000,
        config: { enabled: true, heartbeatMs: 1000 },
      },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'degraded');
    assert.ok(snapshot.warnings.length > 0);
    assert.ok(snapshot.warnings[0].includes('vrm'));
    assert.ok(snapshot.warnings[0].includes('missed heartbeat'));
    assert.strictEqual(snapshot.vrm.status, 'degraded');
    assert.ok(snapshot.vrm.degradedReason.includes('no heartbeat'));
  });

  it('no heartbeat received yet adds warning', () => {
    const unifier = makeMockUnifier([
      { id: 'kernel-b', status: 'online', lastHeartbeatAt: null, heartbeatMs: 1000 },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'degraded');
    assert.ok(snapshot.warnings.some(w => w.includes('no heartbeat received')));
  });

  it('SystemHealthSnapshot returns all modules', () => {
    const unifier = makeMockUnifier([
      { id: 'kernel-b', status: 'online', lastHeartbeatAt: Date.now() },
      { id: 'vrm', status: 'online', lastHeartbeatAt: Date.now() },
      { id: 'vum', status: 'online', lastHeartbeatAt: Date.now() },
      { id: 'vgm', status: 'online', lastHeartbeatAt: Date.now() },
      { id: 'dwp', status: 'online', lastHeartbeatAt: Date.now() },
      { id: 'asc', status: 'online', lastHeartbeatAt: Date.now() },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.ok(snapshot.kernelB);
    assert.ok(snapshot.vrm);
    assert.ok(snapshot.vum);
    assert.ok(snapshot.vgm);
    assert.ok(snapshot.dwp);
    assert.ok(snapshot.asc);
    assert.strictEqual(typeof snapshot.overallStatus, 'string');
    assert.ok(Array.isArray(snapshot.warnings));
    assert.ok(Array.isArray(snapshot.criticalIssues));
  });

  it('overall status reflects warnings', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'online', lastHeartbeatAt: Date.now() - 30000, heartbeatMs: 1000 },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'degraded');
  });

  it('overall status is error when modules have error status', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'error', lastHeartbeatAt: Date.now(), lastError: 'kernel panic' },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'error');
    assert.ok(snapshot.criticalIssues.length > 0);
  });

  it('overall status is fatal when modules have fatal status', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'fatal', lastHeartbeatAt: Date.now(), lastError: 'fatal crash' },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'fatal');
  });

  it('offline modules generate warnings', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'offline', lastHeartbeatAt: null },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'degraded');
    assert.ok(snapshot.warnings.some(w => w.includes('offline')));
  });

  it('disabled modules are skipped', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'offline', lastHeartbeatAt: null, config: { enabled: false } },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'online');
  });

  it('getSystemHealthSnapshot returns fresh copy', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'online', lastHeartbeatAt: Date.now() },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snap1 = monitor.getSystemHealthSnapshot();
    const snap2 = monitor.getSystemHealthSnapshot();
    assert.deepStrictEqual(snap1, snap2);
    snap1.overallStatus = 'modified';
    assert.strictEqual(snap2.overallStatus, 'online');
  });

  it('start and stop do not throw', () => {
    const unifier = makeMockUnifier([
      { id: 'vrm', status: 'online', lastHeartbeatAt: Date.now() },
    ]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    assert.doesNotThrow(() => monitor.start());
    assert.doesNotThrow(() => monitor.stop());
    assert.doesNotThrow(() => monitor.stop());
  });

  it('non-existent module in snapshot returns offline', () => {
    const unifier = makeMockUnifier([]);
    const { HealthMonitor } = require('../health/healthMonitor');
    const monitor = new HealthMonitor(unifier);

    monitor._runCheck();
    const snapshot = monitor.getSystemHealthSnapshot();
    assert.strictEqual(snapshot.vrm.status, 'offline');
    assert.strictEqual(snapshot.kernelB.status, 'offline');
    assert.strictEqual(snapshot.vum.status, 'offline');
  });
});
