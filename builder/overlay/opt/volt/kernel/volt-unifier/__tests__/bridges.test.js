'use strict';
const { describe, it } = require('node:test');
const assert = require('node:assert');

function makeMockEventBus() {
  const subs = new Map();
  const published = [];
  return {
    subs,
    published,
    subscribe(type, handler) {
      if (!subs.has(type)) subs.set(type, []);
      const id = `sub-${Math.random().toString(36).slice(2, 8)}`;
      subs.get(type).push({ id, handler });
      return id;
    },
    unsubscribe(id) {
      for (const [, handlers] of subs) {
        const idx = handlers.findIndex(h => h.id === id);
        if (idx >= 0) { handlers.splice(idx, 1); return true; }
      }
      return false;
    },
    publish(ev) {
      published.push(ev);
      const handlers = subs.get(ev.type) || [];
      for (const h of handlers) {
        try { h.handler(ev); } catch (e) { /* ignore handler errors */ }
      }
    },
  };
}

function makeMockUnifier(overrides) {
  const eventBus = makeMockEventBus();
  const modules = new Map();

  const reg = {
    get(id) { return modules.get(id) || null; },
    getAll() { return Array.from(modules.values()); },
  };

  const mock = {
    eventBus,
    registry: reg,
    config: {
      modules: {},
    },
    ...overrides,
  };

  mock.addModule = (id, entry) => {
    modules.set(id, entry);
    mock.config.modules[id] = entry.config || { enabled: true };
  };

  return mock;
}

describe('DesktopBridge', () => {
  it('onFramePressure calls DWP setDesktopPressure', async () => {
    let calledPressure = null;
    const unifier = makeMockUnifier();
    unifier.addModule('dwp', {
      id: 'dwp', status: 'online',
      client: { setDesktopPressure(p) { calledPressure = p; return Promise.resolve(); } },
      health: {},
    });

    const { DesktopBridge } = require('../bridges/desktopBridge');
    const bridge = new DesktopBridge(unifier);
    bridge.start();

    await bridge.onFramePressure(0.75);
    assert.strictEqual(calledPressure, 0.75);
  });

  it('onFramePressure publishes GPU_FRAME_PRESSURE event', async () => {
    const unifier = makeMockUnifier();
    unifier.addModule('dwp', {
      id: 'dwp', status: 'online',
      client: { setDesktopPressure() {} },
      health: {},
    });

    const { DesktopBridge } = require('../bridges/desktopBridge');
    const bridge = new DesktopBridge(unifier);
    bridge.start();

    await bridge.onFramePressure(0.5);
    const event = unifier.eventBus.published.find(e => e.type === 'GPU_FRAME_PRESSURE');
    assert.ok(event);
    assert.strictEqual(event.payload.pressure, 0.5);
    assert.strictEqual(event.source, 'desktop-bridge');
  });

  it('getHealthSnapshot calls healthMonitor', () => {
    let called = false;
    const unifier = makeMockUnifier({
      healthMonitor: {
        getSystemHealthSnapshot() { called = true; return { overallStatus: 'online' }; },
      },
    });

    const { DesktopBridge } = require('../bridges/desktopBridge');
    const bridge = new DesktopBridge(unifier);
    const snap = bridge.getHealthSnapshot();
    assert.ok(called);
    assert.strictEqual(snap.overallStatus, 'online');
  });

  it('getHealthSnapshot returns null without healthMonitor', () => {
    const unifier = makeMockUnifier();
    const { DesktopBridge } = require('../bridges/desktopBridge');
    const bridge = new DesktopBridge(unifier);
    assert.strictEqual(bridge.getHealthSnapshot(), null);
  });

  it('start subscribes to MODULE_HEALTH and BOOT_READY', () => {
    const unifier = makeMockUnifier();
    const { DesktopBridge } = require('../bridges/desktopBridge');
    const bridge = new DesktopBridge(unifier);
    bridge.start();

    assert.ok(unifier.eventBus.subs.has('MODULE_HEALTH'));
    assert.ok(unifier.eventBus.subs.has('BOOT_READY'));
  });

  it('destroy unsubscribes all', () => {
    const unifier = makeMockUnifier();
    const { DesktopBridge } = require('../bridges/desktopBridge');
    const bridge = new DesktopBridge(unifier);
    bridge.start();

    const idsBefore = bridge._subscriptionIds.length;
    assert.ok(idsBefore > 0);

    bridge.destroy();
    assert.strictEqual(bridge._subscriptionIds.length, 0);
  });
});

