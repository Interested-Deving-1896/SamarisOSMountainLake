const { fmtNs } = require("../engine/benchmark");

function header(level, text) {
  return `${"#".repeat(level)} ${text}\n\n`;
}

function table(headers, rows) {
  const colWidths = headers.map((h, i) =>
    Math.max(h.length, ...rows.map((r) => String(r[i] || "").length))
  );
  const sep = "| " + colWidths.map((w) => "-".repeat(w)).join(" | ") + " |\n";
  const head = "| " + headers.map((h, i) => h.padEnd(colWidths[i])).join(" | ") + " |\n";
  const body = rows.map((r) =>
    "| " + r.map((v, i) => String(v || "").padEnd(colWidths[i])).join(" | ") + " |\n"
  ).join("");
  return head + sep + body;
}

function statusBadge(status) {
  if (status === "passed") return "✅";
  if (status === "failed") return "❌";
  if (status === "skipped") return "⚠️";
  return "⬜";
}

function renderBenchmark(b) {
  if (!b) return "—";
  return `${fmtNs(b.avg)} (p50: ${fmtNs(b.p50)} · p95: ${fmtNs(b.p95)} · p99: ${fmtNs(b.p99)})`;
}

function render(results) {
  let md = "";

  // Title
  md += "# ═══════════════════════════════════════════\n";
  md += "#   SAMARIS OS — BACKEND AUDIT REPORT\n";
  md += "# ═══════════════════════════════════════════\n\n";

  // 1. Executive Summary
  const total = results.length;
  const passed = results.filter((r) => r.status === "passed").length;
  const failed = results.filter((r) => r.status === "failed").length;
  const skipped = results.filter((r) => r.status === "skipped").length;
  const d = new Date();
  const pct = total > 0 ? ((passed / total) * 100).toFixed(1) : "0.0";
  const healthScore = total > 0 ? Math.round((passed / total) * 100) : 0;

  md += header(1, "1. EXECUTIVE SUMMARY");
  md += "\n| Metric | Value |\n|--------|-------|\n";
  md += `| Audit date | ${d.toISOString()} |\n`;
  md += `| Total tests | ${total} |\n`;
  md += `| ✅ Passed | ${passed} (${pct}%) |\n`;
  md += `| ❌ Failed | ${failed} |\n`;
  md += `| ⚠️  Skipped | ${skipped} |\n`;
  md += `| Health score | ${healthScore}/100 |\n\n`;

  // 2. Environment
  md += header(1, "2. ENVIRONMENT");
  md += "\n| Variable | Value |\n|----------|-------|\n";
  md += `| Node.js | ${process.version} |\n`;
  md += `| Platform | ${process.platform} (${process.arch}) |\n`;
  md += `| Memory | ${(require('os').totalmem() / 1e9).toFixed(1)} GB |\n`;
  md += `| CPUs | ${require('os').cpus().length} cores |\n\n`;

  // 3. Results by service/module
  md += header(1, "3. DETAILED RESULTS");

  const groups = {};
  for (const r of results) {
    const key = r.service || r.handler || "Unknown";
    if (!groups[key]) groups[key] = [];
    groups[key].push(r);
  }

  for (const [groupName, items] of Object.entries(groups)) {
    md += header(2, groupName);

    const rows = items.map((r) => [
      statusBadge(r.status),
      r.test,
      r.status === "passed" ? renderBenchmark(r.benchmark) : (r.error || r.notes || ""),
    ]);

    // Add error detail rows for failures
    const failures = items.filter((r) => r.status === "failed");
    for (const f of failures) {
      md += `\n_${f.test}_: \`${f.error || "unknown error"}\`\n`;
    }

    // Skip rendering empty table, just use a simple list
    for (const r of items) {
      const badge = statusBadge(r.status);
      let desc = r.test;
      if (r.status === "passed" && r.benchmark) {
        desc += ` — ${renderBenchmark(r.benchmark)}`;
      } else if (r.status === "passed" && r.notes) {
        desc += ` — ${r.notes}`;
      } else if (r.status === "failed") {
        desc += ` — ${r.error || "error"}`;
      } else if (r.status === "skipped") {
        desc += ` — ${r.notes || "skipped"}`;
      }
      md += `- ${badge} ${desc}\n`;
    }
    md += "\n";
  }

  // 4. Performance ranking
  md += header(1, "4. PERFORMANCE RANKING");

  const withBench = results.filter((r) => r.status === "passed" && r.benchmark && r.benchmark.avg);
  withBench.sort((a, b) => a.benchmark.avg - b.benchmark.avg);

  const fastest = withBench.slice(0, 10);
  const slowest = withBench.slice(-10).reverse();

  md += header(2, "Fastest operations");
  for (const r of fastest) {
    md += `- ${fmtNs(r.benchmark.avg)} — ${r.service}/${r.test}\n`;
  }
  md += "\n";

  md += header(2, "Slowest operations");
  for (const r of slowest) {
    md += `- ${fmtNs(r.benchmark.avg)} — ${r.service}/${r.test}\n`;
  }
  md += "\n";

  // 5. Failures & Skipped
  const failItems = results.filter((r) => r.status === "failed");
  const skipItems = results.filter((r) => r.status === "skipped");

  md += header(1, "5. FAILURES & SKIPPED");

  if (failItems.length > 0) {
    md += header(2, `❌ Failed (${failItems.length})`);
    for (const f of failItems) {
      md += `- **${f.service || f.handler}** - ${f.test}: \`${f.error}\`\n`;
    }
    md += "\n";
  }

  if (skipItems.length > 0) {
    md += header(2, `⚠️ Skipped (${skipItems.length})`);
    for (const s of skipItems) {
      md += `- **${s.service || s.handler}** - ${s.test}: ${s.notes || "skipped"}\n`;
    }
    md += "\n";
  }

  if (failItems.length === 0 && skipItems.length === 0) {
    md += "None — all tests passed.\n\n";
  }

  return md;
}

module.exports = { render };
