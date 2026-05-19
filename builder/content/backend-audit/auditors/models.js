const { runServiceTest } = require("../engine/serviceRunner");

async function auditModels(logger, kernelRoot) {
  const results = [];

  const App = require(`${kernelRoot}/models/App`);
  results.push(await runServiceTest("Models", "App instantiation", () => {
    new App({ id: "test", name: "Test", runtime: "app", permissions: ["fs.read"] });
  }));

  const Device = require(`${kernelRoot}/models/Device`);
  results.push(await runServiceTest("Models", "Device instantiation", () => {
    new Device({ id: "dev-1", type: "display" });
  }));

  const Process = require(`${kernelRoot}/models/Process`);
  results.push(await runServiceTest("Models", "Process instantiation", () => {
    new Process({ pid: 1, appId: "test", runtime: "chromium", permissions: [] });
  }));

  const Runtime = require(`${kernelRoot}/models/Runtime`);
  results.push(await runServiceTest("Models", "Runtime instantiation", () => {
    new Runtime({ id: "rt-1", kind: "chromium", target: "test" });
  }));

  return results;
}

module.exports = { auditModels, label: "Models" };
