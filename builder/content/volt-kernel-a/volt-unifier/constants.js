'use strict';

const SBP_MAGIC = 0x53425005;
const SBP_VERSION = 0x05;

const SBP_FLAGS = {
  REQUEST: 0x01,
  RESPONSE: 0x02,
  ERROR: 0x04,
  EVENT: 0x08,
  ACK_BUFFERED: 0x10,
  ACK_DURABLE: 0x20,
};

const SBP_MAX_PAYLOAD = 65536;

const MODULE_IDS = Object.freeze([
  'kernel-b',
  'vrm',
  'vum',
  'vgm',
  'dwp',
  'asc',
  'volt-boot',
]);

const MODULE_PROTOCOLS = Object.freeze({
  'kernel-b': 'SBP_V5',
  vrm: 'SBP_MEM',
  vum: 'SBP_USB',
  vgm: 'SBP_GPU',
  dwp: 'SBP_V5',
  asc: 'CLI',
  'volt-boot': 'SBP_V5',
});

const MODULE_STATUSES = Object.freeze([
  'offline',
  'starting',
  'connecting',
  'online',
  'degraded',
  'recovering',
  'error',
  'fatal',
]);

const SOCKET_PATHS = Object.freeze({
  'kernel-b': '/run/samaris/volt-kernel-b.sock',
  vrm: '/run/samaris/volt-ram-manager.sock',
  vum: '/run/samaris/volt-usb-manager.sock',
  vgm: '/run/samaris/volt-gpu-manager.sock',
  dwp: '/run/samaris/volt-worker-pool.sock',
});

const HEARTBEAT_DEFAULTS = Object.freeze({
  'kernel-b': 1000,
  vrm: 1000,
  vum: 1000,
  vgm: 1000,
  dwp: 2000,
  asc: 5000,
});

const HEALTH_TIMEOUT_MULTIPLIER = 3;

const RECONNECT_BACKOFF = Object.freeze({
  initialMs: 500,
  maxMs: 30000,
  factor: 2,
  jitter: 0.1,
  maxAttempts: 10,
});

const SHUTDOWN_TIMEOUTS = Object.freeze({
  vumFlushMs: 5000,
  vrmFlushMs: 3000,
  vgmDrainMs: 3000,
  dwpDrainMs: 2000,
  kernelBPrepareMs: 3000,
  totalMs: 15000,
});

const OPOCODES = Object.freeze({
  // Kernel B
  GPU_RENDER: 0x01,
  CPU_EXEC: 0x05,
  QUERY_CORES: 0x0A,
  QUERY_GPU: 0x0B,
  HEARTBEAT: 0x0C,
  THERMAL_STATUS: 0x0F,
  // VRM
  RAM_STATUS: 0x15,
  RAM_FLUSH: 0x16,
  RAM_GC_SIGNAL: 0x17,
  RAM_REGISTER_APP: 0x18,
  RAM_UNREGISTER_APP: 0x19,
  RAM_SET_QUOTA: 0x1A,
  RAM_APP_STATUS: 0x1B,
  RAM_PRESSURE_EVENT: 0x1C,
  RAM_COMPRESS_APP: 0x1D,
  RAM_RELEASE_CACHE: 0x1E,
  RAM_HEARTBEAT: 0x1F,
  RAM_SUBSCRIBE_EVENTS: 0x20,
  RAM_UNSUBSCRIBE_EVENTS: 0x21,
  RAM_POLICY_UPDATE: 0x22,
  RAM_SNAPSHOT: 0x23,
  // VUM
  USB_STATUS: 0x30,
  USB_READ: 0x31,
  USB_WRITE: 0x32,
  USB_FLUSH: 0x33,
  USB_CACHE_STATUS: 0x34,
  USB_PREFETCH: 0x35,
  USB_EJECT: 0x36,
  USB_HEARTBEAT: 0x37,
  USB_MOUNT: 0x38,
  USB_UNMOUNT: 0x39,
  USB_JOURNAL_STATUS: 0x3A,
  USB_RECOVERY_RUN: 0x3B,
  USB_DURABILITY_STATUS: 0x3C,
  USB_WRITE_ACK_EVENT: 0x3D,
  USB_DEVICE_EVENT: 0x3E,
  USB_METRICS_SNAPSHOT: 0x3F,
  // VGM
  GPU_STATUS: 0x40,
  GPU_ALLOC_RESOURCE: 0x41,
  GPU_FREE_RESOURCE: 0x42,
  GPU_EXEC_COMPUTE: 0x43,
  GPU_RENDER_FRAME: 0x44,
  GPU_THERMAL_STATUS: 0x45,
  GPU_SWITCH_DEVICE: 0x46,
  GPU_SHADER_COMPILE: 0x47,
  GPU_VRAM_STATUS: 0x48,
  GPU_BATCH_SUBMIT: 0x49,
  GPU_PREFETCH_SHADERS: 0x4A,
  GPU_COMPRESS_RESOURCE: 0x4B,
  GPU_RESTORE_RESOURCE: 0x4C,
  GPU_EVICT_RESOURCE: 0x4D,
  GPU_METRICS_SNAPSHOT: 0x4E,
});

