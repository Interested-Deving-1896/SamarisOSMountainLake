# Acceptance Criteria

## Critical (must pass for Alpha)

| # | Criterion | Verification |
|---|-----------|------------|
| 1 | `bench --quick` runs without crashing | CLI exits 0 |
| 2 | `bench --full` runs without crashing | CLI exits 0 |
| 3 | JSON output validates against `bench-result.schema.json` | Schema validation passes |
| 4 | Score formula is mathematically correct | `round(normalized × 100) = samaris_score`, verified by unit test |
| 5 | Missing collector does not crash benchmark | Run without VRM socket → exit 0, MISSING_COLLECTOR flag set |
| 6 | Reliability flags are populated correctly | Unit tests verify each flag trigger |
| 7 | `bench --history` shows previous runs | Two sequential runs → both appear |
| 8 | `bench --latest` displays the most recent result | JSON matches last `--full` output |
| 9 | React dashboard displays latest result | Dashboard fetches `/api/bench/latest` and renders |
| 10 | Import baseline works | `bench --import-baseline ./example.json` succeeds |
| 11 | Comparison validity is clearly marked | Output shows `"same_hardware"` or `"reference_only"` |
| 12 | Optimizer input is generated | `/var/lib/samaris/bench/optimizer-input.json` exists after run |
| 13 | CI mode exits correctly | `bench --ci` exits 0 on stable score, non-zero on regression > threshold |

## Important (should pass for Alpha)

| # | Criterion | Verification |
|---|-----------|------------|
| 14 | `bench --stress` detects throttling | Run under load → thermal events reported |
| 15 | `bench --watch` refreshes every 5 seconds | Output updates |
| 16 | WebSocket events fire during a run | Listen to `ws://host/ws` during `bench --run` |
| 17 | REST API returns all endpoints | `GET /api/bench/*` all return 200 |
| 18 | CSV export is parseable | `bench --export csv` → file opens in spreadsheet |
| 19 | History max 100 entries enforced | Run 101 times → oldest entry removed |
| 20 | Hardware class is detected correctly | Compare against `dmidecode` / `lscpu` output |

## Nice to Have (post-Alpha)

| # | Criterion |
|---|-----------|
| 21 | Score badge appears in React UI |
| 22 | History chart shows trend line |
| 23 | Comparison table shows color-coded deltas |
| 24 | Per-baseline scoring shows category breakdown |
| 25 | `volt-bench.service` runs on boot and logs to journald |