describe('FinderBridge', () => {
  it('readFile uses VUM client read', async () => {
    let calledPath = null;
    const unifier = makeMockUnifier();
    unifier.addModule('vum', {
      id: 'vum', status: 'online',
      client: { read(p) { calledPath = p; return Promise.resolve(Buffer.from('data')); } },
      health: {},
    });

    const { FinderBridge } = require('../bridges/finderBridge');
    const bridge = new FinderBridge(unifier);
    const result = await bridge.readFile('/test/path');
    assert.strictEqual(calledPath, '/test/path');
    assert.ok(result);
  });

  it('writeFile uses VUM client write', async () => {
    let calledArgs = null;
    const unifier = makeMockUnifier();
    unifier.addModule('vum', {
      id: 'vum', status: 'online',
      client: { write(p, d) { calledArgs = { p, d }; return Promise.resolve(); } },
      health: {},
    });

    const { FinderBridge } = require('../bridges/finderBridge');
    const bridge = new FinderBridge(unifier);
    await bridge.writeFile('/test/path', Buffer.from('content'));
    assert.strictEqual(calledArgs.p, '/test/path');
    assert.deepStrictEqual(calledArgs.d, Buffer.from('content'));
  });

  it('eject uses VUM client eject', async () => {
    let calledDevice = null;
    const unifier = makeMockUnifier();
    unifier.addModule('vum', {
      id: 'vum', status: 'online',
      client: { eject(d) { calledDevice = d; return Promise.resolve(); } },
      health: {},
    });

    const { FinderBridge } = require('../bridges/finderBridge');
    const bridge = new FinderBridge(unifier);
    await bridge.eject('/dev/sda1');
    assert.strictEqual(calledDevice, '/dev/sda1');
  });

  it('readFile throws without VUM client', async () => {
    const unifier = makeMockUnifier();
    const { FinderBridge } = require('../bridges/finderBridge');
    const bridge = new FinderBridge(unifier);
    await assert.rejects(
      () => bridge.readFile('/test'),
      /VUM client not available/
    );
  });

  it('getStatus returns module status', () => {
    const unifier = makeMockUnifier();
    unifier.addModule('vum', {
      id: 'vum', status: 'degraded',
      client: { status: 'degraded', degradedReason: 'timeout', lastError: 'err' },
      health: { degradedReason: 'timeout', lastError: 'err' },
    });

    const { FinderBridge } = require('../bridges/finderBridge');
    const bridge = new FinderBridge(unifier);
    const status = bridge.getStatus();
    assert.strictEqual(status.moduleId, 'vum');
    assert.strictEqual(status.status, 'degraded');
  });

  it('getStatus returns offline when VUM not registered', () => {
    const unifier = makeMockUnifier();
    const { FinderBridge } = require('../bridges/finderBridge');
    const bridge = new FinderBridge(unifier);
    const status = bridge.getStatus();
    assert.strictEqual(status.status, 'offline');
  });
});

describe('SettingsBridge', () => {
  it('getModuleStatuses reads all registry entries', () => {
    const unifier = makeMockUnifier();
    unifier.addModule('kernel-b', {
      id: 'kernel-b', status: 'online',
      config: { enabled: true },
      health: { degradedReason: null, lastError: null, errorCount: 0, reconnectCount: 0, lastHeartbeatAt: 1000 },
    });
    unifier.addModule('vrm', {
      id: 'vrm', status: 'degraded',
      config: { enabled: true },
      health: { degradedReason: 'oom', lastError: null, errorCount: 2, reconnectCount: 1, lastHeartbeatAt: 2000 },
    });

    const { SettingsBridge } = require('../bridges/settingsBridge');
    const bridge = new SettingsBridge(unifier);
    const statuses = bridge.getModuleStatuses();

    assert.ok(statuses['kernel-b']);
    assert.strictEqual(statuses['kernel-b'].status, 'online');
    assert.strictEqual(statuses['kernel-b'].enabled, true);

    assert.ok(statuses.vrm);
    assert.strictEqual(statuses.vrm.status, 'degraded');
    assert.strictEqual(statuses.vrm.degradedReason, 'oom');
    assert.strictEqual(statuses.vrm.errorCount, 2);
  });

  it('getAscExplain returns explain result', async () => {
    const unifier = makeMockUnifier();
    unifier.addModule('asc', {
      id: 'asc', status: 'online',
      client: { explain() { return Promise.resolve({ ok: true, stdout: 'explain data' }); } },
      health: {},
    });

    const { SettingsBridge } = require('../bridges/settingsBridge');
    const bridge = new SettingsBridge(unifier);
    const result = await bridge.getAscExplain();
    assert.ok(result.ok);
    assert.strictEqual(result.stdout, 'explain data');
  });

  it('getAscExplain throws without ASC client', async () => {
    const unifier = makeMockUnifier();
    const { SettingsBridge } = require('../bridges/settingsBridge');
    const bridge = new SettingsBridge(unifier);
    await assert.rejects(
      () => bridge.getAscExplain(),
      /ASC client not available/
    );
  });

  it('getDwpMetrics returns metrics', async () => {
    const unifier = makeMockUnifier();
    unifier.addModule('dwp', {
      id: 'dwp', status: 'online',
      client: { metrics() { return Promise.resolve({ mode: 'local_fallback', jobsSubmitted: 5 }); } },
      health: {},
    });

    const { SettingsBridge } = require('../bridges/settingsBridge');
    const bridge = new SettingsBridge(unifier);
    const m = await bridge.getDwpMetrics();
    assert.strictEqual(m.jobsSubmitted, 5);
  });

  it('getDwpMetrics throws without DWP client', async () => {
    const unifier = makeMockUnifier();
    const { SettingsBridge } = require('../bridges/settingsBridge');
    const bridge = new SettingsBridge(unifier);
    await assert.rejects(
      () => bridge.getDwpMetrics(),
      /DWP client not available/
    );
  });
});

