'use strict';

const fs = require('fs');
const path = require('path');
const { defaultConfig } = require('./defaultConfig');

function loadConfig() {
  const config = JSON.parse(JSON.stringify(defaultConfig));

  const configPath = process.env.SAMARIS_UNIFIER_CONFIG;
  if (configPath) {
    try {
      const resolved = path.resolve(configPath);
      const raw = fs.readFileSync(resolved, 'utf8');
      const overrides = JSON.parse(raw);
      mergeDeep(config, overrides);
    } catch (err) {
      throw new Error(`Failed to load config from ${configPath}: ${err.message}`);
    }
  }

  if (process.env.SAMARIS_UNIFIER_DEBUG_PORT) {
    config.debugHttpPort = parseInt(process.env.SAMARIS_UNIFIER_DEBUG_PORT, 10);
  }
  if (process.env.SAMARIS_UNIFIER_DEBUG_BIND) {
    config.debugHttpBind = process.env.SAMARIS_UNIFIER_DEBUG_BIND;
  }
  if (process.env.SAMARIS_UNIFIER_MODE) {
    config.mode = process.env.SAMARIS_UNIFIER_MODE;
  }
  if (process.env.SAMARIS_UNIFIER_DISABLED === '1' || process.env.SAMARIS_UNIFIER_DISABLED === 'true') {
    config.enabled = false;
  }

  if (process.env.SAMARIS_SOCKET_DIR) {
    config.transport.socketDir = process.env.SAMARIS_SOCKET_DIR;
  }
  if (process.env.SAMARIS_SHM_PATH) {
    config.transport.shmPath = process.env.SAMARIS_SHM_PATH;
  }
  if (process.env.SAMARIS_SHM_SIZE_MB) {
    config.transport.shmSizeMb = parseInt(process.env.SAMARIS_SHM_SIZE_MB, 10);
  }

  return config;
}

function mergeDeep(target, source) {
  for (const key of Object.keys(source)) {
    if (source[key] !== null && typeof source[key] === 'object' && !Array.isArray(source[key])) {
      if (!target[key] || typeof target[key] !== 'object') {
        target[key] = {};
      }
      mergeDeep(target[key], source[key]);
    } else if (source[key] !== undefined) {
      target[key] = source[key];
    }
  }
  return target;
}

module.exports = { loadConfig };
