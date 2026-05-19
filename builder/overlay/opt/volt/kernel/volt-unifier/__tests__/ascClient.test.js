'use strict';
const { describe, it, before, after } = require('node:test');
const assert = require('node:assert');

describe('AscClient', () => {
  let origExecFile;
  let currentMock;

  before(() => {
    const cp = require('node:child_process');
    origExecFile = cp.execFile;
    delete require.cache[require.resolve('../clients/ascClient')];

    currentMock = (_binary, _args, _opts, _cb) => {
      throw new Error('no mock set');
    };
    cp.execFile = (...args) => currentMock(...args);
  });

  after(() => {
    const cp = require('node:child_process');
    cp.execFile = origExecFile;
  });

  function setupMock(fn) {
    currentMock = fn;
  }

  it('probe command format', async () => {
    let capturedArgs = null;
    setupMock((binary, args, opts, cb) => {
      capturedArgs = { binary, args };
      cb(null, 'probe ok', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    const result = await client.probe();

    assert.ok(capturedArgs);
    assert.deepStrictEqual(capturedArgs.args, ['probe']);
    assert.ok(result.ok);
    assert.strictEqual(result.stdout, 'probe ok');
  });

  it('generate command format', async () => {
    let capturedArgs = null;
    setupMock((binary, args, opts, cb) => {
      capturedArgs = { binary, args };
      cb(null, 'generated config', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    const result = await client.generate();

    assert.deepStrictEqual(capturedArgs.args, ['generate']);
    assert.ok(result.ok);
    assert.strictEqual(result.stdout, 'generated config');
  });

  it('explain command format', async () => {
    let capturedArgs = null;
    setupMock((binary, args, opts, cb) => {
      capturedArgs = { binary, args };
      cb(null, 'explanation', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    const result = await client.explain();

    assert.deepStrictEqual(capturedArgs.args, ['explain']);
    assert.ok(result.ok);
    assert.strictEqual(result.stdout, 'explanation');
  });

  it('generateAndWrite passes --write flag', async () => {
    let capturedArgs = null;
    setupMock((binary, args, opts, cb) => {
      capturedArgs = { binary, args };
      cb(null, 'written', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    const result = await client.generateAndWrite('/tmp/out.toml');

    assert.deepStrictEqual(capturedArgs.args, ['--write', '/tmp/out.toml', 'write']);
    assert.ok(result.ok);
  });

  it('generateAndWrite handles empty path', async () => {
    let capturedArgs = null;
    setupMock((binary, args, opts, cb) => {
      capturedArgs = { binary, args };
      cb(null, '', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    await client.generateAndWrite('');

    assert.deepStrictEqual(capturedArgs.args, ['write']);
  });

  it('check command works', async () => {
    let capturedArgs = null;
    setupMock((binary, args, opts, cb) => {
      capturedArgs = { binary, args };
      cb(null, 'all checks passed', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    const result = await client.check();

    assert.deepStrictEqual(capturedArgs.args, ['check']);
    assert.ok(result.ok);
  });

  it('failure marks degraded', async () => {
    setupMock((binary, args, opts, cb) => {
      cb(new Error('binary not found'), '', 'stderr output');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    client.on('error', () => {});
    const result = await client.probe();

    assert.strictEqual(result.ok, false);
    assert.strictEqual(result.error, 'binary not found');
    assert.strictEqual(client.status, 'degraded');
    assert.strictEqual(client.lastError, 'binary not found');
  });

  it('failure emits error event', async () => {
    let emitted = null;
    setupMock((binary, args, opts, cb) => {
      cb(new Error('command failed'), '', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    client.on('error', (e) => { emitted = e; });
    await client.probe();

    assert.ok(emitted);
    assert.strictEqual(emitted.message, 'command failed');
  });

  it('success emits result event', async () => {
    let emitted = null;
    setupMock((binary, args, opts, cb) => {
      cb(null, 'output', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    client.on('result', (r) => { emitted = r; });
    await client.probe();

    assert.ok(emitted);
    assert.strictEqual(emitted.stdout, 'output');
    assert.strictEqual(client.status, 'online');
  });

  it('binary path uses x86_64 on x64', () => {
    const origArch = process.arch;
    Object.defineProperty(process, 'arch', {
      value: 'x64',
      configurable: true,
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    assert.ok(client._binary.includes('x86_64'));

    Object.defineProperty(process, 'arch', {
      value: origArch,
      configurable: true,
    });
  });

  it('binary path uses aarch64 on arm64', () => {
    const origArch = process.arch;
    Object.defineProperty(process, 'arch', {
      value: 'arm64',
      configurable: true,
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    assert.ok(client._binary.includes('aarch64'));

    Object.defineProperty(process, 'arch', {
      value: origArch,
      configurable: true,
    });
  });

  it('uses /opt/volt/bin prefix', () => {
    const origArch = process.arch;
    Object.defineProperty(process, 'arch', {
      value: 'x64',
      configurable: true,
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    assert.ok(client._binary.startsWith('/opt/volt/bin/volt-asc-'));

    Object.defineProperty(process, 'arch', {
      value: origArch,
      configurable: true,
    });
  });

  it('execFile uses 15s timeout', async () => {
    let capturedOpts = null;
    setupMock((binary, args, opts, cb) => {
      capturedOpts = opts;
      cb(null, '', '');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    await client.probe();

    assert.ok(capturedOpts);
    assert.strictEqual(capturedOpts.timeout, 15000);
  });

  it('stderr is captured on failure', async () => {
    setupMock((binary, args, opts, cb) => {
      cb(new Error('fail'), 'stdout-data', 'stderr-data');
    });

    const { AscClient } = require('../clients/ascClient');
    const client = new AscClient({});
    client.on('error', () => {});
    const result = await client.probe();

    assert.strictEqual(result.ok, false);
    assert.strictEqual(result.stdout, 'stdout-data');
    assert.strictEqual(result.stderr, 'stderr-data');
  });
});
