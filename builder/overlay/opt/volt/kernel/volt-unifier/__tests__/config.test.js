'use strict';
const { describe, it } = require('node:test');
const assert = require('node:assert');

describe('Config', () => {
  it('loads default config without error', () => {
    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.ok(config);
    assert.strictEqual(typeof config.enabled, 'boolean');
    assert.strictEqual(config.mode, 'local');
    assert.ok(config.transport);
    assert.ok(config.transport.unixSocketEnabled);
    assert.ok(config.modules);
    assert.ok(config.modules['kernel-b']);
    assert.ok(config.modules.vrm);
    assert.ok(config.modules.vum);
    assert.ok(config.modules.vgm);
    assert.ok(config.modules.dwp);
    assert.ok(config.modules.asc);
    assert.ok(config.safety);
    assert.strictEqual(config.safety.requireLocalhostDebug, true);
    assert.strictEqual(config.safety.denyPublicBind, true);
  });

  it('debugHttpBind defaults to 127.0.0.1', () => {
    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.strictEqual(config.debugHttpBind, '127.0.0.1');
  });

  it('debugHttpPort defaults to 9999', () => {
    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.strictEqual(config.debugHttpPort, 9999);
  });

  it('default config is not the frozen reference', () => {
    const { loadConfig } = require('../config/loader');
    const { defaultConfig } = require('../config/defaultConfig');
    const config = loadConfig();
    assert.notStrictEqual(config, defaultConfig);
    config.mode = 'remote';
    assert.strictEqual(defaultConfig.mode, 'local');
  });

  it('LocalOnlyGuard rejects 0.0.0.0 bind', () => {
    const { LocalOnlyGuard } = require('../safety/localOnlyGuard');
    assert.throws(
      () => LocalOnlyGuard.validateBindAddress('0.0.0.0'),
      { code: 'UNSAFE_DEBUG_BIND' }
    );
  });

  it('LocalOnlyGuard rejects arbitrary addresses', () => {
    const { LocalOnlyGuard } = require('../safety/localOnlyGuard');
    assert.throws(
      () => LocalOnlyGuard.validateBindAddress('192.168.1.1'),
      { code: 'UNSAFE_DEBUG_BIND' }
    );
    assert.throws(
      () => LocalOnlyGuard.validateBindAddress('10.0.0.1'),
      { code: 'UNSAFE_DEBUG_BIND' }
    );
  });

  it('LocalOnlyGuard accepts 127.0.0.1', () => {
    const { LocalOnlyGuard } = require('../safety/localOnlyGuard');
    assert.strictEqual(LocalOnlyGuard.validateBindAddress('127.0.0.1'), true);
  });

  it('LocalOnlyGuard accepts localhost', () => {
    const { LocalOnlyGuard } = require('../safety/localOnlyGuard');
    assert.strictEqual(LocalOnlyGuard.validateBindAddress('localhost'), true);
  });

  it('socket paths are valid', () => {
    const { SOCKET_PATHS } = require('../constants');
    for (const [id, p] of Object.entries(SOCKET_PATHS)) {
      assert.strictEqual(typeof p, 'string', `Path for ${id} is not a string`);
      assert.ok(p.startsWith('/'), `Path for ${id} does not start with /`);
      assert.ok(p.endsWith('.sock'), `Path for ${id} does not end with .sock`);
    }
    assert.ok(Object.keys(SOCKET_PATHS).length >= 4);
  });

  it('socketDir is set in transport config', () => {
    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.strictEqual(typeof config.transport.socketDir, 'string');
    assert.ok(config.transport.socketDir.startsWith('/'));
  });

  it('shmPath is set in transport config', () => {
    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.strictEqual(typeof config.transport.shmPath, 'string');
    assert.ok(config.transport.shmPath.startsWith('/'));
  });

  it('shmSizeMb is a positive integer', () => {
    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.strictEqual(typeof config.transport.shmSizeMb, 'number');
    assert.ok(config.transport.shmSizeMb > 0);
    assert.ok(Number.isInteger(config.transport.shmSizeMb));
  });

  it('defaultConfig has all required module entries', () => {
    const { defaultConfig } = require('../config/defaultConfig');
    const required = ['kernel-b', 'vrm', 'vum', 'vgm', 'dwp', 'asc'];
    for (const id of required) {
      assert.ok(defaultConfig.modules[id], `Missing module: ${id}`);
      assert.ok(defaultConfig.modules[id].enabled, `${id} should be enabled`);
    }
  });

  it('environment variables override config', () => {
    process.env.SAMARIS_UNIFIER_MODE = 'remote';
    process.env.SAMARIS_UNIFIER_DISABLED = 'true';
    process.env.SAMARIS_UNIFIER_DEBUG_PORT = '7777';
    process.env.SAMARIS_UNIFIER_DEBUG_BIND = '127.0.0.1';
    process.env.SAMARIS_SOCKET_DIR = '/custom/sockets';
    process.env.SAMARIS_SHM_PATH = '/custom/shm';
    process.env.SAMARIS_SHM_SIZE_MB = '128';

    const { loadConfig } = require('../config/loader');
    const config = loadConfig();
    assert.strictEqual(config.mode, 'remote');
    assert.strictEqual(config.enabled, false);
    assert.strictEqual(config.debugHttpPort, 7777);
    assert.strictEqual(config.debugHttpBind, '127.0.0.1');
    assert.strictEqual(config.transport.socketDir, '/custom/sockets');
    assert.strictEqual(config.transport.shmPath, '/custom/shm');
    assert.strictEqual(config.transport.shmSizeMb, 128);

    delete process.env.SAMARIS_UNIFIER_MODE;
    delete process.env.SAMARIS_UNIFIER_DISABLED;
    delete process.env.SAMARIS_UNIFIER_DEBUG_PORT;
    delete process.env.SAMARIS_UNIFIER_DEBUG_BIND;
    delete process.env.SAMARIS_SOCKET_DIR;
    delete process.env.SAMARIS_SHM_PATH;
    delete process.env.SAMARIS_SHM_SIZE_MB;
  });

  it('isPublicBind detects 0.0.0.0 and ::', () => {
    const { LocalOnlyGuard } = require('../safety/localOnlyGuard');
    assert.strictEqual(LocalOnlyGuard.isPublicBind('0.0.0.0'), true);
    assert.strictEqual(LocalOnlyGuard.isPublicBind('::'), true);
    assert.strictEqual(LocalOnlyGuard.isPublicBind('127.0.0.1'), false);
  });
});
