const fs = require("node:fs/promises");
const path = require("node:path");

const ALPHA_ONE_ENCRYPTION_DISABLED_WARNING =
  "Encryption is temporarily disabled in Alpha One to simplify public testing.";

const DEFAULT_STATE = {
  completed: false,
  currentStep: "welcome",
  licenseAccepted: false,
  accountCreated: false,
  fullName: "",
  username: "",
  setup: {
    finished: false,
    encryptionAvailable: false,
    encrypted: false,
    encryptionEnabled: false,
    encryptionConfigured: false,
    encryptionStatus: "disabled-alpha",
    limitation: ALPHA_ONE_ENCRYPTION_DISABLED_WARNING
  }
};

const VALID_STEPS = new Set(["welcome", "intro", "license", "account", "encryption", "final"]);

class OnboardingService {
  constructor(logger, sessionFeatures, encryption, storage) {
    this.logger = logger;
    this.sessionFeatures = sessionFeatures;
    this.encryption = encryption;
    this.storage = storage;
    this.stateFile = path.resolve(__dirname, "../../.volt/system/onboarding.json");
  }

  async load() {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      const parsed = JSON.parse(raw);
      return {
        ...DEFAULT_STATE,
        ...parsed,
        setup: {
          ...DEFAULT_STATE.setup,
          ...(parsed && parsed.setup ? parsed.setup : {})
        }
      };
    } catch {
      return { ...DEFAULT_STATE };
    }
  }

  async save(next) {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      await fs.writeFile(this.stateFile, JSON.stringify(next, null, 2), "utf8");
    } catch (error) {
      this.logger.error("onboarding:save_failed", error?.message);
      // Continue — state is valid in memory even if not persisted to disk
    }
    return next;
  }

  async get() {
    return await this.load();
  }

  async patch(payload = {}) {
    const current = await this.load();
    const next = {
      ...current,
      ...payload,
      currentStep: VALID_STEPS.has(payload.currentStep) ? payload.currentStep : current.currentStep,
      setup: {
        ...current.setup,
        ...(payload.setup || {})
      }
    };
    return await this.save(next);
  }

  async createAccount(payload = {}) {
    try {
      const fullName = String(payload.fullName || "").trim();
      const username = String(payload.username || "")
        .trim()
        .toLowerCase();
      const password = String(payload.password || "");

      if (fullName.length < 2) {
        return { ok: false, code: "invalid_full_name", message: "Please enter your full name." };
      }

      if (!/^[a-z0-9][a-z0-9._-]{1,31}$/i.test(username)) {
        return { ok: false, code: "invalid_username", message: "Username must use letters, numbers, dots, dashes, or underscores." };
      }

      if (password.length < 4) {
        return { ok: false, code: "weak_password", message: "Password must contain at least 4 characters." };
      }

      await this.sessionFeatures.set({
        displayName: fullName,
        username,
        password,
        guestMode: false,
        locked: true
      });

      const next = await this.patch({
        accountCreated: true,
        fullName,
        username
      });

      return { ok: true, state: next };
    } catch (error) {
      this.logger.error("createAccount:failed", { error: error?.message, stack: error?.stack });
      throw error;
    }
  }

  async evaluateSetup(payload = {}) {
    const storage = await this.storage.setupFirstBoot({
      password: String(payload.password || ""),
      username: payload.username,
      fullName: payload.fullName
    }).catch((error) => ({
      ok: false,
      dryRun: true,
      message: error && error.message ? error.message : "Storage setup skipped for Alpha One."
    }));

    const encryption = {
      available: false,
      encrypted: false,
      note: ALPHA_ONE_ENCRYPTION_DISABLED_WARNING,
      status: "disabled-alpha"
    };
    const next = await this.patch({
      setup: {
        finished: true,
        encryptionAvailable: false,
        encrypted: false,
        encryptionEnabled: false,
        encryptionConfigured: false,
        encryptionStatus: "disabled-alpha",
        limitation: ALPHA_ONE_ENCRYPTION_DISABLED_WARNING
      }
    });

    return {
      ok: true,
      state: next,
      encryption,
      storage
    };
  }

  async complete() {
    return await this.patch({
      completed: true,
      currentStep: "final"
    });
  }

  async reset() {
    return await this.save({ ...DEFAULT_STATE });
  }
}

module.exports = OnboardingService;
