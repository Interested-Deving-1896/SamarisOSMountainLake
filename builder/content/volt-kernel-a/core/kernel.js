const logger = require("./logger");
const EventBus = require("./eventBus");
const Scheduler = require("./scheduler");
const Auth = require("./auth");
const IPC = require("./ipc");
const App = require("../models/App");
const Device = require("../models/Device");

const FileSystemService = require("../services/fileSystem");
const MediaService = require("../services/mediaService");
const ProcessManager = require("../services/processManager");
const RuntimeManager = require("../services/runtimeManager");
const PermissionManager = require("../services/permissionManager");
const WindowManager = require("../services/windowManager");
const OrbitRuntimeService = require("../services/orbitRuntime");
const MailService = require("../services/mailService");
const ConnectivityService = require("../services/connectivityService");
const NetworkService = require("../services/networkService");
const AudioService = require("../services/audioService");
const BatteryService = require("../services/batteryService");
const BrowserService = require("../services/browserService");
const PrintService = require("../services/printService");
const FirewallService = require("../services/firewallService");
const AppStoreService = require("../services/appStoreService");
const AppStaticServer = require("../services/appStaticServer");
const EncryptionService = require("../services/encryptionService");
const SearchService = require("../services/searchService");
const SessionFeaturesService = require("../services/sessionFeaturesService");
const WineService = require("../services/wineService");
const OnboardingService = require("../services/onboardingService");
const DevStateService = require("../services/devStateService");
const StorageService = require("../services/storageService");
const DiskService = require("../services/diskService");
const ArchiveService = require("../services/archiveService");
const PowerService = require("../services/powerService");
const SystemMetricsService = require("../services/systemMetricsService");
const UserService = require("../services/userService");
const KernelBClient = require("../services/kernelBClient");
const VaultService = require("../services/vaultService");
const SttService = require("../services/sttService");
const TtsService = require("../services/ttsService");

class VoltKernel {
  constructor() {
    this.logger = logger;
    this.eventBus = new EventBus(logger);
    this.scheduler = new Scheduler(logger, this.eventBus);
    this.auth = new Auth(logger);
    this.ipc = new IPC(logger, this.eventBus);

    // Core infrastructure — needed early or by many services
    this.permissionManager = new PermissionManager(logger);
    this.processManager = new ProcessManager(logger, this.eventBus, this.scheduler);
    this.runtimeManager = new RuntimeManager(logger, this.eventBus);
    this.systemMetrics = new SystemMetricsService(logger, this.eventBus);
    this.user = new UserService(logger);
    this.kernelB = new KernelBClient(logger);
    this.fileSystem = new FileSystemService(logger, this.eventBus, this.user, this.kernelB);
    this.storage = new StorageService(logger, this.eventBus, this.fileSystem);

    // Everything else loads on first access
    this._lazy = {};

    const self = this;
    function resolveLazy(prop) {
      if (prop in self) return self[prop];
      if (lazy[prop]) {
        if (!self._lazy[prop]) {
          self._lazy[prop] = lazy[prop]();
        }
        return self._lazy[prop];
      }
      return undefined;
    }

    const lazy = {
      vault: () => new VaultService(logger, self.user, self.kernelB),
      media: () => new MediaService(logger, self.fileSystem),
      windowManager: () => new WindowManager(logger, self.eventBus),
      orbit: () => new OrbitRuntimeService(logger, self.eventBus),
      mail: () => new MailService(logger, self.eventBus, self.user, resolveLazy("vault")),
      connectivity: () => new ConnectivityService(logger, self.eventBus, self.user, resolveLazy("vault")),
      network: () => new NetworkService(logger, resolveLazy("connectivity"), self.fileSystem),
      audio: () => new AudioService(logger),
      battery: () => new BatteryService(logger),
      browser: () => new BrowserService(logger, self.eventBus),
      print: () => new PrintService(logger, self.fileSystem),
      firewall: () => new FirewallService(logger),
      appStaticServer: () => new AppStaticServer(logger),
      appStore: () => new AppStoreService(logger, resolveLazy("appStaticServer")),
      encryption: () => new EncryptionService(logger),
      search: () => new SearchService(logger, self.fileSystem),
      disk: () => new DiskService(logger),
      archive: () => new ArchiveService(logger),
      sessionFeatures: () => new SessionFeaturesService(logger, self.storage, self.user),
      wine: () => new WineService(logger, self.fileSystem),
      onboarding: () => new OnboardingService(logger, resolveLazy("sessionFeatures"), resolveLazy("encryption"), self.storage),
      powerService: () => new PowerService(logger, self.eventBus),
      stt: () => new SttService(logger),
      tts: () => new TtsService(logger),
      devState: () => new DevStateService(logger),
    };

    // Wrap property access for lazy loading
    return new Proxy(this, {
      get(target, prop) {
        if (prop in target) return target[prop];
        if (lazy[prop]) {
          if (!target._lazy[prop]) {
            target._lazy[prop] = lazy[prop]();
          }
          return target._lazy[prop];
        }
        return undefined;
      },
      set(target, prop, value) {
        target[prop] = value;
        return true;
      }
    });
  }

