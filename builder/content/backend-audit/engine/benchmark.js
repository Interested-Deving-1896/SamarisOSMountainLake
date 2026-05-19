function ns() {
  return Number(process.hrtime.bigint());
}

function computeStats(samples) {
  const n = samples.length;
  if (n === 0) return { min: 0, max: 0, avg: 0, p50: 0, p95: 0, p99: 0, p999: 0, p9999: 0, stddev: 0, opsPerSec: 0, totalNs: 0, count: 0, jitter: 0 };

  const sorted = [...samples].sort((a, b) => a - b);
  const sum = sorted.reduce((s, v) => s + v, 0);
  const avg = sum / n;

  const p50 = sorted[Math.floor(n * 0.50)];
  const p95 = sorted[Math.floor(n * 0.95)];
  const p99 = sorted[Math.floor(n * 0.99)];
  const p999 = sorted[Math.floor(n * 0.999)];
  const p9999 = sorted[Math.floor(n * 0.9999)];
  const min = sorted[0];
  const max = sorted[n - 1];

  const variance = sorted.reduce((s, v) => s + (v - avg) ** 2, 0) / n;
  const stddev = Math.sqrt(variance);
  const opsPerSec = avg > 0 ? 1e9 / avg : 0;
  const totalNs = sum;

  const q1 = sorted[Math.floor(n * 0.25)];
  const q3 = sorted[Math.floor(n * 0.75)];
  const iqr = q3 - q1;
  const jitter = n > 1 ? (stddev / avg) * 100 : 0;

  // Find the highest outlier (> Q3 + 3*IQR) count
  const upperFence = q3 + 3 * iqr;
  const outliers = sorted.filter((v) => v > upperFence).length;

  return { min, max, avg, p50, p95, p99, p999, p9999, stddev, opsPerSec, totalNs, count: n, jitter, outliers, iqr };
}

function measure(fn, iterations = 100, warmup = 10) {
  for (let i = 0; i < warmup; i++) fn();
  const samples = [];
  for (let i = 0; i < iterations; i++) {
    const start = ns();
    fn();
    samples.push(ns() - start);
  }
  return computeStats(samples);
}

async function measureAsync(fn, iterations = 100, warmup = 10) {
  for (let i = 0; i < warmup; i++) await fn();
  const samples = [];
  for (let i = 0; i < iterations; i++) {
    const start = ns();
    await fn();
    samples.push(ns() - start);
  }
  return computeStats(samples);
}

function fmtNs(nsVal) {
  if (nsVal < 1000) return `${nsVal.toFixed(0)} ns`;
  if (nsVal < 1e6) return `${(nsVal / 1000).toFixed(1)} μs`;
  if (nsVal < 1e9) return `${(nsVal / 1e6).toFixed(2)} ms`;
  return `${(nsVal / 1e9).toFixed(3)} s`;
}

function fmtPerf(b) {
  if (!b) return "—";
  const avg = fmtNs(b.avg);
  const p99 = fmtNs(b.p99);
  const ops = b.opsPerSec > 1000 ? `${(b.opsPerSec / 1000).toFixed(1)}K/s` : `${b.opsPerSec.toFixed(0)}/s`;
  return `${avg} │ p99: ${p99} │ ${ops} │ ±${b.jitter.toFixed(1)}%`;
}

function throughputMBps(bytes, totalNs) {
  if (totalNs <= 0) return 0;
  return (bytes / 1e6) / (totalNs / 1e9);
}

module.exports = { measure, measureAsync, fmtNs, fmtPerf, ns, computeStats, throughputMBps };
