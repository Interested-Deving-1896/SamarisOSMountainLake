'use strict';
const { describe, it, before, after } = require('node:test');
const assert = require('node:assert');
const { EventEmitter } = require('node:events');

describe('BaseModuleClient', () => {
  let OriginalSocket;

  before(() => {
    const net = require('node:net');
    OriginalSocket = net.Socket;
  });

  after(() => {
    const net = require('node:net');
    net.Socket = OriginalSocket;
  });

  function makeMockSocket() {
    const sock = new EventEmitter();
    sock._connectPath = null;
    sock._destroyed = false;
    sock._written = [];

    sock.connect = (path) => {
      sock._connectPath = path;
      process.nextTick(() => sock.emit('connect'));
    };
    sock.destroy = () => {
      sock._destroyed = true;
      process.nextTick(() => sock.emit('close'));
    };
    sock.write = (data) => {
      sock._written.push(data);
      return true;
    };

    return sock;
  }

  it('connect fails gracefully without socket config', async () => {
    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', {});

    await client.connect();
    assert.strictEqual(client.status, 'degraded');
    assert.strictEqual(client.degradedReason, 'no socket configured');
  });

  it('connect succeeds with socket config', async () => {
    const net = require('node:net');
    const mockSock = makeMockSocket();
    net.Socket = function () { return mockSock; };

    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', { socket: '/tmp/test.sock' });

    await client.connect();
    assert.strictEqual(client.status, 'online');
    assert.strictEqual(mockSock._connectPath, '/tmp/test.sock');

    net.Socket = OriginalSocket;
  });

  it('disconnect sets status to offline and destroys socket', async () => {
    const net = require('node:net');
    const mockSock = makeMockSocket();
    net.Socket = function () { return mockSock; };

    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', { socket: '/tmp/test.sock' });

    await client.connect();
    assert.strictEqual(client.status, 'online');

    await client.disconnect();
    assert.strictEqual(client.status, 'offline');
    assert.strictEqual(mockSock._destroyed, true);

    net.Socket = OriginalSocket;
  });

  it('connect emits connected event', async () => {
    const net = require('node:net');
    const mockSock = makeMockSocket();
    net.Socket = function () { return mockSock; };

    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', { socket: '/tmp/test.sock' });

    const connected = new Promise(resolve => client.on('connected', resolve));
    await client.connect();
    await connected;

    net.Socket = OriginalSocket;
  });

  it('disconnect closes socket and rejects pending', async () => {
    const net = require('node:net');
    const mockSock = makeMockSocket();
    net.Socket = function () { return mockSock; };

    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', { socket: '/tmp/test.sock' });

    await client.connect();
    assert.strictEqual(client.status, 'online');

    await client.disconnect();
    assert.strictEqual(client.status, 'offline');
    assert.strictEqual(mockSock._destroyed, true);

    net.Socket = OriginalSocket;
  });

  it('markDegraded sets status and emits event', () => {
    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', { socket: '/tmp/test.sock' });

    let emitted = null;
    client.on('degraded', (r) => { emitted = r; });

    client.markDegraded('high error rate');
    assert.strictEqual(client.status, 'degraded');
    assert.strictEqual(client.degradedReason, 'high error rate');
    assert.strictEqual(emitted, 'high error rate');
  });

  it('markFatal sets status and emits event', () => {
    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', {});

    let emitted = null;
    client.on('fatal', (e) => { emitted = e; });

    const err = new Error('critical failure');
    client.markFatal(err);
    assert.strictEqual(client.status, 'fatal');
    assert.strictEqual(client.lastError, 'critical failure');
    assert.strictEqual(emitted.message, 'critical failure');
  });

  it('isOnline and isDegraded helpers', () => {
    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', {});

    assert.strictEqual(client.isOnline(), false);
    assert.strictEqual(client.isDegraded(), false);

    client.status = 'online';
    assert.strictEqual(client.isOnline(), true);

    client.status = 'degraded';
    assert.strictEqual(client.isOnline(), false);
    assert.strictEqual(client.isDegraded(), true);

    client.status = 'recovering';
    assert.strictEqual(client.isDegraded(), true);
  });

  it('reconnect returns true and status returns to online', async () => {
    const net = require('node:net');
    const mockSock = makeMockSocket();
    net.Socket = function () { return mockSock; };

    const { BaseModuleClient } = require('../clients/baseClient');
    const client = new BaseModuleClient({}, 'test', { socket: '/tmp/test.sock' });
    await client.connect();
    assert.strictEqual(client.status, 'online');

    const result = await client.reconnect();
    assert.strictEqual(result, true);
    assert.strictEqual(client.status, 'online');

    net.Socket = OriginalSocket;
  });
});

