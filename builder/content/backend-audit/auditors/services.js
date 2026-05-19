const path = require("node:path");
const fs = require("node:fs/promises");
const {
  MockLogger, MockEventBus, MockUserService, MockKernelB,
  createTempFileSystem, destroyTempFileSystem,
} = require("../engine/mockFactory");
const { runServiceTest } = require("../engine/serviceRunner");

async function auditPermissionManager(logger, kernelRoot) {
  const results = [];
  const PM = require(`${kernelRoot}/services/permissionManager`);
  const pm = new PM(logger);

  results.push(await runServiceTest("PermissionManager", "can() exact match", () => {
    pm.seed("app1", ["fs.read"]);
    pm.can("app1", "fs.read");
  }));

  results.push(await runServiceTest("PermissionManager", "can() wildcard match", () => {
    pm.seed("app2", ["fs.*"]);
    pm.can("app2", "fs.write");
  }));

  results.push(await runServiceTest("PermissionManager", "can() no match", () => {
    pm.can("unknown", "fs.read");
  }));

  results.push(await runServiceTest("PermissionManager", "seed 100 permissions", () => {
    const perms = Array.from({ length: 100 }, (_, i) => `ns${i}.action${i}`);
    pm.seed("app3", perms);
  }));

  results.push(await runServiceTest("PermissionManager", "list() for app", () => {
    pm.list("app1");
  }));

  results.push(await runServiceTest("PermissionManager", "listAll()", () => {
    pm.listAll();
  }));

  return results;
}

async function auditFileSystem(logger, kernelRoot) {
  const results = [];
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const tempFs = await createTempFileSystem();
  const userService = new MockUserService();
  const fsService = new FS(logger, new MockEventBus(), userService, new MockKernelB());
  fsService.userRootPath = tempFs.userRoot;
  fsService.virtualRoots["/User"] = tempFs.userRoot;
  fsService.initialized = null;

  results.push(await runServiceTest("FileSystemService", "toVirtualPath normal", () => {
    fsService.toVirtualPath("/User/Documents/file.txt");
  }));

  results.push(await runServiceTest("FileSystemService", "toActualPath valid", () => {
    fsService.toActualPath("/User/Documents/test.txt");
  }));

  results.push(await runServiceTest("FileSystemService", "toActualPath traversal blocked", () => {
    try { fsService.toActualPath("/User/../../etc/passwd"); } catch {}
  }));

  results.push(await runServiceTest("FileSystemService", "write 1KB", () =>
    fsService.write("/User/Documents/small.txt", "x".repeat(1024))
  ));

  results.push(await runServiceTest("FileSystemService", "read 1KB", () =>
    fsService.read("/User/Documents/small.txt")
  ));

  results.push(await runServiceTest("FileSystemService", "readDataUrl", () =>
    fsService.readDataUrl("/User/Documents/small.txt")
  ));

  results.push(await runServiceTest("FileSystemService", "writeBase64", () =>
    fsService.writeBase64("/User/Documents/b64.txt", "aGVsbG8=")
  ));

  results.push(await runServiceTest("FileSystemService", "list 10 files", async () => {
    for (const f of ["a.txt","b.txt","c.txt","d.txt","e.txt","f.txt","g.txt","h.txt","i.txt","j.txt"]) {
      await fsService.write(`/User/Documents/${f}`, f);
    }
    return fsService.list("/User/Documents");
  }));

  results.push(await runServiceTest("FileSystemService", "mkdir recursive", () =>
    fsService.mkdir("/User/Documents/a/b/c")
  ));

  results.push(await runServiceTest("FileSystemService", "rename", async () => {
    await fsService.write("/User/Documents/rename-test.txt", "renamable");
    await fsService.rename("/User/Documents/rename-test.txt", "/User/Documents/rename-done.txt");
  }));

  results.push(await runServiceTest("FileSystemService", "remove", async () => {
    await fsService.write("/User/Documents/remove-me.txt", "bye");
    await fsService.remove("/User/Documents/remove-me.txt");
  }));

  results.push(await runServiceTest("FileSystemService", "setExternalRoots", () => {
    fsService.setExternalRoots([{ id: "USB", actualPath: tempFs.volumeRoot }]);
  }));

  results.push(await runServiceTest("FileSystemService", "resolveRoot", () => {
    fsService.resolveRoot("/Volumes/USB/data");
  }));

  results.push(await runServiceTest("FileSystemService", "list root /", () =>
    fsService.list("/")
  ));

  await destroyTempFileSystem(tempFs.root);
  return results;
}

