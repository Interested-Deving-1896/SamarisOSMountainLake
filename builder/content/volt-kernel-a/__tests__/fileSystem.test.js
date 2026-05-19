const { describe, it, beforeEach, afterEach } = require("node:test");
const assert = require("node:assert");
const path = require("node:path");
const fs = require("node:fs/promises");
const os = require("node:os");

const FileSystemService = require("../services/fileSystem");

const TMP = path.join(os.tmpdir(), `samaris-fs-test-${Date.now()}`);

describe("FileSystemService", () => {
  let fsService;

  beforeEach(async () => {
    await fs.mkdir(TMP, { recursive: true });
    fsService = new FileSystemService(
      { info() {}, warn() {} },
      { emit() {} },
      null, null
    );
    const userRoot = path.join(TMP, ".volt", "user");
    fsService.userRootPath = userRoot;
    fsService.virtualRoots["/User"] = userRoot;
    fsService.initialized = null;
  });

  afterEach(async () => {
    await fs.rm(TMP, { recursive: true, force: true });
  });

  it("toVirtualPath normalizes paths", () => {
    assert.strictEqual(fsService.toVirtualPath("foo"), "/foo");
    assert.strictEqual(fsService.toVirtualPath("/foo/bar"), "/foo/bar");
    assert.strictEqual(fsService.toVirtualPath("\\foo\\bar"), "/foo/bar");
    assert.strictEqual(fsService.toVirtualPath("/"), "/");
    assert.strictEqual(fsService.toVirtualPath(""), "/");
  });

  it("toActualPath resolves User root", () => {
    const result = fsService.toActualPath("/User/Documents/test.txt");
    assert.ok(result.actualPath.endsWith(path.join(".volt", "user", "Documents", "test.txt")));
    assert.strictEqual(result.virtualPath, "/User/Documents/test.txt");
    assert.strictEqual(result.root, "/User");
  });

  it("toActualPath throws ENOENT for unmapped path", () => {
    assert.throws(() => {
      fsService.toActualPath("/Unknown/path");
    }, (err) => err.code === "ENOENT");
  });

  it("toActualPath blocks path traversal via ENOENT", () => {
    assert.throws(() => {
      fsService.toActualPath("/User/Documents/../../../../etc/passwd");
    }, (err) => err.code === "ENOENT");
  });

  it("resolveRoot finds longest matching prefix", () => {
    fsService.externalRoots = {
      "/Volumes/USB": "/mnt/usb"
    };
    const root = fsService.resolveRoot("/Volumes/USB/data");
    assert.ok(root);
    assert.strictEqual(root.prefix, "/Volumes/USB");
  });

  it("list returns User dir at root", async () => {
    const result = await fsService.list("/");
    assert.strictEqual(result.path, "/");
    const userNode = result.nodes.find((n) => n.name === "User");
    assert.ok(userNode);
    assert.strictEqual(userNode.kind, "dir");
  });

  it("list filters dotfiles", async () => {
    const userRoot = fsService.userRootPath;
    await fs.mkdir(path.join(userRoot, "Documents"), { recursive: true });
    await fs.writeFile(path.join(userRoot, "Documents", ".hidden"), "secret");
    await fs.writeFile(path.join(userRoot, "Documents", "visible.txt"), "hello");

    const result = await fsService.list("/User/Documents");
    const names = result.nodes.map((n) => n.name);
    assert.ok(names.includes("visible.txt"));
    assert.ok(!names.includes(".hidden"));
  });

  it("list sorts dirs before files", async () => {
    const userRoot = fsService.userRootPath;
    await fs.mkdir(path.join(userRoot, "Documents"), { recursive: true });
    await fs.writeFile(path.join(userRoot, "Documents", "z.txt"), "z");
    await fs.mkdir(path.join(userRoot, "Documents", "a-dir"), { recursive: true });

    const result = await fsService.list("/User/Documents");
    const kinds = result.nodes.map((n) => n.kind);
    const firstDirIndex = kinds.indexOf("dir");
    const firstFileIndex = kinds.indexOf("file");
    assert.ok(firstDirIndex < firstFileIndex, "dirs should come before files");
  });

  it("write and read round-trips", async () => {
    const content = "Hello Samaris!";
    await fsService.write("/User/Documents/hello.txt", content);
    const result = await fsService.read("/User/Documents/hello.txt");
    assert.strictEqual(result.content, content);
    assert.strictEqual(result.path, "/User/Documents/hello.txt");
  });

  it("mkdir creates directories", async () => {
    await fsService.mkdir("/User/Documents/nested/deep");
    const result = await fsService.list("/User/Documents");
    const dir = result.nodes.find((n) => n.name === "nested");
    assert.ok(dir);
    assert.strictEqual(dir.kind, "dir");
  });

  it("rename moves files within same root", async () => {
    await fsService.write("/User/Documents/old.txt", "old");
    await fsService.rename("/User/Documents/old.txt", "/User/Documents/new.txt");
    const result = await fsService.list("/User/Documents");
    assert.ok(result.nodes.find((n) => n.name === "new.txt"));
    assert.ok(!result.nodes.find((n) => n.name === "old.txt"));
  });

  it("rename blocks cross-root moves", async () => {
    await fsService.write("/User/Documents/x.txt", "x");
    fsService.externalRoots = { "/Volumes/USB": "/mnt/usb" };
    await assert.rejects(
      () => fsService.rename("/User/Documents/x.txt", "/Volumes/USB/x.txt"),
      (err) => err.code === "EACCES"
    );
  });

  it("remove deletes files", async () => {
    await fsService.write("/User/Documents/trash.txt", "trash");
    await fsService.remove("/User/Documents/trash.txt");
    const result = await fsService.list("/User/Documents");
    assert.ok(!result.nodes.find((n) => n.name === "trash.txt"));
  });

  it("setUserRoot switches user home", () => {
    const newRoot = path.join(TMP, "other-user");
    fsService.setUserRoot(newRoot);
    const result = fsService.toActualPath("/User/test.txt");
    assert.ok(result.actualPath.startsWith(newRoot));
  });

  it("toActualPath returns null for root /", () => {
    const result = fsService.toActualPath("/");
    assert.strictEqual(result.actualPath, null);
    assert.strictEqual(result.virtualPath, "/");
  });

  it("toActualPath resolves /Volumes via virtualRoots", () => {
    const result = fsService.toActualPath("/Volumes/USB/data");
    assert.ok(result.actualPath);
    assert.strictEqual(result.root, "/Volumes");
  });
});
