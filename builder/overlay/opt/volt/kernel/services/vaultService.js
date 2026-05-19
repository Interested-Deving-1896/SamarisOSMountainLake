const crypto = require("node:crypto");

const ALGORITHM = "aes-256-gcm";
const KDF = "scrypt";
const VERSION = 1;

function asBuffer(hex, label) {
  if (!hex || typeof hex !== "string") {
    const error = new Error(`${label}_missing`);
    error.code = "VAULT_INVALID_ENVELOPE";
    throw error;
  }
  return Buffer.from(hex, "hex");
}

class VaultService {
  constructor(logger, userService, kernelB = null) {
    this.logger = logger;
    this.userService = userService;
    this.kernelB = kernelB;
    this._keyCache = new Map();
  }

  requireIdentity() {
    const identity = this.userService?.getVaultIdentity?.();
    if (!identity?.username || !identity?.secret) {
      const error = new Error("vault_locked");
      error.code = "VAULT_LOCKED";
      throw error;
    }
    return identity;
  }

  async encryptForActiveUser(plaintext) {
    const identity = this.requireIdentity();
    const text = String(plaintext || "");
    return { ...this.encryptString(text, identity.secret), owner: identity.username };
  }

  async decryptForActiveUser(envelope) {
    const identity = this.requireIdentity();
    if (!envelope) return "";
    return this.decryptString(envelope, identity.secret);
  }

  encryptString(plaintext, password) {
    const salt = crypto.randomBytes(16);
    const iv = crypto.randomBytes(12);
    const key = this._deriveKey(String(password), salt);
    const cipher = crypto.createCipheriv(ALGORITHM, key, iv);
    const encrypted = Buffer.concat([cipher.update(String(plaintext), "utf8"), cipher.final()]);
    return {
      version: VERSION,
      algorithm: ALGORITHM,
      kdf: KDF,
      salt: salt.toString("hex"),
      iv: iv.toString("hex"),
      data: encrypted.toString("hex"),
      authTag: cipher.getAuthTag().toString("hex")
    };
  }

  decryptString(envelope, password) {
    if (envelope.version !== VERSION || envelope.algorithm !== ALGORITHM || envelope.kdf !== KDF) {
      const error = new Error("vault_unsupported_envelope");
      error.code = "VAULT_INVALID_ENVELOPE";
      throw error;
    }
    const salt = asBuffer(envelope.salt, "salt");
    const key = this._deriveKey(String(password), salt);
    const decipher = crypto.createDecipheriv(ALGORITHM, key, asBuffer(envelope.iv, "iv"));
    decipher.setAuthTag(asBuffer(envelope.authTag, "authTag"));
    return decipher.update(asBuffer(envelope.data, "data"), undefined, "utf8") + decipher.final("utf8");
  }

  clearKeyCache() {
    this._keyCache.clear();
  }

  _deriveKey(password, salt) {
    const cacheKey = `${password}::${salt.toString("hex")}`;
    const cached = this._keyCache.get(cacheKey);
    if (cached) return cached;
    const key = crypto.scryptSync(password, salt, 32);
    this._keyCache.set(cacheKey, key);
    return key;
  }
}

module.exports = VaultService;
