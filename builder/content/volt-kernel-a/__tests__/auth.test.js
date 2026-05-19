const { describe, it } = require("node:test");
const assert = require("node:assert");

const Auth = require("../core/auth");
const PermissionManager = require("../services/permissionManager");

describe("Auth", () => {
  function createKernelMock(permissions) {
    const pm = new PermissionManager({ info() {}, warn() {} });
    if (permissions) {
      pm.seed("test-app", permissions);
    }
    return { permissionManager: pm, auth: null };
  }

  it("denies when appId is missing", () => {
    const kernel = createKernelMock(["fs.read"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "fs.read" }, kernel), false);
  });

  it("denies when appId is not a string", () => {
    const kernel = createKernelMock(["fs.read"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "fs.read", appId: 123 }, kernel), false);
  });

  it("allows exact permission match", () => {
    const kernel = createKernelMock(["fs.read", "fs.write"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "fs.read", appId: "test-app" }, kernel), true);
  });

  it("denies unmatched permission", () => {
    const kernel = createKernelMock(["fs.read"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "fs.write", appId: "test-app" }, kernel), false);
  });

  it("allows wildcard namespace '*.*'", () => {
    const kernel = createKernelMock(["fs.*"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "fs.read", appId: "test-app" }, kernel), true);
    assert.strictEqual(auth.authorize({ type: "fs.write", appId: "test-app" }, kernel), true);
    assert.strictEqual(auth.authorize({ type: "fs.list", appId: "test-app" }, kernel), true);
  });

  it("allows global wildcard '*'", () => {
    const kernel = createKernelMock(["*"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "anything.here", appId: "test-app" }, kernel), true);
  });

  it("allows deep namespace wildcard", () => {
    const kernel = createKernelMock(["media.*"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "media.musicLibrary", appId: "test-app" }, kernel), true);
    assert.strictEqual(auth.authorize({ type: "media.videoLibrary", appId: "test-app" }, kernel), true);
  });

  it("denies partial namespace match", () => {
    const kernel = createKernelMock(["fs.*"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "media.read", appId: "test-app" }, kernel), false);
  });

  it("checks namespace wildcard as fallback", () => {
    const kernel = createKernelMock(["system.*"]);
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ type: "system.ping", appId: "test-app" }, kernel), true);
    assert.strictEqual(auth.authorize({ type: "system.info", appId: "test-app" }, kernel), true);
  });

  it("handles empty type", () => {
    const kernel = createKernelMock();
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize({ appId: "test-app" }, kernel), false);
  });

  it("handles null message", () => {
    const kernel = createKernelMock();
    const auth = new Auth({ warn() {} });
    assert.strictEqual(auth.authorize(null, kernel), false);
  });
});
