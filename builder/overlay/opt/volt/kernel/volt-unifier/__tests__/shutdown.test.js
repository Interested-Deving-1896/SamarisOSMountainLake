'use strict';
const { describe, it, before, after } = require('node:test');
const assert = require('node:assert');

describe('ShutdownOrchestrator', () => {
  function makeMockClient(isOnline, flushImpl) {
    return {
      _online: isOnline,
      isOnline() { return this._online; },
      flush: flushImpl || (() => Promise.resolve()),
      durabilityStatus: () => Promise.resolve({ clean: true }),
    };
  }

  function makeMockUnifier(opts = {}) {
    const events = [];
    const modules = new Map();

    if (opts.vumOnline !== false) {
      modules.set('vum', {
        id: 'vum',
        status: 'online',
        client: makeMockClient(true, opts.vumFlushImpl),
      });
    }
    if (opts.vrmOnline !== false) {
      modules.set('vrm', {
        id: 'vrm',
        status: 'online',
        client: makeMockClient(true),
      });
    }
    if (opts.vgmOnline !== false) {
      modules.set('vgm', {
        id: 'vgm',
        status: 'online',
        client: makeMockClient(true),
      });
    }
    if (opts.dwpOnline !== false) {
      modules.set('dwp', {
        id: 'dwp',
        status: 'online',
        client: { _mode: 'adapter_ready' },
      });
    }

    return {
      config: {
        safety: {
          shutdownRequiresCleanVum: opts.shutdownRequiresCleanVum !== false,
        },
      },
      registry: {
        get(id) { return modules.get(id) || null; },
      },
      eventBus: {
        publish(ev) { events.push(ev); },
        events,
      },
    };
  }

  it('clean shutdown succeeds with all modules online', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({});
    const orch = new ShutdownOrchestrator(unifier);

    const result = await orch.prepareShutdown();
    assert.ok(result.ok);
    assert.strictEqual(result.blocked, false);
    assert.ok(result.steps.length >= 3);
    assert.ok(result.steps.some(s => s.module === 'vum' && s.action === 'flush' && s.status === 'ok'));
    assert.ok(result.steps.some(s => s.module === 'vrm' && s.action === 'flush' && s.status === 'ok'));
    assert.ok(result.steps.some(s => s.module === 'vgm' && s.action === 'drain' && s.status === 'ok'));
  });

  it('shutdown blocked when VUM flush fails with clean required', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({
      shutdownRequiresCleanVum: true,
      vumFlushImpl: () => Promise.reject(new Error('VUM flush timeout')),
    });
    const orch = new ShutdownOrchestrator(unifier);

    const result = await orch.prepareShutdown();
    assert.strictEqual(result.ok, false);
    assert.strictEqual(result.blocked, true);
    assert.ok(result.reason.includes('VUM flush failed'));
  });

  it('shutdown blocked publishes SHUTDOWN_BLOCKED event', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({
      shutdownRequiresCleanVum: true,
      vumFlushImpl: () => Promise.reject(new Error('VUM flush timeout')),
    });
    const orch = new ShutdownOrchestrator(unifier);

    await orch.prepareShutdown();
    const shutdownBlocked = unifier.eventBus.events.find(e => e.type === 'SHUTDOWN_BLOCKED');
    assert.ok(shutdownBlocked);
    assert.ok(shutdownBlocked.payload.reason.includes('VUM flush failed'));
  });

  it('shutdown request event is published', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({});
    const orch = new ShutdownOrchestrator(unifier);

    await orch.prepareShutdown();
    const shutdownReq = unifier.eventBus.events.find(e => e.type === 'SHUTDOWN_REQUEST');
    assert.ok(shutdownReq);
    assert.strictEqual(shutdownReq.source, 'shutdown-orchestrator');
  });

  it('steps are recorded correctly', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({});
    const orch = new ShutdownOrchestrator(unifier);

    const result = await orch.prepareShutdown();
    assert.ok(Array.isArray(result.steps));
    assert.ok(result.steps.length > 0);

    for (const step of result.steps) {
      assert.ok(step.module);
      assert.ok(step.action);
      assert.ok(step.status);
    }
  });

  it('prepareShutdown throws if already shutting down', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({
      shutdownRequiresCleanVum: true,
      vumFlushImpl: () => new Promise(() => {}),
    });
    const orch = new ShutdownOrchestrator(unifier);

    const p1 = orch.prepareShutdown().catch(() => {});
    await assert.rejects(
      () => orch.prepareShutdown(),
      /Already shutting down/
    );
  });

  it('checkCanShutdown returns canShutdown:true with no VUM', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({ vumOnline: false });
    const orch = new ShutdownOrchestrator(unifier);

    const check = await orch.checkCanShutdown();
    assert.strictEqual(check.canShutdown, true);
  });

  it('checkCanShutdown returns canShutdown:true even with VUM online', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({ vumOnline: true });
    const orch = new ShutdownOrchestrator(unifier);

    const check = await orch.checkCanShutdown();
    assert.strictEqual(check.canShutdown, true);
  });

  it('VUM flush is called during shutdown', async () => {
    let flushCalled = false;
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({
      vumFlushImpl: () => {
        flushCalled = true;
        return Promise.resolve();
      },
    });
    const orch = new ShutdownOrchestrator(unifier);

    await orch.prepareShutdown();
    assert.strictEqual(flushCalled, true);
  });

  it('VUM flush error recorded in step when no clean required', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({
      shutdownRequiresCleanVum: false,
      vumFlushImpl: () => Promise.reject(new Error('timeout')),
    });
    const orch = new ShutdownOrchestrator(unifier);

    const result = await orch.prepareShutdown();
    assert.ok(result.ok);
    assert.strictEqual(result.blocked, false);
    const vumStep = result.steps.find(s => s.module === 'vum');
    assert.ok(vumStep);
    assert.strictEqual(vumStep.status, 'error');
  });

  it('DWP drain step includes mode', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = makeMockUnifier({});
    const orch = new ShutdownOrchestrator(unifier);

    const result = await orch.prepareShutdown();
    const dwpStep = result.steps.find(s => s.module === 'dwp');
    assert.ok(dwpStep);
    assert.strictEqual(dwpStep.mode, 'adapter_ready');
  });

  it('shutdown without any modules still succeeds', async () => {
    const { ShutdownOrchestrator } = require('../lifecycle/shutdown');
    const unifier = {
      config: { safety: { shutdownRequiresCleanVum: false } },
      registry: { get: () => null },
      eventBus: { publish() {} },
    };
    const orch = new ShutdownOrchestrator(unifier);

    const result = await orch.prepareShutdown();
    assert.ok(result.ok);
    assert.strictEqual(result.steps.length, 0);
  });
});