  init() {
    this.logger.info("kernel:init", "Samaris Kernel V3");
    this.scheduler.start();
    void this.storage.init();
    this.systemMetrics.start();

    const SYSTEM_NAMESPACES = [
      "system.*", "user.*", "fs.*", "window.*", "event.*",
      "device.*", "audio.*", "battery.*", "session.*", "runtime.*",
      "process.*", "app.*", "search.*", "storage.*", "network.*",
      "power.*", "permission.*", "mail.*", "media.*", "print.*",
      "wine.*", "orbit.*", "encryption.*", "onboarding.*", "firewall.*",
      "browser.*", "disk.*", "archive.*", "stt.*", "tts.*",
    ];
    const desktopApp = new App({ id: "volt.desktop", name: "Samaris OS UI", runtime: "app", permissions: SYSTEM_NAMESPACES });
    const finderApp = new App({ id: "finder", name: "Finder", runtime: "app", permissions: ["fs.read", "fs.write", "window.focus", "event.emit"] });
    const mailApp = new App({ id: "mail", name: "Mail", runtime: "app", permissions: ["event.emit", "window.focus"] });
    const systemApps = [
      ["print", "Print", ["event.emit", "window.focus"]],
      ["permissions-manager", "Permissions Manager", ["event.emit", "window.focus"]],
      ["firewall", "Firewall", ["event.emit", "window.focus"]],
      ["app-store", "App Store", ["event.emit", "window.focus", "network"]],
      ["pdf-viewer", "PDF Viewer", ["fs.read", "window.focus"]],
      ["encryption", "Encryption", ["window.focus"]],
      ["wine", "Wine", ["fs.read", "window.focus", "event.emit"]]
    ].map(([id, name, permissions]) => new App({ id, name, runtime: "app", permissions }));
    this.apps = [desktopApp, finderApp, mailApp, ...systemApps];
    for (const app of this.apps) {
      this.permissionManager.seed(app.id, app.permissions);
    }

    this.runtimeManager.startRuntime({ id: "runtime-chromium-1", kind: "chromium", target: "volt.desktop" });
    this.processManager.createProcess({ appId: "volt.desktop", runtime: "chromium", permissions: desktopApp.permissions });

    this.devices = [new Device({ id: "device-display-1", type: "display" }), new Device({ id: "device-input-1", type: "input" })];
    this.session = { lastBootAt: new Date().toISOString(), restored: false };
  }

  getPublicState() {
    return {
      processes: this.processManager.list(),
      windows: (this._lazy.windowManager || this.windowManager).list(),
      runtimes: this.runtimeManager.list(),
      devices: [...(this.devices || [])],
      session: { ...(this.session || {}) }
    };
  }
}

function createKernel() {
  return new VoltKernel();
}

module.exports = { createKernel };
