const fs = require("node:fs/promises");
const path = require("node:path");
const os = require("node:os");
const crypto = require("node:crypto");

const STATE_ROOT = process.env.SAMARIS_STATE_ROOT || path.join(os.homedir(), ".volt");
const USERS_DIR = path.join(STATE_ROOT, "users");
const LAST_LOGIN_FILE = path.join(USERS_DIR, ".last-login");

const HOME_FOLDERS = [
  "Desktop", "Documents", "Downloads", "Music",
  "Photos", "Pictures", "Trash", "Videos",
];

// Use a random guest user ID on each boot; never persisted.
let _guestId = 0;
function nextGuestId() {
  _guestId += 1;
  return `guest-${Date.now().toString(36)}-${_guestId}`;
}

function deriveHash(password, salt) {
  return crypto.scryptSync(String(password), String(salt), 64).toString("hex");
}

function sanitizeUsername(raw) {
  return String(raw || "").trim().toLowerCase().replace(/[^a-z0-9_-]/g, "");
}

class UserService {
  constructor(logger) {
    this.logger = logger;
    this._activeUser = null;
    this._vaultSecret = null;
    this._guestMode = false;
  }

  // ── Internal helpers ──────────────────────────────────

  async _ensureDirs(username) {
    const userDir = path.join(USERS_DIR, username);
    await fs.mkdir(path.join(userDir, "home"), { recursive: true });
    for (const folder of HOME_FOLDERS) {
      await fs.mkdir(path.join(userDir, "home", folder), { recursive: true });
    }
    return userDir;
  }

  _profilePath(username) {
    return path.join(USERS_DIR, username, "profile.json");
  }

  async _readProfile(username) {
    try {
      const raw = await fs.readFile(this._profilePath(username), "utf8");
      return JSON.parse(raw);
    } catch {
      return null;
    }
  }

  async _writeProfile(username, profile) {
    await fs.mkdir(path.dirname(this._profilePath(username)), { recursive: true });
    await fs.writeFile(this._profilePath(username), JSON.stringify(profile, null, 2), "utf8");
  }

  async _readLastLogin() {
    try {
      return (await fs.readFile(LAST_LOGIN_FILE, "utf8")).trim();
    } catch {
      return null;
    }
  }

  async _writeLastLogin(username) {
    await fs.mkdir(path.dirname(LAST_LOGIN_FILE), { recursive: true });
    await fs.writeFile(LAST_LOGIN_FILE, username, "utf8");
  }

  // ── Public API ────────────────────────────────────────

  async create(username, displayName, password) {
    const sanitized = sanitizeUsername(username);
    if (!sanitized) throw Object.assign(new Error("Invalid username"), { code: "invalid_username" });
    if (await this._readProfile(sanitized)) throw Object.assign(new Error("User already exists"), { code: "user_exists" });

    const salt = crypto.randomBytes(16).toString("hex");
    const profile = {
      username: sanitized,
      displayName: String(displayName || sanitized).trim() || sanitized,
      passwordHash: password ? deriveHash(password, salt) : "",
      passwordSalt: password ? salt : "",
      createdAt: new Date().toISOString(),
      guest: false,
    };

    await this._ensureDirs(sanitized);
    await this._writeProfile(sanitized, profile);
    this.logger.info("user:created", { username: sanitized });
    return this._sanitize(profile);
  }

  async authenticate(username, password) {
    if (this._guestMode) {
      return null;
    }

    const sanitized = sanitizeUsername(username);
    if (!sanitized) return null;
    const profile = await this._readProfile(sanitized);
    if (!profile) return null;

    if (!profile.passwordHash) {
      return null;
    }

    const candidateHash = deriveHash(String(password || ""), profile.passwordSalt);
    const left = Buffer.from(candidateHash, "hex");
    const right = Buffer.from(profile.passwordHash, "hex");
    if (left.length !== right.length) return null;

    const match = crypto.timingSafeEqual(left, right);
    if (!match) return null;

    this._activeUser = { ...profile };
    this._vaultSecret = Buffer.from(String(password || ""), "utf8");
    await this._writeLastLogin(sanitized);
    return this._sanitize(profile);
  }

