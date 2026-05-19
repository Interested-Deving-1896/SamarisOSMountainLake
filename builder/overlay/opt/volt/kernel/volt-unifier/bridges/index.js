'use strict';
const { ServiceBridge } = require('./serviceBridge');
const { DesktopBridge } = require('./desktopBridge');
const { FinderBridge } = require('./finderBridge');
const { SettingsBridge } = require('./settingsBridge');
const { OrbitBridge } = require('./orbitBridge');
const { DevToolsBridge } = require('./devtoolsBridge');
const { AirBarBridge } = require('./airbarBridge');

module.exports = {
  ServiceBridge,
  DesktopBridge,
  FinderBridge,
  SettingsBridge,
  OrbitBridge,
  DevToolsBridge,
  AirBarBridge,
};