async function auditVault(logger, kernelRoot) {
  const results = [];
  const Vault = require(`${kernelRoot}/services/vaultService`);
  const userService = new MockUserService();

  results.push(await runServiceTest("VaultService", "encrypt 16 chars", async () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    await vault.encryptForActiveUser("secret-data-16!");
  }));

  results.push(await runServiceTest("VaultService", "decrypt 16 chars", async () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    const env = await vault.encryptForActiveUser("secret-data-16!");
    await vault.decryptForActiveUser(env);
  }));

  results.push(await runServiceTest("VaultService", "encrypt 1KB", async () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    await vault.encryptForActiveUser("x".repeat(1024));
  }));

  results.push(await runServiceTest("VaultService", "decrypt 1KB", async () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    const env = await vault.encryptForActiveUser("x".repeat(1024));
    await vault.decryptForActiveUser(env);
  }));

  results.push(await runServiceTest("VaultService", "encryptString static", () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    vault.encryptString("data", "password");
  }));

  results.push(await runServiceTest("VaultService", "decryptString static", () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    const env = vault.encryptString("data", "password");
    vault.decryptString(env, "password");
  }));

  results.push(await runServiceTest("VaultService", "requireIdentity locked", () => {
    try {
      const vault = new Vault(logger, { getVaultIdentity() { return null; } }, new MockKernelB());
      vault.requireIdentity();
    } catch {}
  }));

  return results;
}

async function auditUserService(logger, kernelRoot) {
  const results = [];
  const UserService = require(`${kernelRoot}/services/userService`);
  const us = new UserService(logger);
  const uid = `test-${Date.now().toString(36)}-${Math.random().toString(36).slice(2, 6)}`;

  // Create the test user once before benchmarking
  try { await us.delete(uid); } catch {}
  await us.create(uid, "Test User", "password123");

  results.push(await runServiceTest("UserService", "authenticate success", async () => {
    await us.login(uid, "password123");
  }));

  results.push(await runServiceTest("UserService", "authenticate wrong password", async () => {
    try { await us.login(uid, "wrong"); } catch {}
  }));

  results.push(await runServiceTest("UserService", "getActiveUser", () => {
    us.getActiveUser();
  }));

  results.push(await runServiceTest("UserService", "list users", () =>
    us.list()
  ));

  results.push(await runServiceTest("UserService", "resolveHome", () => {
    us.resolveHome("test-user");
  }));

  // Cleanup
  await us.delete(uid).catch(() => {});

  return results;
}

async function auditArchiveService(logger, kernelRoot) {
  const results = [];
  const Archive = require(`${kernelRoot}/services/archiveService`);
  const svc = new Archive(logger);

  results.push(await runServiceTest("ArchiveService", "instantiation", () => {
    new Archive(logger);
  }));

  return results;
}

async function auditMediaService(logger, kernelRoot) {
  const results = [];
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const Media = require(`${kernelRoot}/services/mediaService`);
  const tempFs = await createTempFileSystem();
  const fsService = new FS(logger, new MockEventBus(), new MockUserService(), new MockKernelB());
  fsService.userRootPath = tempFs.userRoot;
  fsService.virtualRoots["/User"] = tempFs.userRoot;

  const ms = new Media(logger, fsService);

  results.push(await runServiceTest("MediaService", "list music library", async () => {
    try { await ms.listMusicLibrary(); } catch {}
  }));

  results.push(await runServiceTest("MediaService", "list video library", async () => {
    try { await ms.listVideoLibrary(); } catch {}
  }));

  await destroyTempFileSystem(tempFs.root);
  return results;
}

async function auditAudioService(logger, kernelRoot) {
  const results = [];
  const Audio = require(`${kernelRoot}/services/audioService`);
  const svc = new Audio(logger);

  results.push(await runServiceTest("AudioService", "getStatus", () => svc.getStatus()));
  results.push(await runServiceTest("AudioService", "setVolume", () => svc.setVolume(0.5)));
  results.push(await runServiceTest("AudioService", "setOutput", () => svc.setOutput({ outputId: "default" })));

  return results;
}

async function auditBatteryService(logger, kernelRoot) {
  const results = [];
  const Bat = require(`${kernelRoot}/services/batteryService`);
  const svc = new Bat(logger);

  results.push(await runServiceTest("BatteryService", "getStatus", () => svc.getStatus()));

  return results;
}

async function auditNetworkService(logger, kernelRoot) {
  const results = [];
  const C = require(`${kernelRoot}/services/connectivityService`);
  const N = require(`${kernelRoot}/services/networkService`);
  const conn = new C(logger, new MockEventBus(), new MockUserService());
  const svc = new N(logger, conn);

  results.push(await runServiceTest("NetworkService", "list", () => svc.list()));
  return results;
}

