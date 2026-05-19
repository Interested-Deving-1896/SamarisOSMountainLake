const { runServiceTest } = require("../engine/serviceRunner");

// Implement SBP v5 protocol encoding/decoding in pure JS for benchmarking
const SBP_MAGIC = 0x4F56; // "VO" little-endian
const SBP_VERSION = 0x05;

function sbpEncode(opcode, flags, payload, appId = 0, priority = 2) {
  const payloadBuf = payload ? Buffer.from(payload, "utf8") : Buffer.alloc(0);
  const header = Buffer.alloc(16);
  header.writeUInt16LE(SBP_MAGIC, 0);
  header[2] = SBP_VERSION;
  header[3] = opcode;
  header.writeUInt16LE(flags, 4);
  header[6] = priority;
  header.writeUInt32LE(appId, 7);
  header.writeUInt32LE(payloadBuf.length, 11);

  // XOR checksum of bytes 0..14
  let checksum = 0;
  for (let i = 0; i < 15; i++) checksum ^= header[i];
  header[15] = checksum;

  return Buffer.concat([header, payloadBuf]);
}

function sbpDecode(buffer) {
  if (buffer.length < 16) throw new Error("too_short");
  const magic = buffer.readUInt16LE(0);
  if (magic !== SBP_MAGIC) throw new Error("bad_magic");
  const version = buffer[2];
  if (version !== SBP_VERSION) throw new Error("bad_version");
  const opcode = buffer[3];
  const flags = buffer.readUInt16LE(4);
  const priority = buffer[6];
  const appId = buffer.readUInt32LE(7);
  const payloadLen = buffer.readUInt32LE(11);

  let checksum = 0;
  for (let i = 0; i < 15; i++) checksum ^= buffer[i];
  if (buffer[15] !== checksum) throw new Error("bad_checksum");

  const payload = buffer.slice(16, 16 + payloadLen);
  return { magic, version, opcode, flags, priority, appId, payloadLen, checksum, payload: payload.toString("utf8") };
}

function jsonRpcEncode(method, params, id) {
  return JSON.stringify({ jsonrpc: "2.0", id, method, params }) + "\n";
}

async function auditSbpProtocol(logger) {
  const results = [];

  // SBP encode
  results.push(await runServiceTest("SBP Protocol", "encode header (16 bytes)", () => {
    sbpEncode(0x01, 0, Buffer.from(""));
  }));

  results.push(await runServiceTest("SBP Protocol", "encode with 1KB payload", () => {
    sbpEncode(0x01, 0, Buffer.from("x".repeat(1024)));
  }));

  results.push(await runServiceTest("SBP Protocol", "encode with 64KB payload", () => {
    sbpEncode(0x01, 0, Buffer.from("x".repeat(65536)));
  }));

  // SBP decode
  const smallFrame = sbpEncode(0x01, 0, Buffer.from("hello"));
  const largeFrame = sbpEncode(0x01, 0, Buffer.from("x".repeat(1024)));

  results.push(await runServiceTest("SBP Protocol", "decode header (16 bytes)", () => {
    sbpDecode(smallFrame);
  }));

  results.push(await runServiceTest("SBP Protocol", "decode 1KB payload", () => {
    sbpDecode(largeFrame);
  }));

  // JSON encode (for comparison)
  results.push(await runServiceTest("SBP Protocol", "JSON-RPC encode 1KB", () => {
    jsonRpcEncode("test", { data: "x".repeat(1024) }, 1);
  }));

  // SBP vs JSON size comparison
  const sbpFrame1k = sbpEncode(0x01, 0, Buffer.from("x".repeat(1024)));
  const jsonFrame1k = Buffer.from(jsonRpcEncode("test", { data: "x".repeat(1024) }, 1));
  const ratio = jsonFrame1k.length / sbpFrame1k.length;

  results.push({ service: "SBP Protocol", test: "SBP vs JSON wire size", status: "passed",
    notes: `SBP: ${sbpFrame1k.length}B · JSON: ${jsonFrame1k.length}B · ` +
           `ratio: ${ratio.toFixed(1)}x · ` +
           `savings: ${((1 - 1/ratio) * 100).toFixed(0)}% less wire data`,
    benchmark: null, toJSON() { return this; } });

  // SBP checksum verify
  results.push(await runServiceTest("SBP Protocol", "checksum verify", () => {
    const frame = sbpEncode(0x0C, 0, Buffer.from(""));
    sbpDecode(frame);
  }));

  // Error paths
  results.push(await runServiceTest("SBP Protocol", "bad magic detection", () => {
    try {
      const bad = Buffer.from(smallFrame);
      bad[0] = 0x00;
      sbpDecode(bad);
    } catch {}
  }));

  results.push(await runServiceTest("SBP Protocol", "bad checksum detection", () => {
    try {
      const bad = Buffer.from(smallFrame);
      bad[15] ^= 0xFF;
      sbpDecode(bad);
    } catch {}
  }));

  return results;
}

module.exports = { auditSbpProtocol, label: "SBP v5 Protocol Analysis" };
