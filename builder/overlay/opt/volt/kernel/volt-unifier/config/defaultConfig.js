'use strict';

const SOCKET_DIR = process.env.VOLT_SOCKET_DIR || '/run/samaris';

const defaultConfig = {
  enabled: true,
  mode: 'local',
  debugHttpEnabled: true,
  debugHttpBind: '127.0.0.1',
  debugHttpPort: 9999,
  transport: {
    unixSocketEnabled: true,
    shmEnabled: true,
    websocketDebugEnabled: true,
    socketDir: SOCKET_DIR,
    shmPath: `${SOCKET_DIR}/sbp-shm`,
    shmSizeMb: 64,
  },
  modules: {
    'kernel-b': { enabled: true, socket: `${SOCKET_DIR}/volt-kernel-b.sock`, protocol: 'SBP_V5', heartbeatMs: 1000 },
    vrm: { enabled: true, service: 'volt-ram-manager.service', protocol: 'systemd-health', heartbeatMs: 1000 },
    vum: { enabled: true, service: 'volt-usb-manager.service', protocol: 'systemd-health', heartbeatMs: 1000 },
    vgm: { enabled: true, service: 'volt-gpu-manager.service', protocol: 'systemd-health', heartbeatMs: 1000 },
    dwp: { enabled: true, service: 'volt-worker-pool.service', mode: 'adapter_ready', protocol: 'systemd-health', heartbeatMs: 2000 },
    asc: { enabled: true, mode: 'cli', binary: 'volt-asc', generatedConfig: `${SOCKET_DIR}/adaptive.generated.toml` },
  },
  safety: {
    requireLocalhostDebug: true,
    requireCapabilities: true,
    denyPublicBind: true,
    shutdownRequiresCleanVum: true,
  },
};

module.exports = { defaultConfig };
