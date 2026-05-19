const os = require("node:os");
const { execFile } = require("node:child_process");
const { promisify } = require("node:util");

const execFileAsync = promisify(execFile);

class DiskService {
  constructor(logger) {
    this.logger = logger;
    this.platform = os.platform();
  }

  async getStorage() {
    try {
      const { stdout } = await execFileAsync("df", ["-h"]);
      const lines = stdout.trim().split("\n");
      if (lines.length < 2) return [];
      const header = lines[0].toLowerCase();
      const filesystemIdx = header.includes("filesystem") ? 0 : -1;
      const sizeIdx = header.includes("size") ? header.split(/\s+/).findIndex((s) => s === "size") : -1;
      const usedIdx = header.includes("used") ? header.split(/\s+/).findIndex((s) => s === "used") : -1;
      const availIdx = header.includes("avail") ? header.split(/\s+/).findIndex((s) => s === "avail") : -1;
      const usePctIdx = header.includes("use%") || header.includes("capacity")
        ? header.split(/\s+/).findIndex((s) => s === "use%" || s === "capacity" || s === "use%")
        : -1;
      const mountedIdx = header.includes("mounted") ? header.split(/\s+/).findIndex((s) => s === "mounted") : -1;

      return lines.slice(1).map((line) => {
        const parts = line.split(/\s+/);
        return {
          filesystem: parts[filesystemIdx] || "",
          size: parts[sizeIdx] || "",
          used: parts[usedIdx] || "",
          avail: parts[availIdx] || "",
          usePercent: parts[usePctIdx] || "",
          mounted: parts[mountedIdx] || ""
        };
      });
    } catch (error) {
      this.logger.error("disk:getStorage", error instanceof Error ? error.message : String(error));
      return [];
    }
  }

  async listDisks() {
    try {
      if (this.platform === "darwin") {
        const { stdout } = await execFileAsync("diskutil", ["list"]);
        const lines = stdout.trim().split("\n");
        const disks = [];
        let currentDisk = null;
        for (const line of lines) {
          if (/^\/dev\//.test(line.trim())) {
            if (currentDisk) disks.push(currentDisk);
            currentDisk = { device: line.trim(), partitions: [] };
          } else if (currentDisk && line.trim()) {
            const parts = line.trim().split(/\s+/);
            if (parts.length >= 3) {
              currentDisk.partitions.push({
                name: parts[0] || "",
                size: parts[1] || "",
                identifier: parts.slice(2).join(" ") || ""
              });
            }
          }
        }
        if (currentDisk) disks.push(currentDisk);
        return disks;
      }
      const { stdout } = await execFileAsync("lsblk", ["-J", "-o", "NAME,PATH,TYPE,SIZE,FSTYPE,MOUNTPOINT,MODEL"]);
      const tree = JSON.parse(stdout || "{}");
      const entries = [];
      const visit = (node, parentDisk = null) => {
        entries.push({
          name: node.NAME || node.name || "",
          path: node.PATH || node.path || "",
          type: node.TYPE || node.type || "",
          size: node.SIZE || node.size || "",
          fstype: node.FSTYPE || node.fstype || "",
          mountpoint: node.MOUNTPOINT || node.mountpoint || "",
          model: node.MODEL || node.model || "",
          parent: parentDisk
        });
        const nextParent = (node.type || node.TYPE) === "disk" ? (node.path || node.PATH) : parentDisk;
        for (const child of node.children || []) visit(child, nextParent);
      };
      if (Array.isArray(tree.blockdevices)) {
        for (const node of tree.blockdevices) visit(node, null);
      }
      return entries;
    } catch (error) {
      this.logger.error("disk:listDisks", error instanceof Error ? error.message : String(error));
      return [];
    }
  }
}

module.exports = DiskService;