describe('DevToolsBridge', () => {
  it('getDashboardSnapshot aggregates module metrics', () => {
    const unifier = makeMockUnifier();
    unifier.addModule('vrm', {
      id: 'vrm', status: 'online',
      client: { metricsSnapshot() { return {}; } },
      health: {},
    });
    unifier.addModule('vum', {
      id: 'vum', status: 'offline',
      client: null,
      health: {},
    });

    const { DevToolsBridge } = require('../bridges/devtoolsBridge');
    const bridge = new DevToolsBridge(unifier);
    bridge.start();

    const snapshot = bridge.getDashboardSnapshot();
    assert.ok(snapshot.modules.vrm);
    assert.strictEqual(snapshot.modules.vum.available, false);
    assert.strictEqual(snapshot.modules.vum.status, 'offline');
    assert.strictEqual(typeof snapshot.timestamp, 'number');
    assert.strictEqual(typeof snapshot.eventCount, 'number');
  });

  it('getHealthOverview returns health data for all modules', () => {
    const unifier = makeMockUnifier();
    unifier.addModule('kernel-b', {
      id: 'kernel-b', status: 'online',
      health: { status: 'online', lastHeartbeatAt: 100, latencyMs: 5, errorCount: 0, reconnectCount: 0, lastError: null, degradedReason: null },
    });
    unifier.addModule('vrm', {
      id: 'vrm', status: 'degraded',
      health: { status: 'degraded', lastHeartbeatAt: 200, latencyMs: 50, errorCount: 3, reconnectCount: 1, lastError: 'timeout', degradedReason: 'high latency' },
    });

    const { DevToolsBridge } = require('../bridges/devtoolsBridge');
    const bridge = new DevToolsBridge(unifier);
    const overview = bridge.getHealthOverview();

    assert.ok(overview['kernel-b']);
    assert.strictEqual(overview['kernel-b'].status, 'online');
    assert.strictEqual(overview.vrm.status, 'degraded');
    assert.strictEqual(overview.vrm.lastError, 'timeout');
  });

  it('getEventHistory returns bounded events', () => {
    const unifier = makeMockUnifier();
    const { DevToolsBridge } = require('../bridges/devtoolsBridge');
    const bridge = new DevToolsBridge(unifier);
    bridge.start();

    for (let i = 0; i < 5; i++) {
      unifier.eventBus.publish({
        id: `evt-${i}`, type: 'MODULE_HEALTH', source: 'test',
        severity: 'info', timestamp: i, payload: {},
      });
    }

    const history = bridge.getEventHistory(3);
    assert.strictEqual(history.length, 3);
  });

  it('getEventHistory returns all events when limit exceeds count', () => {
    const unifier = makeMockUnifier();
    const { DevToolsBridge } = require('../bridges/devtoolsBridge');
    const bridge = new DevToolsBridge(unifier);
    bridge.start();

    unifier.eventBus.publish({
      id: 'evt-1', type: 'MODULE_HEALTH', source: 'test',
      severity: 'info', timestamp: 1, payload: {},
    });

    const history = bridge.getEventHistory(100);
    assert.strictEqual(history.length, 1);
  });

  it('overall status reflects fatal modules', () => {
    const unifier = makeMockUnifier();
    unifier.addModule('vrm', { id: 'vrm', status: 'online', health: {} });
    unifier.addModule('vum', { id: 'vum', status: 'fatal', health: {} });

    const { DevToolsBridge } = require('../bridges/devtoolsBridge');
    const bridge = new DevToolsBridge(unifier);
    const snapshot = bridge.getDashboardSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'fatal');
  });

  it('overall status returns online when all modules are online', () => {
    const unifier = makeMockUnifier();
    unifier.addModule('kernel-b', { id: 'kernel-b', status: 'online', health: {} });

    const { DevToolsBridge } = require('../bridges/devtoolsBridge');
    const bridge = new DevToolsBridge(unifier);
    const snapshot = bridge.getDashboardSnapshot();
    assert.strictEqual(snapshot.overallStatus, 'online');
  });
});