const EVENT_TYPES = Object.freeze({
  RAM_PRESSURE: 'RAM_PRESSURE',
  RAM_GC_REQUEST: 'RAM_GC_REQUEST',
  USB_DEVICE: 'USB_DEVICE',
  USB_DURABILITY: 'USB_DURABILITY',
  GPU_THERMAL: 'GPU_THERMAL',
  GPU_FRAME_PRESSURE: 'GPU_FRAME_PRESSURE',
  KERNEL_HEARTBEAT: 'KERNEL_HEARTBEAT',
  MODULE_HEALTH: 'MODULE_HEALTH',
  BOOT_READY: 'BOOT_READY',
  SHUTDOWN_REQUEST: 'SHUTDOWN_REQUEST',
  SHUTDOWN_BLOCKED: 'SHUTDOWN_BLOCKED',
  SERVICE_ERROR: 'SERVICE_ERROR',
  MODULE_RECONNECTED: 'MODULE_RECONNECTED',
});

const EVENT_SEVERITIES = Object.freeze(['debug', 'info', 'warning', 'error', 'critical']);

const EVENT_ROUTING = Object.freeze({
  RAM_PRESSURE: ['airbar', 'devtools', 'settings'],
  RAM_GC_REQUEST: ['kernel'],
  USB_DURABILITY: ['finder', 'airbar', 'devtools'],
  USB_DEVICE: ['finder', 'settings', 'airbar'],
  GPU_THERMAL: ['airbar', 'orbit', 'devtools'],
  GPU_FRAME_PRESSURE: ['desktop', 'dwp', 'devtools'],
  MODULE_HEALTH: ['devtools', 'settings'],
  BOOT_READY: ['desktop'],
  SHUTDOWN_BLOCKED: ['desktop', 'settings'],
  MODULE_RECONNECTED: ['devtools'],
});

const SENSITIVE_COMMANDS = Object.freeze([
  'RAM_SET_QUOTA',
  'USB_EJECT',
  'USB_FLUSH',
  'GPU_SWITCH_DEVICE',
  'GPU_EVICT_RESOURCE',
  'ASC_GENERATE',
  'ASC_WRITE',
  'SHUTDOWN_REQUEST',
  'SHUTDOWN_PREPARE',
]);

module.exports = {
  SBP_MAGIC,
  SBP_VERSION,
  SBP_FLAGS,
  SBP_MAX_PAYLOAD,
  MODULE_IDS,
  MODULE_PROTOCOLS,
  MODULE_STATUSES,
  SOCKET_PATHS,
  HEARTBEAT_DEFAULTS,
  HEALTH_TIMEOUT_MULTIPLIER,
  RECONNECT_BACKOFF,
  SHUTDOWN_TIMEOUTS,
  OPOCODES,
  EVENT_TYPES,
  EVENT_SEVERITIES,
  EVENT_ROUTING,
  SENSITIVE_COMMANDS,
};
