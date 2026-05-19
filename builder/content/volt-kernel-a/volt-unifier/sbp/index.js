'use strict';

const { crc32 } = require('./checksum');
const { SbpMessage, HEADER_SIZE } = require('./message');
const {
  encodeMessage,
  decodeMessage,
  NEEDED_BYTES,
} = require('./serializer');
const {
  checkCapability,
  requireCapability,
  isSensitiveCommand,
  isSensitiveOpcode,
  requirePermission,
} = require('./permissions');
const { SbpRouter } = require('./router');
const {
  InvalidSbpMessageError,
  VoltUnifierError,
} = require('../errors');

module.exports = {
  crc32,
  SbpMessage,
  HEADER_SIZE,
  encodeMessage,
  decodeMessage,
  NEEDED_BYTES,
  checkCapability,
  requireCapability,
  isSensitiveCommand,
  isSensitiveOpcode,
  requirePermission,
  SbpRouter,
  InvalidSbpMessageError,
  VoltUnifierError,
};
