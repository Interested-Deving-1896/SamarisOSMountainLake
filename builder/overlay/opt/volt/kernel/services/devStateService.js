const fs = require("node:fs/promises");
const path = require("node:path");

const DEFAULT_STATE = {
  token: "",
  updatedAt: null
};

class DevStateService {
  constructor(logger) {
    this.logger = logger;
    this.stateFile = path.resolve(__dirname, "../../.volt/system/dev-reset.json");
  }

  async getResetState() {
    try {
      await fs.mkdir(path.dirname(this.stateFile), { recursive: true });
      const raw = await fs.readFile(this.stateFile, "utf8");
      const parsed = JSON.parse(raw);
      return {
        ...DEFAULT_STATE,
        ...parsed
      };
    } catch {
      return { ...DEFAULT_STATE };
    }
  }
}

module.exports = DevStateService;