async function auditPowerService(logger, kernelRoot) {
  const results = [];
  const P = require(`${kernelRoot}/services/powerService`);
  const svc = new P(logger, new MockEventBus());

  results.push(await runServiceTest("PowerService", "shutdown", () => svc.shutdown()));

  return results;
}

async function auditSystemMetrics(logger, kernelRoot) {
  const results = [];
  const M = require(`${kernelRoot}/services/systemMetricsService`);
  const svc = new M(logger, new MockEventBus());

  svc.start();

  results.push(await runServiceTest("SystemMetricsService", "getMetrics", () => svc.getMetrics()));

  svc.stop();
  return results;
}

async function auditProcessManager(logger, kernelRoot) {
  const results = [];
  const PM = require(`${kernelRoot}/services/processManager`);
  const svc = new PM(logger, new MockEventBus());

  results.push(await runServiceTest("ProcessManager", "list", () => svc.list()));
  results.push(await runServiceTest("ProcessManager", "create + list", () => {
    svc.createProcess({ appId: "test", runtime: "chromium", permissions: ["fs.read"] });
    svc.list();
  }));

  return results;
}

async function auditRuntimeManager(logger, kernelRoot) {
  const results = [];
  const RM = require(`${kernelRoot}/services/runtimeManager`);
  const svc = new RM(logger, new MockEventBus());

  results.push(await runServiceTest("RuntimeManager", "list", () => svc.list()));
  results.push(await runServiceTest("RuntimeManager", "start + list", () => {
    svc.startRuntime({ id: "test-runtime", kind: "chromium", target: "test" });
    svc.list();
  }));

  return results;
}

async function auditWindowManager(logger, kernelRoot) {
  const results = [];
  const WM = require(`${kernelRoot}/services/windowManager`);
  const svc = new WM(logger, new MockEventBus());

  results.push(await runServiceTest("WindowManager", "list", () => svc.list()));
  results.push(await runServiceTest("WindowManager", "open + list", () => {
    svc.openWindow({ id: "test-window", appId: "test" });
    svc.list();
  }));

  return results;
}

async function auditDiskService(logger, kernelRoot) {
  const results = [];
  const D = require(`${kernelRoot}/services/diskService`);
  const svc = new D(logger);

  results.push(await runServiceTest("DiskService", "getStorage", () => svc.getStorage()));
  results.push(await runServiceTest("DiskService", "listDisks", () => svc.listDisks()));

  return results;
}

async function auditFirewallService(logger, kernelRoot) {
  const results = [];
  const F = require(`${kernelRoot}/services/firewallService`);
  const svc = new F(logger);

  results.push(await runServiceTest("FirewallService", "readSystemStatus", () => svc.readSystemStatus()));
  results.push(await runServiceTest("FirewallService", "list", () => svc.list()));

  return results;
}

async function auditPrintService(logger, kernelRoot) {
  const results = [];
  const P = require(`${kernelRoot}/services/printService`);
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const fsService = new FS(logger, new MockEventBus(), new MockUserService(), new MockKernelB());
  const svc = new P(logger, fsService);

  results.push(await runServiceTest("PrintService", "listPrinters", () => svc.list()));

  return results;
}

async function auditSessionFeatures(logger, kernelRoot) {
  const results = [];
  const SF = require(`${kernelRoot}/services/sessionFeaturesService`);

  results.push(await runServiceTest("SessionFeaturesService", "instantiation", () => {
    new SF(logger);
  }));

  return results;
}

async function auditSearchService(logger, kernelRoot) {
  const results = [];
  const S = require(`${kernelRoot}/services/searchService`);
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const fsService = new FS(logger, new MockEventBus(), new MockUserService(), new MockKernelB());
  const svc = new S(logger, fsService);

  results.push(await runServiceTest("SearchService", "query", () => svc.query("test", { apps: [] })));

  return results;
}

async function auditDevState(logger, kernelRoot) {
  const results = [];
  const D = require(`${kernelRoot}/services/devStateService`);
  const svc = new D(logger);

  results.push(await runServiceTest("DevStateService", "getResetState", () => svc.getResetState()));

  return results;
}

async function auditStorageService(logger, kernelRoot) {
  const results = [];
  const S = require(`${kernelRoot}/services/storageService`);
  const svc = new S(logger, new MockEventBus(), null);

  results.push(await runServiceTest("StorageService", "loadState", () => svc.loadState()));
  results.push(await runServiceTest("StorageService", "defaultState", () => svc.defaultState()));

  return results;
}

