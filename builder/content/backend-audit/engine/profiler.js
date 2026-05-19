class MemorySnapshot {
  constructor() {
    const mem = process.memoryUsage();
    this.heapUsed = mem.heapUsed;
    this.heapTotal = mem.heapTotal;
    this.external = mem.external;
    this.rss = mem.rss;
    this.arrayBuffers = mem.arrayBuffers || 0;
    this.timestamp = Date.now();
  }
}

async function profileMemory(fn, label, { iterations = 1000 } = {}) {
  // Force a GC if available
  if (typeof global.gc === "function") global.gc();

  const before = new MemorySnapshot();

  for (let i = 0; i < iterations; i++) {
    fn();
  }

  if (typeof global.gc === "function") global.gc();

  const after = new MemorySnapshot();

  const perCall = {
    heapUsed: (after.heapUsed - before.heapUsed) / iterations,
    heapTotal: (after.heapTotal - before.heapTotal) / iterations,
    external: (after.external - before.external) / iterations,
    rss: (after.rss - before.rss) / iterations,
  };

  return {
    label,
    iterations,
    before: before,
    after: after,
    perCall,
    totalDiff: {
      heapUsed: after.heapUsed - before.heapUsed,
      heapTotal: after.heapTotal - before.heapTotal,
      rss: after.rss - before.rss,
    },
  };
}

async function profileMemoryAsync(fn, label, { iterations = 100 } = {}) {
  if (typeof global.gc === "function") global.gc();

  const before = new MemorySnapshot();

  for (let i = 0; i < iterations; i++) {
    await fn();
  }

  if (typeof global.gc === "function") global.gc();

  const after = new MemorySnapshot();

  const perCall = {
    heapUsed: (after.heapUsed - before.heapUsed) / iterations,
    heapTotal: (after.heapTotal - before.heapTotal) / iterations,
  };

  return {
    label,
    iterations,
    before,
    after,
    perCall,
    totalDiff: {
      heapUsed: after.heapUsed - before.heapUsed,
      heapTotal: after.heapTotal - before.heapTotal,
    },
  };
}

function measureCpuTime(fn, iterations = 1000) {
  const start = process.cpuUsage();
  for (let i = 0; i < iterations; i++) fn();
  const elapsed = process.cpuUsage(start);
  return {
    user: elapsed.user / iterations,
    system: elapsed.system / iterations,
    total: (elapsed.user + elapsed.system) / iterations,
  };
}

module.exports = { profileMemory, profileMemoryAsync, measureCpuTime, MemorySnapshot };
