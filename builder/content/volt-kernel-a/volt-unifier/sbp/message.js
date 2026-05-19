'use strict';

const { crc32 } = require('./checksum');
const { SBP_MAGIC, SBP_VERSION, SBP_FLAGS, SBP_MAX_PAYLOAD } = require('../constants');
const { InvalidSbpMessageError } = require('../errors');

const HEADER_SIZE = 32;

class SbpMessage {
  #magic;
  #version;
  #opcode;
  #flags;
  #requestId;
  #timestampUs;
  #payloadLen;
  #checksum;
  #payload;

  constructor(header, payload) {
    this.#magic = header.magic;
    this.#version = header.version;
    this.#opcode = header.opcode;
    this.#flags = header.flags;
    this.#requestId = header.requestId;
    this.#timestampUs = header.timestampUs;
    this.#payloadLen = header.payloadLen;
    this.#checksum = header.checksum;
    this.#payload = Buffer.from(payload);
  }

  static create(opcode, flags, payload, requestId) {
    if (typeof requestId === 'undefined' || requestId === null) {
      requestId = process.hrtime.bigint();
    }
    if (typeof requestId !== 'bigint') {
      requestId = BigInt(requestId);
    }

    const payloadBuf = payload ? Buffer.from(payload) : Buffer.alloc(0);

    if (payloadBuf.length > SBP_MAX_PAYLOAD) {
      throw new RangeError(
        `Payload exceeds maximum size of ${SBP_MAX_PAYLOAD} bytes`
      );
    }

    const timestampUs = process.hrtime.bigint() / 1000n;

    const header = {
      magic: SBP_MAGIC,
      version: SBP_VERSION,
      opcode,
      flags,
      requestId,
      timestampUs,
      payloadLen: payloadBuf.length,
      checksum: 0,
    };

    const msg = new SbpMessage(header, payloadBuf);

    const head = Buffer.alloc(HEADER_SIZE);
    head.writeUInt32LE(SBP_MAGIC, 0);
    head.writeUInt8(SBP_VERSION, 4);
    head.writeUInt8(opcode, 5);
    head.writeUInt16LE(flags, 6);
    head.writeBigInt64LE(requestId, 8);
    head.writeBigInt64LE(timestampUs, 16);
    head.writeUInt32LE(payloadBuf.length, 24);

    const combined = payloadBuf.length > 0
      ? Buffer.concat([head.slice(0, 28), payloadBuf])
      : head.slice(0, 28);

    msg.#checksum = crc32(combined);
    head.writeUInt32LE(msg.#checksum, 28);
    msg.#payload = payloadBuf;

    return msg;
  }

  static fromBuffer(buffer) {
    if (!Buffer.isBuffer(buffer)) {
      throw new TypeError('Expected a Buffer');
    }

    if (buffer.length < HEADER_SIZE) {
      return null;
    }

    const magic = buffer.readUInt32LE(0);
    if (magic !== SBP_MAGIC) {
      throw new InvalidSbpMessageError(
        `Invalid magic: 0x${magic.toString(16)}`
      );
    }

    const version = buffer.readUInt8(4);
    if (version !== SBP_VERSION) {
      throw new InvalidSbpMessageError(
        `Unsupported SBP version: ${version}`
      );
    }

    const opcode = buffer.readUInt8(5);
    const flags = buffer.readUInt16LE(6);
    const requestId = buffer.readBigInt64LE(8);
    const timestampUs = buffer.readBigInt64LE(16);
    const payloadLen = buffer.readUInt32LE(24);

    if (payloadLen > SBP_MAX_PAYLOAD) {
      throw new InvalidSbpMessageError(
        `Payload length ${payloadLen} exceeds maximum ${SBP_MAX_PAYLOAD}`
      );
    }

    const totalSize = HEADER_SIZE + payloadLen;
    if (buffer.length < totalSize) {
      return null;
    }

    const storedChecksum = buffer.readUInt32LE(28);
    const payload = buffer.slice(HEADER_SIZE, totalSize);

    const headerPrefix = buffer.slice(0, 28);
    const checksumInput = payloadLen > 0
      ? Buffer.concat([headerPrefix, payload])
      : headerPrefix;
    const expectedChecksum = crc32(checksumInput);

    if (storedChecksum !== expectedChecksum) {
      throw new InvalidSbpMessageError(
        `Checksum mismatch: stored 0x${storedChecksum.toString(16)}, ` +
        `computed 0x${expectedChecksum.toString(16)}`
      );
    }

    const header = {
      magic,
      version,
      opcode,
      flags,
      requestId,
      timestampUs,
      payloadLen,
      checksum: storedChecksum,
    };

    return new SbpMessage(header, payload);
  }

  encode() {
    const totalSize = HEADER_SIZE + this.#payload.length;
    const buf = Buffer.alloc(totalSize);

    buf.writeUInt32LE(this.#magic, 0);
    buf.writeUInt8(this.#version, 4);
    buf.writeUInt8(this.#opcode, 5);
    buf.writeUInt16LE(this.#flags, 6);
    buf.writeBigInt64LE(this.#requestId, 8);
    buf.writeBigInt64LE(this.#timestampUs, 16);
    buf.writeUInt32LE(this.#payload.length, 24);

    if (this.#payload.length > 0) {
      this.#payload.copy(buf, HEADER_SIZE);
    }

    const headerPrefix = buf.slice(0, 28);
    const payloadSlice = buf.slice(HEADER_SIZE);
    const checksumInput = this.#payload.length > 0
      ? Buffer.concat([headerPrefix, payloadSlice])
      : headerPrefix;
    const computedChecksum = crc32(checksumInput);

    buf.writeUInt32LE(computedChecksum, 28);
    this.#checksum = computedChecksum;

    return buf;
  }

  get magic() {
    return this.#magic;
  }

  get version() {
    return this.#version;
  }

  get opcode() {
    return this.#opcode;
  }

  get flags() {
    return this.#flags;
  }

  get requestId() {
    return this.#requestId;
  }

  get timestampUs() {
    return this.#timestampUs;
  }

  get payloadLen() {
    return this.#payload.length;
  }

  get checksum() {
    return this.#checksum;
  }

  get payload() {
    return this.#payload;
  }

  isRequest() {
    return (this.#flags & SBP_FLAGS.REQUEST) !== 0;
  }

  isResponse() {
    return (this.#flags & SBP_FLAGS.RESPONSE) !== 0;
  }

  isError() {
    return (this.#flags & SBP_FLAGS.ERROR) !== 0;
  }

  isEvent() {
    return (this.#flags & SBP_FLAGS.EVENT) !== 0;
  }
}

SbpMessage.HEADER_SIZE = HEADER_SIZE;

module.exports = { SbpMessage, HEADER_SIZE };
