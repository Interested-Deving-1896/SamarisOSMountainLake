// VOLT Display Manager — Kernel A Handler
//
// Bridges display state changes from VDM to Kernel A / React UI.
// VDM writes events to /run/samaris/display.event.json.
// Kernel A reads these events and forwards them to the UI.
//
// For Alpha: simple file-read + JSON parse.
// For Beta: SBP bridge over Unix socket (direct VDM ↔ Kernel A).

const fs = require("node:fs");
const path = require("node:path");

const DISPLAY_EVENT_PATH = "/run/samaris/display.event.json";

function readDisplayEvent() {
  try {
    if (!fs.existsSync(DISPLAY_EVENT_PATH)) return null;
    const raw = fs.readFileSync(DISPLAY_EVENT_PATH, "utf8");
    return JSON.parse(raw);
  } catch {
    return null;
  }
}

async function handle(message, kernel, context = {}) {
  switch (message.type) {
    case "display.status":
      return {
        type: "display.status.result",
        data: readDisplayEvent() || { type: "display.ready", safe_mode: true }
      };

    case "display.reapply":
      // Signal VDM to re-detect and apply (triggered by safe_mode or user request)
      // For Alpha: VDM is --watch enabled, so it auto-reconfigures on hotplug.
      // The reapply signal is informational for now.
      return {
        type: "display.reapply.result",
        data: { ok: true }
      };

    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
