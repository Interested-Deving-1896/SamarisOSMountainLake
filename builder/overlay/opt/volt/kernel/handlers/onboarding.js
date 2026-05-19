const logger = require("../core/logger");

function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "onboarding.get":
      return { type: "onboarding.get.result", data: await kernel.onboarding.get() };
    case "onboarding.patch":
      return { type: "onboarding.patch.result", data: await kernel.onboarding.patch(payload) };
    case "onboarding.createAccount":
      try {
        const result = await kernel.onboarding.createAccount(payload);
        return { type: "onboarding.createAccount.result", data: result };
      } catch (err) {
        const detail = err?.message || err?.code?.toLowerCase() || "onboarding_create_account_failed";
        logger.error("onboarding:createAccount_failed", detail, err?.stack);
        return { type: "error", data: detail };
      }
    case "onboarding.evaluateSetup":
      try {
        const result = await kernel.onboarding.evaluateSetup(payload);
        return { type: "onboarding.evaluateSetup.result", data: result };
      } catch (err) {
        logger.error("onboarding:evaluateSetup_failed", err?.stack || String(err));
        return { type: "error", data: err?.code?.toLowerCase() || "onboarding_setup_failed", message: err?.message || "Setup evaluation failed" };
      }
    case "onboarding.complete":
      return { type: "onboarding.complete.result", data: await kernel.onboarding.complete() };
    case "onboarding.reset":
      return { type: "onboarding.reset.result", data: await kernel.onboarding.reset() };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
