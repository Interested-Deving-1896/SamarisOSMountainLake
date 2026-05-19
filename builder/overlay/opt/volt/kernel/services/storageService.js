const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const { spawn } = require("node:child_process");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

const USER_DIRECTORIES = [
  "Desktop",
  "Documents",
  "Downloads",
  "Pictures",
  "Music",
  "Videos",
  "Applications",
  "AppData",
  "Trash",
  ".samaris"
];

class StorageService {
  constructor(logger, eventBus, fileSystem) {
    this.logger = logger;
    this.eventBus = eventBus;
    this.fileSystem = fileSystem;
    this.rootPath = path.resolve(__dirname, "../..");
    this.defaultUserRoot = path.join(this.rootPath, ".volt", "user");
    this.defaultVolumesRoot = path.join(this.rootPath, ".volt", "volumes");
    this.stateDir = process.env.SAMARIS_STATE_DIR
      ? path.resolve(process.env.SAMARIS_STATE_DIR)
      : path.join(this.rootPath, ".volt", "system");
    this.stateFile = path.join(this.stateDir, "storage-state.json");
    this.firstBootFlag = path.join(this.stateDir, "firstboot.done");
    this.mountPoint = process.env.SAMARIS_USER_MOUNT || "/mnt/samaris-user";
    this.mappingName = process.env.SAMARIS_LUKS_NAME || "samaris-user";
    this.liveMode = os.platform() === "linux" && process.env.SAMARIS_STORAGE_MODE === "live";
  }

  defaultState() {
    return {
      mode: this.liveMode ? "live" : "dry-run",
      firstBootCompleted: false,
      mounted: false,
      encrypted: false,
      userRoot: this.defaultUserRoot,
      bootDevicePath: "",
      userPartitionPath: "",
      mappingName: this.mappingName,
      lastError: "",
      lastSetupAt: "",
      lastUnlockAt: ""
    };
  }

  async init() {
    const state = await this.loadState();
    await fs.mkdir(this.defaultUserRoot, { recursive: true });
    await fs.mkdir(this.defaultVolumesRoot, { recursive: true });
    this.fileSystem.setUserRoot(state.userRoot || this.defaultUserRoot);
    await this.refreshExternalBindings();
    return state;
  }

