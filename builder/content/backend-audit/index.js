#!/usr/bin/env node

const path = require("node:path");
const fs = require("node:fs");
const config = require("./config");
const { MockLogger } = require("./engine/mockFactory");
const { writeBoth } = require("./reporters");

const logger = new MockLogger();

const KERNEL_ROOT = config.kernelRoot;

function verifyKernelRoot() {
  try {
    const pkg = require(path.join(KERNEL_ROOT, "package.json"));
    console.log(`   Kernel: ${pkg.name} v${pkg.version}`);
  } catch {
    console.error(`❌ Cannot find volt-kernel-a at ${KERNEL_ROOT}`);
    console.error(`   Expected to find: ${path.join(KERNEL_ROOT, "package.json")}`);
    process.exit(1);
  }
}

async function runAuditor(auditorFn, name) {
  const start = Date.now();
  process.stdout.write(`   ${name}... `);
  try {
    const results = await auditorFn(logger, KERNEL_ROOT);
    const passed = results.filter((r) => r.status === "passed").length;
    const failed = results.filter((r) => r.status === "failed").length;
    const skipped = results.filter((r) => r.status === "skipped").length;
    const ms = Date.now() - start;
    console.log(`${results.length} tests · ${passed}✅ ${failed}❌ ${skipped}⚠️  (${ms}ms)`);
    return results;
  } catch (err) {
    console.log(`❌ AUDITOR CRASHED: ${err.message}`);
    return [{ service: name, test: "crash", status: "failed", error: err.message }];
  }
}