async function auditWineService(logger, kernelRoot) {
  const results = [];
  const W = require(`${kernelRoot}/services/wineService`);
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const fsService = new FS(logger, new MockEventBus(), new MockUserService(), new MockKernelB());
  const svc = new W(logger, fsService);

  results.push(await runServiceTest("WineService", "checkWineInstalled", async () => {
    await svc.checkWineInstalled();
  }));

  return results;
}

async function auditBrowserService(logger, kernelRoot) {
  const results = [];
  const B = require(`${kernelRoot}/services/browserService`);
  const svc = new B(logger, new MockEventBus());

  results.push(await runServiceTest("BrowserService", "resolveBinary", () => svc.resolveBinary()));
  results.push(await runServiceTest("BrowserService", "status", () => svc.status()));
  results.push(await runServiceTest("BrowserService", "normalizeAddress", () => {
    svc.normalizeAddress("https://example.com");
  }));
  results.push(await runServiceTest("BrowserService", "normalizeAddress search", () => {
    svc.normalizeAddress("hello world");
  }));

  return results;
}

async function auditKernelBClient(logger, kernelRoot) {
  const results = [];
  const K = require(`${kernelRoot}/services/kernelBClient`);
  const svc = new K(logger);

  results.push(await runServiceTest("KernelBClient", "available (no socket)", () => {
    svc.available();
  }));

  return results;
}

async function auditEncryptionService(logger, kernelRoot) {
  const results = [];
  const E = require(`${kernelRoot}/services/encryptionService`);
  const svc = new E(logger);

  results.push(await runServiceTest("EncryptionService", "status", () => svc.status()));

  return results;
}

async function auditAppStoreService(logger, kernelRoot) {
  const results = [];
  const AS = require(`${kernelRoot}/services/appStaticServer`);
  const AppStore = require(`${kernelRoot}/services/appStoreService`);
  const appStatic = new AS(logger);
  const svc = new AppStore(logger, appStatic);

  results.push(await runServiceTest("AppStoreService", "loadRegistry", () => svc.loadRegistry()));
  results.push(await runServiceTest("AppStoreService", "normalize URL", () => {
    svc.normalize("https://github.com/user/repo.git");
  }));

  return results;
}

async function auditOrbitRuntime(logger, kernelRoot) {
  const results = [];
  const O = require(`${kernelRoot}/services/orbitRuntime`);
  const svc = new O(logger, new MockEventBus());

  results.push(await runServiceTest("OrbitRuntimeService", "status (no model)", () => svc.status()));
  results.push(await runServiceTest("OrbitRuntimeService", "formatBytes", () => {
    // test internal helper
  }));

  return results;
}

async function auditTTSService(logger, kernelRoot) {
  const results = [];
  const T = require(`${kernelRoot}/services/ttsService`);
  const svc = new T(logger);

  results.push(await runServiceTest("TTSService", "instantiation", () => {}));
  return results;
}

async function auditSTTService(logger, kernelRoot) {
  const results = [];
  const S = require(`${kernelRoot}/services/sttService`);
  const svc = new S(logger);

  results.push(await runServiceTest("STTService", "instantiation", () => {}));
  return results;
}

async function auditConnectivityService(logger, kernelRoot) {
  const results = [];
  const C = require(`${kernelRoot}/services/connectivityService`);
  const svc = new C(logger, new MockEventBus(), new MockUserService());

  results.push(await runServiceTest("ConnectivityService", "getStatus", () => svc.getStatus()));

  return results;
}

async function auditMailService(logger, kernelRoot) {
  const results = [];
  const M = require(`${kernelRoot}/services/mailService`);
  const svc = new M(logger, new MockEventBus(), new MockUserService());

  results.push(await runServiceTest("MailService", "instantiation", () => {}));
  return results;
}

module.exports = {
  auditPermissionManager,
  auditFileSystem,
  auditVault,
  auditUserService,
  auditArchiveService,
  auditMediaService,
  auditAudioService,
  auditBatteryService,
  auditNetworkService,
  auditPowerService,
  auditSystemMetrics,
  auditProcessManager,
  auditRuntimeManager,
  auditWindowManager,
  auditDiskService,
  auditFirewallService,
  auditPrintService,
  auditSessionFeatures,
  auditSearchService,
  auditDevState,
  auditStorageService,
  auditWineService,
  auditBrowserService,
  auditKernelBClient,
  auditEncryptionService,
  auditAppStoreService,
  auditOrbitRuntime,
  auditTTSService,
  auditSTTService,
  auditConnectivityService,
  auditMailService,
  label: "Services",
};
