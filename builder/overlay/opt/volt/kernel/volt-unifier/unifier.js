'use strict';

const { ModuleRegistry } = require('./registry/moduleRegistry');
const { SbpRouter } = require('./sbp/router');
const { SystemEventBus } = require('./events/eventBus');
const { HealthMonitor } = require('./health/healthMonitor');
const { HeartbeatManager } = require('./health/heartbeat');
const { ReconnectPolicy } = require('./health/reconnect');
const { LifecycleManager } = require('./lifecycle/lifecycleManager');
const { ReadinessState } = require('./lifecycle/readiness');
const { ShutdownOrchestrator } = require('./lifecycle/shutdown');
const { MetricsAggregator } = require('./metrics/aggregator');
const { DashboardFeed } = require('./metrics/dashboardFeed');
const { AuditLog } = require('./safety/auditLog');
const { BootClient } = require('./clients/bootClient');
const { AscClient } = require('./clients/ascClient');
const { DwpClient } = require('./clients/dwpClient');
const { KernelBClient } = require('./clients/kernelBClient');
const { VrmClient } = require('./clients/vrmClient');
const { VumClient } = require('./clients/vumClient');
const { VgmClient } = require('./clients/vgmClient');
const { DesktopBridge } = require('./bridges/desktopBridge');
const { FinderBridge } = require('./bridges/finderBridge');
const { SettingsBridge } = require('./bridges/settingsBridge');
const { OrbitBridge } = require('./bridges/orbitBridge');
const { DevToolsBridge } = require('./bridges/devtoolsBridge');
const { AirBarBridge } = require('./bridges/airbarBridge');

class VoltUnifier {
  constructor(kernel, config) {
    this.kernel = kernel;
    this.config = config;
    this.state = 'uninitialized';

    this.audit = new AuditLog();
    this.eventBus = new SystemEventBus(kernel.eventBus);
    this.registry = new ModuleRegistry(this);
    this.sbpRouter = new SbpRouter(this);
    this.lifecycle = new LifecycleManager(this);
    this.healthMonitor = new HealthMonitor(this);
    this.heartbeatManager = new HeartbeatManager(this);
    this.reconnectPolicy = new ReconnectPolicy();
    this.readiness = new ReadinessState(this);
    this.shutdownOrch = new ShutdownOrchestrator(this);
    this.metrics = new MetricsAggregator(this);
    this.dashboardFeed = new DashboardFeed(this);

    this.bridges = {};
    this.clients = {};
  }

  async init() {
    this.state = 'starting';
    this.audit.record({ action: 'unifier:init', moduleId: 'core', allowed: true, reason: 'system startup' });

    this._registerModules();
    await this._connectModuleClients();
    await this._checkAscGeneratedConfig();
    this._initBridges();

    this.heartbeatManager.start();
    this.healthMonitor.start();

    this.state = 'running';
    this.lifecycle.start();

    this.readiness.markReady();

    this.audit.record({ action: 'unifier:ready', moduleId: 'core', allowed: true, reason: 'all modules initialized' });
    return this;
  }

  async shutdown() {
    this.state = 'stopping';
    this.audit.record({ action: 'unifier:shutdown', moduleId: 'core', allowed: true, reason: 'system shutdown' });

    this.healthMonitor.stop();
    this.heartbeatManager.stop();

    for (const bridge of Object.values(this.bridges)) {
      try { bridge.destroy(); } catch (e) { /* ignore */ }
    }

    for (const client of Object.values(this.clients)) {
      if (client.disconnect) {
        try { await client.disconnect(); } catch (e) { /* ignore */ }
      }
    }

    this.state = 'stopped';
    this.lifecycle.stop();
  }

  getModule(id) {
    return this.registry.get(id);
  }

  getClient(id) {
    return this.clients[id] || null;
  }

  getSnapshot() {
    return this.metrics.getDashboardSnapshot();
  }

  getHealthSnapshot() {
    return this.healthMonitor.getSystemHealthSnapshot();
  }

  _registerModules() {
    const modules = this.config.modules || {};
    for (const [id, modConfig] of Object.entries(modules)) {
      if (modConfig.enabled !== false) {
        this.registry.register(id, modConfig);
      }
    }
  }

