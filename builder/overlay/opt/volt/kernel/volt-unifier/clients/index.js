'use strict';
const { BaseModuleClient } = require('./baseClient');
const { KernelBClient } = require('./kernelBClient');
const { VrmClient } = require('./vrmClient');
const { VumClient } = require('./vumClient');
const { VgmClient } = require('./vgmClient');
const { DwpClient } = require('./dwpClient');
const { DwpLocalFallback } = require('./dwpLocalFallback');
const { AscClient } = require('./ascClient');
const { BootClient } = require('./bootClient');

module.exports = {
  BaseModuleClient,
  KernelBClient,
  VrmClient,
  VumClient,
  VgmClient,
  DwpClient,
  DwpLocalFallback,
  AscClient,
  BootClient,
};
