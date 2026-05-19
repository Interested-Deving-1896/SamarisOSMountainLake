const { ipcMain } = require("electron");
const { spawn } = require("node:child_process");
const fs = require("node:fs");
const path = require("node:path");

// Production paths
const PROD_BENCH_BIN = "/opt/volt/bin/bench";
const PROD_DATA_DIR = "/var/lib/samaris/bench";

// Dev paths (relative to this file: electron/ipc/bench.js)
const DEV_BENCH_BIN = path.join(__dirname, "..", "..", "volt-bench", "target", "release", "bench");
const DEV_DATA_DIR = path.join(__dirname, "..", "..", "volt-bench", "target", "bench-data");

// Auto-detect environment
const BENCH_BIN = fs.existsSync(PROD_BENCH_BIN) ? PROD_BENCH_BIN :
                  fs.existsSync(DEV_BENCH_BIN) ? DEV_BENCH_BIN : null;
const BENCH_DATA_DIR = fs.existsSync(PROD_DATA_DIR) ? PROD_DATA_DIR :
                       (() => { try { fs.mkdirSync(DEV_DATA_DIR, { recursive: true }); } catch {} return DEV_DATA_DIR; })();

function benchPath(...parts) {
  return path.join(BENCH_DATA_DIR, ...parts);
}

function readJson(filePath) {
  try {
    if (!fs.existsSync(filePath)) return null;
    return JSON.parse(fs.readFileSync(filePath, "utf8"));
  } catch {
    return null;
  }
}

function writeJson(filePath, data) {
  const dir = path.dirname(filePath);
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
  fs.writeFileSync(filePath, JSON.stringify(data, null, 2), "utf8");
}

function spawnBench(args, mainWindow) {
  return new Promise((resolve, reject) => {
    if (!BENCH_BIN) {
      reject(new Error("Bench binary not found. Build it: cd builder/content/volt-bench && cargo build --release"));
      return;
    }

    const child = spawn(BENCH_BIN, args, {
      stdio: ["ignore", "pipe", "pipe"],
      timeout: 5 * 60 * 1000,
      env: { ...process.env, BENCH_STORAGE_PATH: BENCH_DATA_DIR },
    });

    let stdout = "";
    let stderr = "";

    child.stdout.on("data", (chunk) => {
      stdout += chunk.toString();
      if (mainWindow) {
        mainWindow.webContents.send("bench:progress", { text: chunk.toString().trim() });
      }
    });

    child.stderr.on("data", (chunk) => { stderr += chunk.toString(); });

    child.on("close", (code) => {
      const result = readJson(benchPath("latest.json"));
      if (mainWindow) {
        mainWindow.webContents.send("bench:complete", { code, result });
      }
      if (code === 0) {
        resolve(result || { ok: true });
      } else {
        reject(new Error(stderr.trim() || `Exit code ${code}`));
      }
    });

    child.on("error", (err) => {
      if (mainWindow) {
        mainWindow.webContents.send("bench:complete", { code: -1, error: err.message });
      }
      reject(err);
    });
  });
}

function registerBench(mainWindow) {
  ipcMain.handle("bench:latest", async () => readJson(benchPath("latest.json")));

  ipcMain.handle("bench:history", async () => {
    return readJson(benchPath("history.json")) || { entries: [], max_entries: 100 };
  });

  ipcMain.handle("bench:baselines", async () => {
    const dir = benchPath("baselines");
    if (!fs.existsSync(dir)) return [];
    return fs.readdirSync(dir).filter((f) => f.endsWith(".json"));
  });

  ipcMain.handle("bench:run", async (_, mode) => {
    const args = mode === "ci" ? ["--ci"] :
                 mode === "stress" ? ["--stress"] :
                 mode === "full" ? ["--full"] :
                 ["--quick"];
    return spawnBench(args, mainWindow);
  });

  ipcMain.handle("bench:import-baseline", async (_, filePath) => {
    if (!fs.existsSync(filePath)) throw new Error("File not found");
    const data = JSON.parse(fs.readFileSync(filePath, "utf8"));
    const name = path.basename(filePath, ".json");
    writeJson(benchPath("baselines", `${name}.json`), data);
    return { ok: true, name };
  });

  ipcMain.handle("bench:export-json", async () => {
    const latest = readJson(benchPath("latest.json"));
    if (!latest) throw new Error("No benchmark data");
    const dir = benchPath("export");
    if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
    const filePath = path.join(dir, `bench-export-${Date.now()}.json`);
    writeJson(filePath, latest);
    return { ok: true, path: filePath };
  });

  ipcMain.handle("bench:export-csv", async () => {
    const latest = readJson(benchPath("latest.json"));
    if (!latest) throw new Error("No benchmark data");
    const dir = benchPath("export");
    if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
    const filePath = path.join(dir, `bench-export-${Date.now()}.csv`);
    let csv = "category,metric,value\n";
    if (latest.overall) {
      csv += `overall,score,${latest.overall.score}\n`;
      csv += `overall,normalized,${latest.overall.normalized_score}\n`;
      csv += `overall,badge,${latest.overall.badge}\n`;
    }
    if (latest.category_scores) {
      for (const [cat, score] of Object.entries(latest.category_scores)) {
        csv += `category,${cat},${score}\n`;
      }
    }
    fs.writeFileSync(filePath, csv, "utf8");
    return { ok: true, path: filePath };
  });

  ipcMain.handle("bench:optimizer-input", async () => {
    const latest = readJson(benchPath("latest.json"));
    if (!latest) throw new Error("No benchmark data");
    const optInput = {
      fitness_score: latest.overall?.score || 0,
      bottlenecks: latest.optimizer?.bottlenecks || [],
      recommendations: latest.optimizer?.recommendations || [],
      unstable_metrics: [],
      regression_alerts: [],
      run_id: latest.run?.run_id || "unknown",
      hardware_class: latest.hardware?.class || "unknown",
    };
    writeJson(benchPath("optimizer-input.json"), optInput);
    return optInput;
  });
}

module.exports = { registerBench };
