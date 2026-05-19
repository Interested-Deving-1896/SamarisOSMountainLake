class HandlerTestResult {
  constructor(handlerName, testName) {
    this.handler = handlerName;
    this.test = testName;
    this.status = "pending";
    this.responseType = null;
    this.error = null;
    this.durationNs = null;
    this.notes = null;
  }

  passed(responseType, durationNs, notes) {
    this.status = "passed";
    this.responseType = responseType;
    this.durationNs = durationNs;
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
      handler: this.handler,
      test: this.test,
      status: this.status,
      responseType: this.responseType,
      error: this.error,
      durationNs: this.durationNs,
      notes: this.notes,
    };
  }
}

async function runHandlerTest(handlerName, testName, handler, message, kernel) {
  const result = new HandlerTestResult(handlerName, testName);
  const start = Number(process.hrtime.bigint());
  try {
    const response = await handler.handle(message, kernel, { send() {} });
    result.passed(response?.type || "no_type", Number(process.hrtime.bigint()) - start);
  } catch (err) {
    result.failed(err);
  }
  return result;
}

module.exports = { HandlerTestResult, runHandlerTest };