  async loadState() {
    try {
      await fs.mkdir(this.stateDir, { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      return { ...this.defaultState(), ...JSON.parse(raw) };
    } catch {
      return this.defaultState();
    }
  }

  async saveState(next) {
    await fs.mkdir(this.stateDir, { recursive: true });
    await fs.writeFile(this.stateFile, JSON.stringify(next, null, 2), "utf8");
    return next;
  }

  async writeFirstBootFlag() {
    await fs.mkdir(this.stateDir, { recursive: true });
    await fs.writeFile(this.firstBootFlag, "done\n", "utf8");
  }

  async ensureUserDirectories(basePath) {
    await fs.mkdir(basePath, { recursive: true });
    await Promise.all(USER_DIRECTORIES.map((folder) => fs.mkdir(path.join(basePath, folder), { recursive: true })));
  }

  async runExec(command, args, options = {}) {
    this.logger.info("storage:exec", { command, args });
    return await execFileAsync(command, args, options);
  }

  async runExecWithStdin(command, args, input, options = {}) {
    this.logger.info("storage:exec", { command, args });
    return await new Promise((resolve, reject) => {
      const child = spawn(command, args, {
        ...options,
        stdio: ["pipe", "pipe", "pipe"]
      });
      const stdout = [];
      const stderr = [];
      child.stdout.on("data", (chunk) => stdout.push(Buffer.from(chunk)));
      child.stderr.on("data", (chunk) => stderr.push(Buffer.from(chunk)));
      child.on("error", reject);
      child.on("close", (code) => {
        if (code === 0) {
          resolve({
            stdout: Buffer.concat(stdout).toString("utf8"),
            stderr: Buffer.concat(stderr).toString("utf8")
          });
          return;
        }
        const error = new Error(Buffer.concat(stderr).toString("utf8") || `${command} exited with code ${code}`);
        error.code = code;
        reject(error);
      });
      child.stdin.write(String(input || ""));
      child.stdin.end();
    });
  }

  async detectBootTarget() {
    if (!this.liveMode) {
      return {
        ok: true,
        mode: "dry-run",
        diskPath: "",
        sysPartitionPath: "",
        userPartitionPath: "",
        note: "Storage provisioning runs in safe dry-run mode on this environment."
      };
    }

    const { stdout } = await this.runExec("lsblk", ["-J", "-o", "NAME,PATH,PKNAME,TYPE,LABEL,RM,SIZE,FSTYPE,MOUNTPOINT"]);
    const tree = JSON.parse(stdout || "{}");
    const entries = [];
    const visit = (node, parentDisk = null) => {
      const current = {
        name: node.NAME || node.name || "",
        path: node.PATH || node.path || "",
        parent: parentDisk,
        type: node.TYPE || node.type || "",
        label: node.LABEL || node.label || "",
        removable: String(node.RM || node.rm || "0") === "1",
        size: node.SIZE || node.size || "",
        mountpoint: node.MOUNTPOINT || node.mountpoint || "",
        fstype: node.FSTYPE || node.fstype || ""
      };
      entries.push(current);
      const nextParent = current.type === "disk" ? current.path : parentDisk;
      for (const child of node.children || []) {
        visit(child, nextParent);
      }
    };
    for (const node of tree.blockdevices || []) {
      visit(node, null);
    }

    const sysPartition = entries.find((entry) => entry.type === "part" && entry.label === "samaris-sys");
    const userPartition = entries.find((entry) => entry.type === "part" && entry.label === "samaris-user");
    if (!sysPartition || !userPartition || !sysPartition.parent || sysPartition.parent !== userPartition.parent) {
      return {
        ok: false,
        mode: "live",
        reason: "boot_target_uncertain",
        note: "Samaris could not safely identify the boot USB layout."
      };
    }

    const disk = entries.find((entry) => entry.path === sysPartition.parent && entry.type === "disk");
    if (!disk) {
      return {
        ok: false,
        mode: "live",
        reason: "boot_disk_missing",
        note: "The Samaris boot disk could not be verified."
      };
    }

    if (!disk.removable && process.env.SAMARIS_ALLOW_FIXED_BOOT_MEDIA !== "1") {
      return {
        ok: false,
        mode: "live",
        reason: "boot_disk_not_removable",
        note: "Refusing to modify storage because the boot media is not marked removable."
      };
    }

    return {
      ok: true,
      mode: "live",
      diskPath: disk.path,
      sysPartitionPath: sysPartition.path,
      userPartitionPath: userPartition.path,
      note: `Using ${disk.path} with ${sysPartition.path} / ${userPartition.path}`
    };
  }

  async status() {
    const state = await this.loadState();
    const devices = await this.listExternalDevices();
    return {
      ...state,
      available: this.liveMode || true,
      mode: this.liveMode ? "live" : "dry-run",
      mountPoint: this.mountPoint,
      devices,
      note: this.liveMode
        ? state.lastError || "Live storage management is enabled."
        : "Storage setup is running in safe dry-run mode on this development environment."
    };
  }

  async setupFirstBoot(payload = {}) {
    const password = String(payload.password || "");
    if (password.length < 4) {
      return { ok: false, message: "Password must contain at least 4 characters." };
    }

    const current = await this.loadState();
    if (current.firstBootCompleted) {
      await this.fileSystem.ensureVirtualRoots?.();
      return {
        ok: true,
        state: current,
        detail: "already_configured"
      };
    }

    if (!this.liveMode) {
      await this.ensureUserDirectories(this.defaultUserRoot);
      const next = await this.saveState({
        ...current,
        firstBootCompleted: true,
        mounted: true,
        encrypted: false,
        userRoot: this.defaultUserRoot,
        lastError: "",
        lastSetupAt: new Date().toISOString()
      });
      this.fileSystem.setUserRoot(next.userRoot);
      await this.writeFirstBootFlag();
      await this.refreshExternalBindings();
      return {
        ok: true,
        dryRun: true,
        message: "Storage provisioning completed in safe development mode.",
        state: next
      };
    }

    const target = await this.detectBootTarget();
    if (!target.ok) {
      return { ok: false, message: target.note || "Unable to identify the Samaris USB safely." };
    }

    try {
      await this.runExec("sgdisk", ["-e", target.diskPath]);
      const userPartitionNumber = target.userPartitionPath.replace(target.diskPath, "").replace(/^p?/, "");
      await this.runExec("sgdisk", ["-d", userPartitionNumber, target.diskPath]);
      await this.runExec("sgdisk", ["-n", "2:0:0", "-t", "2:8300", "-c", "2:samaris-user", target.diskPath]);
      await this.runExec("partprobe", [target.diskPath]);
      const resizedUserPartition = `${target.diskPath}${target.diskPath.startsWith("/dev/nvme") ? "p2" : "2"}`;
      await this.runExecWithStdin("cryptsetup", ["luksFormat", "--type", "luks2", resizedUserPartition, "--batch-mode", "--key-file", "-"], `${password}\n`);
      await this.runExecWithStdin("cryptsetup", ["open", resizedUserPartition, this.mappingName, "--key-file", "-"], `${password}\n`);
      await this.runExec("mkfs.ext4", ["-L", "samaris-user", `/dev/mapper/${this.mappingName}`]);
      await fs.mkdir(this.mountPoint, { recursive: true });
      await this.runExec("mount", [`/dev/mapper/${this.mappingName}`, this.mountPoint]);
      await this.ensureUserDirectories(this.mountPoint);

      const next = await this.saveState({
        ...current,
        firstBootCompleted: true,
        mounted: true,
        encrypted: true,
        bootDevicePath: target.diskPath,
        userPartitionPath: resizedUserPartition,
        userRoot: this.mountPoint,
        lastError: "",
        lastSetupAt: new Date().toISOString()
      });
      this.fileSystem.setUserRoot(this.mountPoint);
      await this.writeFirstBootFlag();
      await this.refreshExternalBindings();
      return {
        ok: true,
        state: next,
        dryRun: false,
        message: "Encrypted user storage created successfully."
      };
    } catch (error) {
      const message = error instanceof Error ? error.message : "Storage setup failed.";
      await this.saveState({
        ...current,
        lastError: message
      });
      return { ok: false, message };
    }
  }

  async unlockUserStorage(password) {
    const current = await this.loadState();
    if (!current.firstBootCompleted) {
      return { ok: true, state: current, message: "Storage has not been provisioned yet." };
    }

    if (!this.liveMode) {
      await this.ensureUserDirectories(current.userRoot || this.defaultUserRoot);
      this.fileSystem.setUserRoot(current.userRoot || this.defaultUserRoot);
      await this.refreshExternalBindings();
      const next = await this.saveState({
        ...current,
        mounted: true,
        lastError: "",
        lastUnlockAt: new Date().toISOString()
      });
      return { ok: true, state: next };
    }

    if (!current.userPartitionPath) {
      return { ok: false, message: "The encrypted Samaris user partition is missing." };
    }

    try {
      await fs.mkdir(this.mountPoint, { recursive: true });
      await this.runExecWithStdin(
        "cryptsetup",
        ["open", current.userPartitionPath, this.mappingName, "--key-file", "-"],
        `${String(password || "")}\n`
      ).catch(() => null);
      await this.runExec("mount", [`/dev/mapper/${this.mappingName}`, this.mountPoint]).catch(() => null);
      await this.ensureUserDirectories(this.mountPoint);
      this.fileSystem.setUserRoot(this.mountPoint);
      await this.refreshExternalBindings();
      const next = await this.saveState({
        ...current,
        mounted: true,
        encrypted: true,
        userRoot: this.mountPoint,
        lastError: "",
        lastUnlockAt: new Date().toISOString()
      });
      return { ok: true, state: next };
    } catch (error) {
      const message =
        error instanceof Error && /No key available|incorrect passphrase/i.test(error.message)
          ? "The encrypted Samaris storage could not be unlocked with that password."
          : error instanceof Error
            ? error.message
            : "The encrypted Samaris storage could not be mounted.";
      const next = await this.saveState({
        ...current,
        mounted: false,
        lastError: message
      });
      return { ok: false, state: next, message };
    }
  }

  async listExternalDevices() {
    if (!this.liveMode) {
      return [];
    }

    try {
      const { stdout } = await this.runExec("lsblk", ["-J", "-o", "NAME,PATH,PKNAME,TYPE,LABEL,RM,SIZE,FSTYPE,MOUNTPOINT,MODEL"]);
      const tree = JSON.parse(stdout || "{}");
      const entries = [];
      const visit = (node, parentDisk = null) => {
        const current = {
          name: node.NAME || node.name || "",
          path: node.PATH || node.path || "",
          parent: parentDisk,
          type: node.TYPE || node.type || "",
          label: node.LABEL || node.label || "",
          removable: String(node.RM || node.rm || "0") === "1",
          size: node.SIZE || node.size || "",
          fstype: node.FSTYPE || node.fstype || "",
          mountpoint: node.MOUNTPOINT || node.mountpoint || "",
          model: node.MODEL || node.model || ""
        };
        entries.push(current);
        const nextParent = current.type === "disk" ? current.path : parentDisk;
        for (const child of node.children || []) visit(child, nextParent);
      };
      for (const node of tree.blockdevices || []) visit(node, null);
      return entries
        .filter((entry) => entry.type === "part" && entry.removable)
        .filter((entry) => !["samaris-sys", "samaris-user"].includes(entry.label))
        .map((entry) => ({
          id: entry.label || path.basename(entry.path),
          label: entry.label || entry.model || path.basename(entry.path),
          path: entry.path,
          filesystem: entry.fstype || "unknown",
          size: entry.size || "",
          mounted: Boolean(entry.mountpoint),
          mountPath: entry.mountpoint || "",
          removable: true
        }));
    } catch (error) {
      this.logger.warn("storage:devices", error instanceof Error ? error.message : String(error));
      return [];
    }
  }

  async refreshExternalBindings() {
    const devices = await this.listExternalDevices();
    const mounted = devices
      .filter((entry) => entry.mounted && entry.mountPath)
      .map((entry) => ({
        id: entry.id,
        actualPath: entry.mountPath
      }));
    this.fileSystem.setExternalRoots(mounted);
    return devices;
  }

  async mountExternal(devicePath) {
    if (!this.liveMode) {
      return { ok: false, message: "External device mounting is available only on Linux live builds." };
    }
    try {
      const { stdout } = await this.runExec("udisksctl", ["mount", "-b", String(devicePath)]);
      const devices = await this.refreshExternalBindings();
      return { ok: true, message: stdout.trim() || "Mounted successfully", devices };
    } catch (error) {
      const message = error instanceof Error ? error.message : "Mount failed";
      return { ok: false, message };
    }
  }

  async unmountExternal(devicePath) {
    if (!this.liveMode) {
      return { ok: false, message: "External device eject is available only on Linux live builds." };
    }
    try {
      const { stdout } = await this.runExec("udisksctl", ["unmount", "-b", String(devicePath)]);
      const devices = await this.refreshExternalBindings();
      return { ok: true, message: stdout.trim() || "Safe to remove", devices };
    } catch (error) {
      const message = error instanceof Error ? error.message : "Device is busy";
      return { ok: false, message };
    }
  }
}

module.exports = StorageService;
