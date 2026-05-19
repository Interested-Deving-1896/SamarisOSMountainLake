const path = require("node:path");
const { runServiceTest } = require("../engine/serviceRunner");
const { MockLogger, MockEventBus, MockUserService, MockKernelB,
        createTempFileSystem, destroyTempFileSystem } = require("../engine/mockFactory");

async function auditFileWriteReadCycle(logger, kernelRoot) {
  const results = [];
  const FS = require(`${kernelRoot}/services/fileSystem`);
  const tempFs = await createTempFileSystem();
  const fsService = new FS(logger, new MockEventBus(), new MockUserService(), new MockKernelB());
  fsService.userRootPath = tempFs.userRoot;
  fsService.virtualRoots["/User"] = tempFs.userRoot;

  results.push(await runServiceTest("Integration", "file write → read cycle", async () => {
    const original = "Hello Samaris Integration! X".repeat(100);
    await fsService.write("/User/Documents/integration-test.txt", original);
    const result = await fsService.read("/User/Documents/integration-test.txt");
    if (result.content !== original) throw new Error("Data mismatch in write/read cycle");
  }));

  results.push(await runServiceTest("Integration", "base64 write → dataUrl read", async () => {
    const b64 = Buffer.from("binary-data").toString("base64");
    await fsService.writeBase64("/User/Documents/binary.dat", b64);
    const result = await fsService.readDataUrl("/User/Documents/binary.dat");
    if (!result.dataUrl.startsWith("data:")) throw new Error("Invalid data URL");
  }));

  results.push(await runServiceTest("Integration", "mkdir → list → remove", async () => {
    await fsService.mkdir("/User/Documents/temp-dir/nested");
    const list = await fsService.list("/User/Documents");
    const found = list.nodes.find((n) => n.name === "temp-dir");
    if (!found) throw new Error("Directory not found after mkdir");
  }));

  await destroyTempFileSystem(tempFs.root);
  return results;
}

async function auditPermissionAuthIntegration(logger, kernelRoot) {
  const results = [];
  const Auth = require(`${kernelRoot}/core/auth`);
  const PM = require(`${kernelRoot}/services/permissionManager`);

  const pm = new PM(logger);
  const auth = new Auth(logger);
  const kernel = { permissionManager: pm };

  pm.seed("secure-app", ["fs.read", "system.*"]);
  pm.seed("untrusted-app", ["fs.read"]);

  results.push(await runServiceTest("Integration", "permission → auth allow", () => {
    if (!auth.authorize({ type: "system.ping", appId: "secure-app" }, kernel)) {
      throw new Error("Expected allowed");
    }
  }));

  results.push(await runServiceTest("Integration", "permission → auth deny", () => {
    if (auth.authorize({ type: "system.ping", appId: "untrusted-app" }, kernel)) {
      throw new Error("Expected denied");
    }
  }));

  return results;
}

async function auditVaultEncryptDecryptCycle(logger, kernelRoot) {
  const results = [];
  const Vault = require(`${kernelRoot}/services/vaultService`);
  const userService = new MockUserService();

  results.push(await runServiceTest("Integration", "vault encrypt → decrypt cycle", async () => {
    const vault = new Vault(logger, userService, new MockKernelB());
    const secret = "my-super-secret-data-123!";
    const envelope = await vault.encryptForActiveUser(secret);
    const decrypted = await vault.decryptForActiveUser(envelope);
    if (decrypted !== secret) throw new Error("Vault encrypt/decrypt mismatch");
  }));

  return results;
}

async function auditRouterDispatch(logger, kernelRoot) {
  const results = [];
  const { createRouter } = require(`${kernelRoot}/router`);

  results.push(await runServiceTest("Integration", "router instantiation", () => {
    createRouter({ permissionManager: { can() { return true; } }, auth: { authorize() { return true; } } });
  }));

  return results;
}

module.exports = {
  auditFileWriteReadCycle, auditPermissionAuthIntegration,
  auditVaultEncryptDecryptCycle, auditRouterDispatch,
  label: "Integration",
};
