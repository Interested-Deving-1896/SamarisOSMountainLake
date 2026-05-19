function payloadOf(message) {
  return message.payload || message.data || {};
}

async function handle(message, kernel) {
  const payload = payloadOf(message);

  switch (message.type) {
    case "firewall.list":
      return { type: "firewall.list.result", data: await kernel.firewall.list() };
    case "firewall.enabled":
      return { type: "firewall.enabled.result", data: await kernel.firewall.setEnabled(payload.enabled) };
    case "firewall.ruleAdd":
      return { type: "firewall.ruleAdd.result", data: await kernel.firewall.addRule(payload) };
    case "firewall.ruleRemove":
      return { type: "firewall.ruleRemove.result", data: await kernel.firewall.removeRule(payload.ruleId) };
    case "firewall.policy":
      return { type: "firewall.policy.result", data: await kernel.firewall.setPolicy(payload.direction, payload.action) };
    default:
      return { type: "error", data: "unknown_type" };
  }
}

module.exports = { handle };