async function main() {
  console.log("\n═══════════════════════════════════════════");
  console.log("   SAMARIS OS — BACKEND AUDIT");
  console.log("═══════════════════════════════════════════\n");

  verifyKernelRoot();
  console.log(`   Config: ${require('./config').defaultIterations} iterations, ${require('./config').defaultWarmup} warmup\n`);

  const startAll = Date.now();
  const allResults = [];

  // ── Core Modules ──
  console.log("\n── Core Modules ──\n");
  const core = require("./auditors/core");
  allResults.push(...await runAuditor(core.auditAuth, "Auth"));
  allResults.push(...await runAuditor(core.auditEventBus, "EventBus"));
  allResults.push(...await runAuditor(core.auditScheduler, "Scheduler"));

  // ── Models ──
  console.log("\n── Models ──\n");
  const models = require("./auditors/models");
  allResults.push(...await runAuditor(models.auditModels, "Models"));

  // ── Services ──
  console.log("\n── Services ──\n");
  const svc = require("./auditors/services");
  allResults.push(...await runAuditor(svc.auditPermissionManager, "PermissionManager"));
  allResults.push(...await runAuditor(svc.auditFileSystem, "FileSystemService"));
  allResults.push(...await runAuditor(svc.auditVault, "VaultService"));
  allResults.push(...await runAuditor(svc.auditUserService, "UserService"));
  allResults.push(...await runAuditor(svc.auditArchiveService, "ArchiveService"));
  allResults.push(...await runAuditor(svc.auditMediaService, "MediaService"));
  allResults.push(...await runAuditor(svc.auditAudioService, "AudioService"));
  allResults.push(...await runAuditor(svc.auditBatteryService, "BatteryService"));
  allResults.push(...await runAuditor(svc.auditNetworkService, "NetworkService"));
  allResults.push(...await runAuditor(svc.auditPowerService, "PowerService"));
  allResults.push(...await runAuditor(svc.auditSystemMetrics, "SystemMetricsService"));
  allResults.push(...await runAuditor(svc.auditProcessManager, "ProcessManager"));
  allResults.push(...await runAuditor(svc.auditRuntimeManager, "RuntimeManager"));
  allResults.push(...await runAuditor(svc.auditWindowManager, "WindowManager"));
  allResults.push(...await runAuditor(svc.auditDiskService, "DiskService"));
  allResults.push(...await runAuditor(svc.auditFirewallService, "FirewallService"));
  allResults.push(...await runAuditor(svc.auditPrintService, "PrintService"));
  allResults.push(...await runAuditor(svc.auditSessionFeatures, "SessionFeaturesService"));
  allResults.push(...await runAuditor(svc.auditSearchService, "SearchService"));
  allResults.push(...await runAuditor(svc.auditDevState, "DevStateService"));
  allResults.push(...await runAuditor(svc.auditStorageService, "StorageService"));
  allResults.push(...await runAuditor(svc.auditWineService, "WineService"));
  allResults.push(...await runAuditor(svc.auditBrowserService, "BrowserService"));
  allResults.push(...await runAuditor(svc.auditKernelBClient, "KernelBClient"));
  allResults.push(...await runAuditor(svc.auditEncryptionService, "EncryptionService"));
  allResults.push(...await runAuditor(svc.auditAppStoreService, "AppStoreService"));
  allResults.push(...await runAuditor(svc.auditOrbitRuntime, "OrbitRuntimeService"));
  allResults.push(...await runAuditor(svc.auditTTSService, "TTSService"));
  allResults.push(...await runAuditor(svc.auditSTTService, "STTService"));
  allResults.push(...await runAuditor(svc.auditConnectivityService, "ConnectivityService"));
  allResults.push(...await runAuditor(svc.auditMailService, "MailService"));

  // ── Handlers ──
  console.log("\n── Handlers ──\n");
  const handlers = require("./auditors/handlers");
  allResults.push(...await runAuditor(handlers.auditSystemHandler, "system"));
  allResults.push(...await runAuditor(handlers.auditFsHandler, "fs"));
  allResults.push(...await runAuditor(handlers.auditAudioHandler, "audio"));
  allResults.push(...await runAuditor(handlers.auditBatteryHandler, "battery"));
  allResults.push(...await runAuditor(handlers.auditArchiveHandler, "archive"));
  allResults.push(...await runAuditor(handlers.auditMediaHandler, "media"));
  allResults.push(...await runAuditor(handlers.auditUserHandler, "user"));
  allResults.push(...await runAuditor(handlers.auditSessionHandler, "session"));
  allResults.push(...await runAuditor(handlers.auditDeviceHandler, "device"));
  allResults.push(...await runAuditor(handlers.auditDisplayHandler, "display"));
  allResults.push(...await runAuditor(handlers.auditPowerHandler, "power"));
  allResults.push(...await runAuditor(handlers.auditNetworkHandler, "network"));
  allResults.push(...await runAuditor(handlers.auditAppHandler, "app"));
  allResults.push(...await runAuditor(handlers.auditRuntimeHandler, "runtime"));
  allResults.push(...await runAuditor(handlers.auditProcessHandler, "process"));
  allResults.push(...await runAuditor(handlers.auditSearchHandler, "search"));
  allResults.push(...await runAuditor(handlers.auditFirewallHandler, "firewall"));
  allResults.push(...await runAuditor(handlers.auditDiskHandler, "disk"));
  allResults.push(...await runAuditor(handlers.auditStorageHandler, "storage"));
  allResults.push(...await runAuditor(handlers.auditPrintHandler, "print"));
  allResults.push(...await runAuditor(handlers.auditOnboardingHandler, "onboarding"));
  allResults.push(...await runAuditor(handlers.auditEncryptionHandler, "encryption"));
  allResults.push(...await runAuditor(handlers.auditTTSHandler, "tts"));
  allResults.push(...await runAuditor(handlers.auditSTTHandler, "stt"));
  allResults.push(...await runAuditor(handlers.auditWineHandler, "wine"));
  allResults.push(...await runAuditor(handlers.auditOrbitHandler, "orbit"));
  allResults.push(...await runAuditor(handlers.auditBrowserHandler, "browser"));

  // ── Volt-Unifier ──
  console.log("\n── Volt-Unifier ──\n");
  const vu = require("./auditors/voltUnifier");
  allResults.push(...await runAuditor(vu.auditUnifierConstants, "Unifier Constants"));
  allResults.push(...await runAuditor(vu.auditSbpMessage, "SBP Message"));
  allResults.push(...await runAuditor(vu.auditSbpRouter, "SBP Router"));
  allResults.push(...await runAuditor(vu.auditModuleRegistry, "ModuleRegistry"));
  allResults.push(...await runAuditor(vu.auditEventBus, "Unifier EventBus"));
  allResults.push(...await runAuditor(vu.auditCapabilityGuard, "CapabilityGuard"));
  allResults.push(...await runAuditor(vu.auditLifecycle, "Unifier Lifecycle"));
  allResults.push(...await runAuditor(vu.auditBridges, "Unifier Bridges"));
  allResults.push(...await runAuditor(vu.auditHealthMonitor, "Unifier Health"));
  allResults.push(...await runAuditor(vu.auditMetrics, "Unifier Metrics"));

  // ── Kernel B ──
  console.log("\n── Kernel B (Tesseract Engine) ──\n");
  const kb = require("./auditors/kernelB");
  allResults.push(...await runAuditor(kb.auditKernelB, "Kernel B"));

  // ── Inter-Kernel ──
  console.log("\n── Inter-Kernel Communication ──\n");
  const ik = require("./auditors/interKernel");
  allResults.push(...await runAuditor(ik.auditInterKernel, "Inter-Kernel"));

  // ── SBP Protocol ──
  console.log("\n── SBP v5 Protocol ──\n");
  const sbp = require("./auditors/sbpProtocol");
  allResults.push(...await runAuditor(sbp.auditSbpProtocol, "SBP Protocol"));

  // ── Profiling ──
  console.log("\n── CPU & Memory Profiling ──\n");
  const prof = require("./auditors/profiling");
  allResults.push(...await runAuditor(prof.auditProfiling, "Profiling"));

  // ── Integration ──
  console.log("\n── Integration ──\n");
  const integ = require("./auditors/integration");
  allResults.push(...await runAuditor(integ.auditFileWriteReadCycle, "FS write/read cycle"));
  allResults.push(...await runAuditor(integ.auditPermissionAuthIntegration, "Permission/Auth"));
  allResults.push(...await runAuditor(integ.auditVaultEncryptDecryptCycle, "Vault encrypt/decrypt"));
  allResults.push(...await runAuditor(integ.auditRouterDispatch, "Router dispatch"));

  // ── Summary ──
  const totalMs = Date.now() - startAll;
  const total = allResults.length;
  const passed = allResults.filter((r) => r.status === "passed").length;
  const failed = allResults.filter((r) => r.status === "failed").length;
  const skipped = allResults.filter((r) => r.status === "skipped").length;
  const pct = total > 0 ? ((passed / total) * 100).toFixed(1) : "0.0";
  const score = total > 0 ? Math.round((passed / total) * 100) : 0;

  console.log("\n═══════════════════════════════════════════");
  console.log("   AUDIT COMPLETE");
  console.log("═══════════════════════════════════════════\n");
  console.log(`   Duration: ${(totalMs / 1000).toFixed(1)}s`);
  console.log(`   Total:    ${total} tests`);
  console.log(`   ✅ Passed: ${passed} (${pct}%)`);
  console.log(`   ❌ Failed: ${failed}`);
  console.log(`   ⚠️  Skipped: ${skipped}`);
  console.log(`   Score:    ${score}/100\n`);

  // ── Write reports ──
  try {
    const paths = await writeBoth(allResults);
    console.log(`   Reports written:`);
    console.log(`   → ${paths.markdown}`);
    console.log(`   → ${paths.json}\n`);
  } catch (err) {
    console.error(`   ❌ Report write failed: ${err.message}\n`);
  }

  if (failed > 0) {
    console.log("   Failed tests:");
    for (const f of allResults.filter((r) => r.status === "failed")) {
      console.log(`   ❌ ${f.service || f.handler}: ${f.test} — ${f.error}`);
    }
    console.log();
  }

  process.exit(failed > 0 ? 1 : 0);
}

main().catch((err) => {
  console.error(`\n❌ Fatal error: ${err.message}\n${err.stack}\n`);
  process.exit(1);
});
