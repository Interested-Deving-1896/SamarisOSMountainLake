import { useState, useCallback } from "react";
import type { BenchmarkMode } from "../types/bench";

const benchApi = typeof window !== "undefined" ? window.electronAPI?.bench : undefined;
const isDev = !benchApi;

export function useBenchRun(onComplete?: () => void): {
  running: boolean;
  run: (mode: BenchmarkMode) => void;
  devMode: boolean;
} {
  const [running, setRunning] = useState(false);

  const run = useCallback(async (mode: BenchmarkMode) => {
    if (isDev) {
      const cmd = `bench --${mode}`;
      try { await navigator.clipboard.writeText(cmd); } catch {}
      return;
    }
    setRunning(true);
    try { await benchApi.run(mode); onComplete?.(); }
    catch { /* failed */ }
    setRunning(false);
  }, [onComplete]);

  return { running, run, devMode: isDev };
}
