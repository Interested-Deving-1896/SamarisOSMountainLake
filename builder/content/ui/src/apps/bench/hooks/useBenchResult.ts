import { useState, useEffect, useCallback } from "react";
import type { BenchResult } from "../types/bench";

const benchApi = typeof window !== "undefined" ? window.electronAPI?.bench : undefined;

export function useBenchResult(): {
  result: BenchResult | null;
  loading: boolean;
  refresh: () => void;
} {
  const [result, setResult] = useState<BenchResult | null>(null);
  const [loading, setLoading] = useState(true);

  const refresh = useCallback(async () => {
    if (!benchApi) { setLoading(false); return; }
    setLoading(true);
    try {
      const data = await benchApi.latest() as BenchResult | null;
      setResult(data);
    } catch { setResult(null); }
    setLoading(false);
  }, []);

  useEffect(() => { refresh(); }, [refresh]);
  return { result, loading, refresh };
}