  async _connectModuleClients() {
    const clientFactories = {
      'kernel-b': (u) => new KernelBClient(u),
      vrm: (u) => new VrmClient(u),
      vum: (u) => new VumClient(u),
      vgm: (u) => new VgmClient(u),
      dwp: (u) => new DwpClient(u),
    };

    for (const [id, factory] of Object.entries(clientFactories)) {
      const entry = this.registry.get(id);
      if (!entry) continue;

      const client = factory(this);
      this.clients[id] = client;
      entry.client = client;

      try {
        await client.connect();
        this.registry.updateStatus(id, client.status, {
          lastHeartbeatAt: client.lastHeartbeatAt || Date.now(),
          capabilities: id === 'dwp' ? ['scheduling', 'adaptive-scaling', 'orbit-burst'] : entry.health.capabilities,
          lastError: client.lastError,
          degradedReason: client.degradedReason,
          errorCount: client.errorCount,
          reconnectCount: client.reconnectCount,
        });
      } catch (e) {
        this.registry.updateStatus(id, 'error', {
          lastError: e.message,
          errorCount: entry.health.errorCount + 1,
        });
      }
    }

    // ASC client (CLI-based)
    const ascEntry = this.registry.get('asc');
    if (ascEntry) {
      const ascClient = new AscClient(this);
      this.clients.asc = ascClient;
      ascEntry.client = ascClient;
      await ascClient.connect();
      this.registry.updateStatus('asc', ascClient.status, {
        lastHeartbeatAt: ascClient.lastHeartbeatAt || Date.now(),
        lastError: ascClient.lastError,
        degradedReason: ascClient.degradedReason,
        errorCount: ascClient.errorCount,
        reconnectCount: ascClient.reconnectCount,
      });
    }

    // Boot client (file-based)
    const bootEntry = this.registry.get('volt-boot');
    if (bootEntry) {
      const bootClient = new BootClient(this);
      this.clients['volt-boot'] = bootClient;
      bootEntry.client = bootClient;
      this.registry.updateStatus('volt-boot', 'online', {
        lastHeartbeatAt: Date.now(),
      });
    }
  }

  async _checkAscGeneratedConfig() {
    const asc = this.clients.asc;
    if (!asc) return;

    const existing = await asc.refresh();
    if (existing.ok) {
      this.registry.updateStatus('asc', 'online', {
        lastHeartbeatAt: asc.lastHeartbeatAt || Date.now(),
        lastError: null,
        degradedReason: null,
        errorCount: asc.errorCount,
        reconnectCount: asc.reconnectCount,
      });
      this.audit.record({ action: 'asc:config-found', moduleId: 'asc', allowed: true, reason: 'ASC generated config present' });
      return;
    }

    try {
      await asc.generateAndWrite(asc.generatedConfig || '/run/samaris/adaptive.generated.toml');
      this.registry.updateStatus('asc', asc.status, {
        lastHeartbeatAt: asc.lastHeartbeatAt || Date.now(),
        lastError: asc.lastError,
        degradedReason: asc.degradedReason,
        errorCount: asc.errorCount,
        reconnectCount: asc.reconnectCount,
      });
      this.audit.record({ action: 'asc:config-generated', moduleId: 'asc', allowed: true, reason: 'ASC executed at boot' });
    } catch (e) {
      this.registry.updateStatus('asc', 'degraded', {
        lastHeartbeatAt: Date.now(),
        lastError: e.message,
        degradedReason: e.message,
        errorCount: asc.errorCount + 1,
      });
      this.audit.record({ action: 'asc:config-failed', moduleId: 'asc', allowed: false, reason: e.message });
    }
  }

  _initBridges() {
    this.bridges.desktop = new DesktopBridge(this);
    this.bridges.finder = new FinderBridge(this);
    this.bridges.settings = new SettingsBridge(this);
    this.bridges.orbit = new OrbitBridge(this);
    this.bridges.devtools = new DevToolsBridge(this);
    this.bridges.airbar = new AirBarBridge(this);

    for (const bridge of Object.values(this.bridges)) {
      try { bridge.start(); } catch (e) { /* log */ }
    }
  }
}

function createUnifier(kernel, config) {
  return new VoltUnifier(kernel, config);
}

module.exports = { VoltUnifier, createUnifier };
