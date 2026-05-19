const fs = require("node:fs/promises");
const path = require("node:path");
const crypto = require("node:crypto");

const DEFAULT_STATE = {
  guestMode: false,
  lockAfterMinutes: 10,
  locked: true,
  passwordHint: "",
  passwordHash: "",
  passwordSalt: "",
  displayName: "Samaris User",
  username: "user"
};

function normalizePublicState(state) {
  return {
    guestMode: Boolean(state.guestMode),
    lockAfterMinutes: Number(state.lockAfterMinutes || 10),
    locked: Boolean(state.locked),
    passwordHint: String(state.passwordHint || ""),
    displayName: String(state.displayName || "Samaris User"),
    username: String(state.username || "user"),
    hasPassword: Boolean(state.passwordHash)
  };
}

function derivePasswordHash(password, salt) {
  return crypto.scryptSync(String(password), String(salt), 64).toString("hex");
}

class SessionFeaturesService {
  constructor(logger, storage, userService) {
    this.logger = logger;
    this.storage = storage;
    this.userService = userService || null;
    this.stateFile = path.resolve(__dirname, "../../.volt/system/session-security.json");
  }

  async load() {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      return { ...DEFAULT_STATE, ...JSON.parse(raw) };
    } catch {
      return { ...DEFAULT_STATE };
    }
  }

  async save(state) {
    await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
    await fs.writeFile(this.stateFile, JSON.stringify(state, null, 2), "utf8");
  }

  async get() {
    return normalizePublicState(await this.load());
  }

  async setPassword(password) {
    const salt = crypto.randomBytes(16).toString("hex");
    return {
      passwordHash: derivePasswordHash(password, salt),
      passwordSalt: salt
    };
  }

  async set(payload = {}) {
    try {
      const state = await this.load();
      const next = {
        ...state,
        ...payload
      };

      if (Object.prototype.hasOwnProperty.call(payload, "password")) {
        const credentials = await this.setPassword(String(payload.password || ""));
        next.passwordHash = credentials.passwordHash;
        next.passwordSalt = credentials.passwordSalt;
      }

      delete next.password;
      await this.save(next);
      return normalizePublicState(next);
    } catch (error) {
      this.logger.error("sessionFeatures:set_failed", error?.message);
      // Return minimal in-memory state so onboarding can continue even if disk is read-only
      return normalizePublicState({
        ...DEFAULT_STATE,
        ...(payload || {}),
        passwordHash: "unpersisted",
        passwordSalt: "unpersisted"
      });
    }
  }

  async lock() {
    const state = await this.load();
    const next = { ...state, locked: true };
    await this.save(next);
    return normalizePublicState(next);
  }

  async unlock(payload = {}) {
    const state = await this.load();
    const candidate = String(payload.password || "");
    const username = String(payload.username || state.username || "");

    // If userService is available, delegate auth to it
    if (this.userService) {
      const user = await this.userService.login(username, candidate);
      if (!user) {
        return { ok: false, locked: true, message: "Invalid credentials" };
      }
      // Update fileSystem to point to the active user's home
      if (this.userService._activeUser) {
        const fs = require("../core/kernel").fileSystem;
        // The fs reference is not directly accessible here. The caller should
        // call fileSystem.setActiveUser() after a successful unlock.
      }
      const next = { ...state, locked: false };
      await this.save(next);
      return { ok: true, ...normalizePublicState(next), user };
    }

    // Fallback: legacy single-user auth (preserved for backward compat)
    let passwordMatches = false;
    if (state.passwordHash && state.passwordSalt) {
      const candidateHash = derivePasswordHash(candidate, state.passwordSalt);
      const left = Buffer.from(candidateHash, "hex");
      const right = Buffer.from(state.passwordHash, "hex");
      passwordMatches = left.length === right.length && crypto.timingSafeEqual(left, right);
    } else {
      passwordMatches = true;
    }
    if (!passwordMatches) return { ok: false, locked: true, message: "Invalid password" };
    const next = { ...state, locked: false };
    await this.save(next);
    return { ok: true, ...normalizePublicState(next) };
  }
}

module.exports = SessionFeaturesService;
