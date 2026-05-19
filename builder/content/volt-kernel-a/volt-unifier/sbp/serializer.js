'use strict';

const { SbpMessage, HEADER_SIZE } = require('./message');

const NEEDED_BYTES = HEADER_SIZE;

function encodeMessage(opcode, flags, payload, requestId) {
  const msg = SbpMessage.create(opcode, flags, payload, requestId);
  return msg.encode();
}

function decodeMessage(buffer) {
  return SbpMessage.fromBuffer(buffer);
}

module.exports = {
  encodeMessage,
  decodeMessage,
  NEEDED_BYTES,
  HEADER_SIZE,
};