  async login(username, password) {
    const result = await this.authenticate(username, password);
    if (!result) throw Object.assign(new Error("Invalid credentials"), { code: "auth_failed" });
    return result;
  }

  async loginLast() {
    const last = await this._readLastLogin();
    if (last) {
      const profile = await this._readProfile(last);
      if (profile) {
        this._activeUser = { ...profile };
        this._vaultSecret = null;
        return this._sanitize(profile);
      }
    }
    return null;
  }

  setGuestMode(enabled) {
    this._guestMode = Boolean(enabled);
  }

  getActiveUser() {
    return this._activeUser ? this._sanitize(this._activeUser) : null;
  }

  getVaultIdentity() {
    if (!this._activeUser || !this._vaultSecret) return null;
    return {
      username: this._activeUser.username,
      secret: this._vaultSecret.toString("utf8"),
    };
  }

  clearSession() {
    if (this._vaultSecret) {
      this._vaultSecret.fill(0);
    }
    this._activeUser = null;
    this._vaultSecret = null;
  }

  async list() {
    let entries;
    try {
      entries = await fs.readdir(USERS_DIR, { withFileTypes: true });
    } catch {
      return [];
    }

    const users = [];
    for (const entry of entries) {
      if (!entry.isDirectory()) continue;
      if (entry.name.startsWith(".")) continue;
      const profile = await this._readProfile(entry.name);
      if (profile) users.push(this._sanitize(profile));
    }
    return users;
  }

  async delete(username) {
    const sanitized = sanitizeUsername(username);
    if (!sanitized) throw Object.assign(new Error("Invalid username"), { code: "invalid_username" });
    const userDir = path.join(USERS_DIR, sanitized);
    await fs.rm(userDir, { recursive: true, force: true });
    this.logger.info("user:deleted", { username: sanitized });
  }

  async updateProfile(username, updates) {
    const sanitized = sanitizeUsername(username);
    if (!sanitized) return null;
    const profile = await this._readProfile(sanitized);
    if (!profile) return null;

    const next = { ...profile };
    if (updates.displayName) next.displayName = String(updates.displayName).trim();
    if (updates.password) {
      const salt = crypto.randomBytes(16).toString("hex");
      next.passwordHash = deriveHash(updates.password, salt);
      next.passwordSalt = salt;
    }
    if (updates.password === "") {
      next.passwordHash = "";
      next.passwordSalt = "";
    }

    await this._writeProfile(sanitized, next);
    if (this._activeUser?.username === sanitized) {
      this._activeUser = next;
      if (updates.password) {
        if (this._vaultSecret) this._vaultSecret.fill(0);
        this._vaultSecret = Buffer.from(String(updates.password), "utf8");
      }
      if (updates.password === "") {
        if (this._vaultSecret) this._vaultSecret.fill(0);
        this._vaultSecret = null;
      }
    }
    return this._sanitize(next);
  }

  resolveHome(username) {
    const sanitized = sanitizeUsername(username);
    return path.join(USERS_DIR, sanitized || "_unknown", "home");
  }

  resolveUserPath(virtualPath) {
    // /User/... → .users/{active}/home/...
    const active = this._activeUser;
    if (!active) return virtualPath; // fallback
    const home = this.resolveHome(active.username);
    if (virtualPath.startsWith("/User/")) {
      const rest = virtualPath.slice(6); // "/User/" length
      return path.join(home, rest);
    }
    if (virtualPath === "/User") return home;
    return virtualPath;
  }

  _sanitize(profile) {
    return {
      username: profile.username,
      displayName: profile.displayName || profile.username,
      guest: Boolean(profile.guest),
      hasPassword: Boolean(profile.passwordHash),
      createdAt: profile.createdAt || null,
    };
  }
}

module.exports = UserService;
