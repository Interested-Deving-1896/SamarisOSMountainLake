'use strict';

const { ModuleRegistry } = require('./moduleRegistry');
const { getDefaultCapabilities, hasFeature, hasPermission } = require('./capabilities');

module.exports = {
  ModuleRegistry,
  getDefaultCapabilities,
  hasFeature,
  hasPermission,
};
