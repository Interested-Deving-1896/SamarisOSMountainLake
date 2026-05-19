# Roadmap

## Alpha

- [x] System collectors (RAM, CPU, disk, network, boot)
- [x] UI collectors (FPS, app launch, window resize)
- [x] VRM/DWP basic collectors (SBP IPC)
- [x] JSON output (latest.json, history.json)
- [x] Score engine (0–100 → 0–10000, category weights, badges)
- [x] React dashboard (score, metrics, history chart)
- [x] History persistence (max 100 runs)
- [x] CLI: --quick, --full, --latest, --history

## Alpha+

- [ ] Orbit AI metrics (model load, inference, tokens/sec)
- [ ] Peregrine browser metrics (page load, tab switch)
- [ ] Finder metrics (listing, search, Quick Look)
- [ ] VGM collector (VRAM, shader cache, frame budget)
- [ ] VUM collector (cache hit, writeback, journal)
- [ ] Baseline import and comparison
- [ ] CSV export
- [ ] CLI: --stress, --watch, --ci

## Beta

- [ ] AutoOptimizer input generation
- [ ] CI regression mode (non-zero exit on score drop)
- [ ] Advanced charts (radar, trend, distribution)
- [ ] Hardware class profiles (per-class baselines)
- [ ] WebSocket live progress in React UI
- [ ] Score sharing (anonymous, opt-in)

## Gamma

- [ ] Public benchmark reports (HTML export)
- [ ] Signed benchmark certificates (verify authenticity)
- [ ] Local benchmark comparison network (LAN, cloudless)
- [ ] Per-build performance regression tracking
- [ ] Thermal profile recommendations
- [ ] Adaptive benchmark (auto-adjusts iteration count)
