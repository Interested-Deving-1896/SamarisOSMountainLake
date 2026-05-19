'use strict';

function createDashboardSnapshot(input) {
  const snapshot = {
    timestamp: input.timestamp || Date.now(),
    health: input.health || null,
    ram: input.ram || null,
    usb: input.usb || null,
    gpu: input.gpu || null,
    workers: input.workers || null,
    adaptive: input.adaptive || null,
    events: Array.isArray(input.events) ? input.events : [],
  };
  return snapshot;
}

const DashboardSnapshot = {
  create: createDashboardSnapshot,
};

module.exports = { DashboardSnapshot, createDashboardSnapshot };
