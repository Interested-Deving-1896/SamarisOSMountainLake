function render(results) {
  const total = results.length;
  const passed = results.filter((r) => r.status === "passed").length;
  const failed = results.filter((r) => r.status === "failed").length;
  const skipped = results.filter((r) => r.status === "skipped").length;

  const report = {
    meta: {
      timestamp: new Date().toISOString(),
      nodeVersion: process.version,
      platform: process.platform,
      arch: process.arch,
      totalMemory: require("os").totalmem(),
      cpus: require("os").cpus().length,
    },
    summary: {
      total,
      passed,
      failed,
      skipped,
      passRate: total > 0 ? ((passed / total) * 100).toFixed(1) : "0.0",
      healthScore: total > 0 ? Math.round((passed / total) * 100) : 0,
    },
    results: results.map((r) => r.toJSON?.() || r),
  };

  return JSON.stringify(report, null, 2);
}

module.exports = { render };
