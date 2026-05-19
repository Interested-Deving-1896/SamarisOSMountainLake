'use strict';

const { createUnifier, VoltUnifier } = require('./unifier');
const { loadConfig } = require('./config/loader');
const { VoltUnifierError } = require('./errors');

function createVoltUnifier(kernel, configOverride) {
  const config = configOverride || loadConfig();
  return createUnifier(kernel, config);
}

module.exports = {
  createVoltUnifier,
  VoltUnifier,
  VoltUnifierError,
};
