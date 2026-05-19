import { useState, useEffect } from "react";

const benchApi = typeof window !== "undefined" ? window.electronAPI?.bench : undefined;

export function useBenchComparison(): {
  baselines: string[];
  loading: boolean;
} {
  const [baselines, setBaselines] = useState<string[]>([]);
  const [loading, setLoading] = useState(true);
  useEffect(() => {
    if (!benchApi) { setLoading(false); return; }
    (async () => {
      try { setBaselines(await benchApi.baselines()); }
      catch { setBaselines([]); }
      setLoading(false);
    })();
  }, []);
  return { baselines, loading };
}
