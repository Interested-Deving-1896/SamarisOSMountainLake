const { measureAsync } = require("./benchmark");

class ServiceTestResult {
  constructor(serviceName, testName) {
    this.service = serviceName;
    this.test = testName;
    this.status = "pending";
    this.error = null;
    this.benchmark = null;
    this.durationNs = null;
    this.notes = null;
  }

  passed(benchmark, notes) {
    this.status = "passed";
    this.benchmark = benchmark;
    this.durationNs = benchmark?.avg || null;
    this.notes = notes || null;
    return this;
  }

  failed(error, notes) {
    this.status = "failed";
    this.error = error instanceof Error ? error.message : String(error);
    this.notes = notes || null;
    return this;
  }

  skipped(notes) {
    this.status = "skipped";
    this.notes = notes || null;
    return this;
  }

  toJSON() {
    return {
      service: this.service,
      test: this.test,
      status: this.status,
      error: this.error,
      benchmark: this.benchmark,
      notes: this.notes,
    };
  }
}

async function runServiceTest(serviceName, testName, fn, opts = {}) {
  const result = new ServiceTestResult(serviceName, testName);
  try {
    const bench = await measureAsync(fn, opts.iterations, opts.warmup);
    result.passed(bench);
  } catch (err) {
    result.failed(err);
  }
  return result;
}

module.exports = { ServiceTestResult, runServiceTest };