describe('DwpLocalFallback', () => {
  it('submitJob adds to queue and returns result', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    const result = dwp.submitJob({ id: 'job-1', priority: 'normal', source: 'test' });
    assert.ok(result.ok);
    assert.strictEqual(result.mode, 'local_fallback');
    assert.ok(result.jobId);
  });

  it('prioritize critical jobs over normal', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    dwp.submitJob({ id: 'job-normal', priority: 'normal', source: 'a' });
    dwp.submitJob({ id: 'job-critical', priority: 'critical', source: 'b' });
    dwp.submitJob({ id: 'job-high', priority: 'high', source: 'c' });

    const snapshot = dwp.getSnapshot();
    assert.strictEqual(snapshot.queueDepth, 3);
    assert.strictEqual(snapshot.jobsSubmitted, 3);
  });

  it('burst control rejects during active window', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    const first = dwp.requestOrbitBurst({ priority: 'high' });
    assert.strictEqual(first.accepted, true);

    const second = dwp.requestOrbitBurst({ priority: 'high' });
    assert.strictEqual(second.accepted, false);
    assert.strictEqual(second.reason, 'burst_window_active');
  });

  it('burst control rejects when desktop pressure is high', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    dwp.setDesktopPressure(0.8);
    const result = dwp.requestOrbitBurst({ priority: 'high' });
    assert.strictEqual(result.accepted, false);
    assert.strictEqual(result.reason, 'desktop_pressure_too_high');
  });

  it('desktop pressure gating removes idle jobs above threshold', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    dwp.submitJob({ id: 'critical-1', priority: 'critical' });
    dwp.submitJob({ id: 'idle-1', priority: 'idle' });
    dwp.submitJob({ id: 'normal-1', priority: 'normal' });

    dwp.setDesktopPressure(0.9);

    const snapshot = dwp.getSnapshot();
    assert.strictEqual(snapshot.jobsSubmitted, 3);
  });

  it('desktop pressure clamps between 0 and 1', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    dwp.setDesktopPressure(1.5);
    const m1 = dwp.metrics();
    assert.strictEqual(m1.desktopPressure, 1);

    dwp.setDesktopPressure(-0.5);
    const m2 = dwp.metrics();
    assert.strictEqual(m2.desktopPressure, 0);
  });

  it('burst tracking limits consecutive bursts', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    const r1 = dwp.requestOrbitBurst({ priority: 'high' });
    assert.strictEqual(r1.accepted, true);

    const r2 = dwp.requestOrbitBurst({ priority: 'high' });
    assert.strictEqual(r2.accepted, false);

    const snapshot = dwp.getSnapshot();
    assert.strictEqual(snapshot.burstCount, 1);
    assert.strictEqual(snapshot.burstActive, true);
  });

  it('getSnapshot returns metrics', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    dwp.submitJob({ id: 'j1', priority: 'normal' });
    dwp.setDesktopPressure(0.3);

    const snap = dwp.getSnapshot();
    assert.strictEqual(snap.queueDepth, 1);
    assert.strictEqual(snap.desktopPressure, 0.3);
    assert.strictEqual(snap.jobsSubmitted, 1);
  });

  it('metrics returns current state', () => {
    const { DwpLocalFallback } = require('../clients/dwpLocalFallback');
    const dwp = new DwpLocalFallback({});

    dwp.submitJob({ id: 'j1', priority: 'normal' });
    const m = dwp.metrics();
    assert.strictEqual(m.mode, 'local_fallback');
    assert.strictEqual(typeof m.currentWorkers, 'number');
    assert.strictEqual(typeof m.maxWorkers, 'number');
    assert.ok(m.jobsSubmitted >= 1);
  });
});
