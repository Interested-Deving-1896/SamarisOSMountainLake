const crypto = require("node:crypto");
const http = require("node:http");
const fs = require("node:fs");
const fsp = require("node:fs/promises");
const path = require("node:path");
const { createKernel } = require("./core/kernel");
const { createRouter } = require("./router");
const { createVoltUnifier } = require("./volt-unifier");

const logger = require("./core/logger");
const kernel = createKernel();
const router = createRouter(kernel);

kernel.init();

const unifier = createVoltUnifier(kernel);
kernel.unifier = unifier;
kernel.orbit?.setUnifier?.(unifier);
let unifierReady = false;
let unifierError = null;
unifier.init().then(() => {
  unifierReady = true;
  logger.info("unifier:ready");
}).catch(err => {
  unifierError = err;
  logger.error("unifier:init_failed", err && err.stack ? err.stack : String(err));
});

function setCorsHeaders(res, req) {
  const origin = req?.headers?.origin;
  if (origin && (origin.startsWith("http://127.0.0.1:") || origin.startsWith("http://localhost:") || origin === "file://" || origin === "null")) {
    res.setHeader("Access-Control-Allow-Origin", origin);
  } else {
    res.setHeader("Access-Control-Allow-Origin", "*");
  }
  res.setHeader("Access-Control-Allow-Methods", "GET,POST,OPTIONS");
  res.setHeader("Access-Control-Allow-Headers", "Content-Type, Range");
  res.setHeader("Access-Control-Expose-Headers", "Content-Range, Accept-Ranges, Content-Length");
}

function sendHttpJson(res, statusCode, payload, req) {
  setCorsHeaders(res, req);
  res.writeHead(statusCode, { "content-type": "application/json" });
  res.end(JSON.stringify(payload));
}

function contentTypeFor(targetPath) {
  return MIME_MAP[path.extname(targetPath).toLowerCase()] || "application/octet-stream";
}

const MIME_MAP = {
  ".html": "text/html; charset=utf-8",
  ".js": "application/javascript; charset=utf-8",
  ".mjs": "application/javascript; charset=utf-8",
  ".cjs": "application/javascript; charset=utf-8",
  ".css": "text/css; charset=utf-8",
  ".json": "application/json; charset=utf-8",
  ".map": "application/json; charset=utf-8",
  ".svg": "image/svg+xml",
  ".png": "image/png",
  ".jpg": "image/jpeg",
  ".jpeg": "image/jpeg",
  ".webp": "image/webp",
  ".ico": "image/x-icon",
  ".woff": "font/woff",
  ".woff2": "font/woff2",
  ".ttf": "font/ttf",
  ".mp4": "video/mp4",
  ".webm": "video/webm",
  ".ogg": "video/ogg",
  ".mov": "video/quicktime",
  ".avi": "video/x-msvideo",
  ".mkv": "video/x-matroska",
  ".m4v": "video/x-m4v",
  ".ogv": "video/ogg",
  ".mp3": "audio/mpeg",
  ".wav": "audio/wav",
  ".m4a": "audio/mp4",
  ".flac": "audio/flac",
  ".aac": "audio/aac",
};

async function serveStaticFile(res, absolutePath, req) {
  const stat = await fsp.stat(absolutePath).catch(() => null);
  if (!stat || !stat.isFile()) return false;
  setCorsHeaders(res, req);
  res.writeHead(200, {
    "content-type": contentTypeFor(absolutePath),
    "content-length": stat.size,
    "cache-control": path.extname(absolutePath) === ".html" ? "no-store" : "public, max-age=3600"
  });
  fs.createReadStream(absolutePath).pipe(res);
  return true;
}

function readJsonBody(req) {
  return new Promise((resolve, reject) => {
    const chunks = [];
    req.on("data", (chunk) => chunks.push(chunk));
    req.on("end", () => {
      try {
        const raw = Buffer.concat(chunks).toString("utf8");
        resolve(raw ? JSON.parse(raw) : {});
      } catch (error) {
        reject(error);
      }
    });
    req.on("error", reject);
  });
}

function createAcceptValue(key) {
  return crypto
    .createHash("sha1")
    .update(`${key}258EAFA5-E914-47DA-95CA-C5AB0DC85B11`, "utf8")
    .digest("base64");
}

function encodeFrame(payload, opcode = 1) {
  const body = Buffer.from(payload, "utf8");
  const length = body.length;
  let header;

  if (length < 126) {
    header = Buffer.alloc(2);
    header[1] = length;
  } else if (length < 65536) {
    header = Buffer.alloc(4);
    header[1] = 126;
    header.writeUInt16BE(length, 2);
  } else {
    header = Buffer.alloc(10);
    header[1] = 127;
    header.writeBigUInt64BE(BigInt(length), 2);
  }

  header[0] = 0x80 | opcode;
  return Buffer.concat([header, body]);
}

function decodeFrames(buffer) {
  const messages = [];
  let offset = 0;

  while (offset + 2 <= buffer.length) {
    const first = buffer[offset];
    const second = buffer[offset + 1];
    const opcode = first & 0x0f;
    const masked = (second & 0x80) !== 0;
    let payloadLength = second & 0x7f;
    let headerLength = 2;

    if (payloadLength === 126) {
      if (offset + 4 > buffer.length) break;
      payloadLength = buffer.readUInt16BE(offset + 2);
      headerLength = 4;
    } else if (payloadLength === 127) {
      if (offset + 10 > buffer.length) break;
      const bigLen = buffer.readBigUInt64BE(offset + 2);
      if (bigLen > BigInt(Number.MAX_SAFE_INTEGER)) {
        throw new Error("frame_too_large");
      }
      payloadLength = Number(bigLen);
      headerLength = 10;
    }

    const maskLength = masked ? 4 : 0;
    const frameLength = headerLength + maskLength + payloadLength;
    if (offset + frameLength > buffer.length) break;

    let payload = buffer.subarray(offset + headerLength + maskLength, offset + frameLength);
    if (masked) {
      const mask = buffer.subarray(offset + headerLength, offset + headerLength + 4);
      const unmasked = Buffer.alloc(payload.length);
      for (let i = 0; i < payload.length; i += 1) {
        unmasked[i] = payload[i] ^ mask[i % 4];
      }
      payload = unmasked;
    }

    messages.push({
      opcode,
      masked,
      text: payload.toString("utf8")
    });
    offset += frameLength;
  }

  return {
    messages,
    remaining: buffer.subarray(offset)
  };
}

function sendJson(socket, message) {
  socket.write(encodeFrame(JSON.stringify(message)));
}

const SENSITIVE_FIELDS = ["password", "passwordHash", "passwordSalt", "token", "secret", "authorization", "audioBase64", "base64"];

function maskSensitive(data) {
  if (typeof data === "string") {
    try { return maskSensitive(JSON.parse(data)); } catch { return data; }
  }
  if (!data || typeof data !== "object") return data;
  const masked = Array.isArray(data) ? [...data] : { ...data };
  for (const key of Object.keys(masked)) {
    if (SENSITIVE_FIELDS.some((f) => key.toLowerCase().includes(f))) {
      masked[key] = "***MASKED***";
    } else if (typeof masked[key] === "object" && masked[key] !== null) {
      masked[key] = maskSensitive(masked[key]);
    }
  }
  return masked;
}

function handleClientMessage(socket, rawText) {
  if (process.env.NODE_ENV !== "production") {
    logger.info("ws:in", JSON.stringify(maskSensitive(rawText)));
  }

  let parsed;
  try {
    parsed = JSON.parse(rawText);
  } catch {
    sendJson(socket, { type: "error", data: "invalid_json" });
    return;
  }

  const stream = {
    send(message) {
      const envelope =
        parsed && Object.prototype.hasOwnProperty.call(parsed, "requestId") && !Object.prototype.hasOwnProperty.call(message, "requestId")
          ? { ...message, requestId: parsed.requestId }
          : message;
      if (process.env.NODE_ENV !== "production") {
        logger.info("ws:event", JSON.stringify(maskSensitive(envelope)));
      }
      sendJson(socket, envelope);
    }
  };

  Promise.resolve(router.route(parsed, stream))
    .then((response) => {
      const envelope =
        parsed && Object.prototype.hasOwnProperty.call(parsed, "requestId")
          ? { ...response, requestId: parsed.requestId }
          : response;
      if (process.env.NODE_ENV !== "production") {
        logger.info("ws:out", JSON.stringify(maskSensitive(envelope)));
      }
      sendJson(socket, envelope);
    })
    .catch((error) => {
      logger.error("ws:route_error", error?.stack || String(error));
      const envelope =
        parsed && Object.prototype.hasOwnProperty.call(parsed, "requestId")
          ? { type: "error", data: error && error.code ? String(error.code).toLowerCase() : "internal_error", requestId: parsed.requestId }
          : { type: "error", data: error && error.code ? String(error.code).toLowerCase() : "internal_error" };
      sendJson(socket, envelope);
    });
}

const server = http.createServer(async (req, res) => {
  const url = req.url || "/";

  if (req.method === "OPTIONS") {
    setCorsHeaders(res, req);
    res.writeHead(204);
    res.end();
    return;
  }

  try {
    if (req.method === "GET" && url === "/health") {
      sendHttpJson(res, 200, { ok: true, service: "volt-kernel" }, req);
      return;
    }

    if (req.method === "GET" && url === "/api/peregrine/status") {
      sendHttpJson(res, 200, await kernel.browser.status(), req);
      return;
    }

    if (req.method === "GET" && url === "/api/unifier/health") {
      const snapshot = kernel.unifier && unifierReady
        ? kernel.unifier.getHealthSnapshot()
        : { overallStatus: unifierError ? "error" : "initializing", error: unifierError?.message || null };
      sendHttpJson(res, 200, snapshot, req);
      return;
    }

    if (req.method === "GET" && url === "/api/unifier/snapshot") {
      const snapshot = kernel.unifier && unifierReady
        ? kernel.unifier.getSnapshot()
        : { timestamp: Date.now(), health: { overallStatus: unifierError ? "error" : "initializing" } };
      sendHttpJson(res, 200, snapshot, req);
      return;
    }

    if (req.method === "GET" && url === "/api/unifier/modules") {
      const modules = kernel.unifier && unifierReady && kernel.unifier.registry
        ? kernel.unifier.registry.getAll()
        : new Map();
      const statuses = {};
      for (const [id, entry] of modules) {
        statuses[id] = { status: entry.status, degradedReason: entry.health?.degradedReason || null };
      }
      sendHttpJson(res, 200, statuses, req);
      return;
    }

    if (req.method === "GET" && url === "/api/dev/reset-state") {
      sendHttpJson(res, 200, await kernel.devState.getResetState(), req);
      return;
    }

    if (req.method === "POST" && url === "/api/peregrine/open") {
      const body = await readJsonBody(req);
      sendHttpJson(res, 200, await kernel.browser.launch(body.url || ""), req);
      return;
    }

    if (req.method === "POST" && url === "/api/peregrine/session/open") {
      const body = await readJsonBody(req);
      sendHttpJson(res, 200, await kernel.browser.openAttached({
        url: body.url || "",
        sessionId: body.sessionId || null,
        windowId: body.windowId || null,
        bounds: body.bounds || null,
        focused: Boolean(body.focused),
        minimized: Boolean(body.minimized)
      }), req);
      return;
    }

    if (req.method === "POST" && url === "/api/peregrine/session/sync") {
      const body = await readJsonBody(req);
      sendHttpJson(res, 200, await kernel.browser.syncAttached({
        sessionId: body.sessionId || null,
        bounds: body.bounds || null,
        focused: body.focused !== false,
        minimized: Boolean(body.minimized)
      }), req);
      return;
    }

    if (req.method === "POST" && url === "/api/peregrine/session/close") {
      const body = await readJsonBody(req);
      sendHttpJson(res, 200, await kernel.browser.closeSession(body.sessionId || ""), req);
      return;
    }

    // Stream a file from the virtual filesystem (used for media playback)
    if (req.method === "GET" && url.startsWith("/api/fs/read-file?")) {
      const u = new URL(url, `http://${req.headers.host || "localhost"}`);
      const virtualPath = u.searchParams.get("path") || "";
      if (!virtualPath) return sendHttpJson(res, 400, { ok: false, error: "missing_path" }, req);
      const appId = u.searchParams.get("appId") || "volt.desktop";
      const typeNamespace = virtualPath.startsWith("/User/Music/") ? "media.musicLibrary" :
        virtualPath.startsWith("/User/Videos/") ? "media.videoLibrary" : "fs.read";
      if (!kernel.auth.authorize({ type: typeNamespace, appId }, kernel)) {
        return sendHttpJson(res, 403, { ok: false, error: "permission_denied" }, req);
      }
      try {
        const { actualPath, virtualPath: vPath } = kernel.fileSystem.toActualPath(virtualPath);
        const stat = await fsp.stat(actualPath);
        if (!stat.isFile()) return sendHttpJson(res, 404, { ok: false, error: "not_found" }, req);
        const ext = path.extname(actualPath).toLowerCase();
        const mime = contentTypeFor(actualPath);
        const range = req.headers.range;
        if (range) {
          const parts = range.replace(/bytes=/, "").split("-");
          const start = parseInt(parts[0], 10);
          const end = parts[1] ? parseInt(parts[1], 10) : stat.size - 1;
          if (start >= stat.size) return sendHttpJson(res, 416, { ok: false, error: "range_not_satisfiable" }, req);
          res.writeHead(206, {
            "content-range": `bytes ${start}-${end}/${stat.size}`,
            "content-type": mime,
            "content-length": end - start + 1,
            "accept-ranges": "bytes",
            "access-control-allow-origin": "*",
            "cache-control": "no-store"
          });
          fs.createReadStream(actualPath, { start, end }).pipe(res);
        } else {
          res.writeHead(200, {
            "content-type": mime,
            "content-length": stat.size,
            "accept-ranges": "bytes",
            "access-control-allow-origin": "*",
            "cache-control": "no-store"
          });
          fs.createReadStream(actualPath).pipe(res);
        }
      } catch (err) {
        logger.error("fs:read-file:error", err && err.message ? err.message : String(err));
        sendHttpJson(res, 404, { ok: false, error: "not_found" }, req);
      }
      return;
    }
  } catch (error) {
    logger.error("http:route_error", error && error.stack ? error.stack : String(error));
    sendHttpJson(res, 500, { ok: false, error: "internal_error" }, req);
    return;
  }

  sendHttpJson(res, 426, { ok: false, error: "websocket_upgrade_required" }, req);
});

server.on("upgrade", (req, socket) => {
  const upgrade = String(req.headers.upgrade || "").toLowerCase();
  const key = req.headers["sec-websocket-key"];

  if (upgrade !== "websocket" || typeof key !== "string") {
    socket.write("HTTP/1.1 400 Bad Request\r\n\r\n");
    socket.destroy();
    return;
  }

  const origin = req.headers.origin || req.headers["sec-websocket-origin"] || "";
  const allowedOrigins = ["file://"];
  if (origin && (origin.startsWith("http://127.0.0.1:") || origin.startsWith("http://localhost:") || allowedOrigins.includes(origin))) {
    // allow
  } else if (origin) {
    logger.warn("ws:origin_denied", { origin });
    socket.write("HTTP/1.1 403 Forbidden\r\n\r\n");
    socket.destroy();
    return;
  }

  const acceptValue = createAcceptValue(key);
  socket.write(
    [
      "HTTP/1.1 101 Switching Protocols",
      "Upgrade: websocket",
      "Connection: Upgrade",
      `Sec-WebSocket-Accept: ${acceptValue}`,
      "\r\n"
    ].join("\r\n")
  );

  logger.info("ws:connected", req.socket.remoteAddress || "unknown");

  const RATE_LIMIT = 60;
  const rateBuckets = new WeakMap();

  function checkRateLimit(socket) {
    const now = Date.now();
    let bucket = rateBuckets.get(socket);
    if (!bucket || now - bucket.resetAt > 1000) {
      bucket = { count: 0, resetAt: now };
      rateBuckets.set(socket, bucket);
    }
    bucket.count++;
    return bucket.count <= RATE_LIMIT;
  }

  let bufferedChunks = [];
  let buffered = Buffer.alloc(0);

  socket.on("data", (chunk) => {
    try {
      bufferedChunks.push(chunk);
      const totalSize = bufferedChunks.reduce((sum, c) => sum + c.length, 0);
      if (totalSize > 512 * 1024) {
        logger.warn("ws:buffer_overflow", { size: totalSize });
        socket.destroy();
        return;
      }
      buffered = Buffer.concat(bufferedChunks);
      bufferedChunks = [];
      const decoded = decodeFrames(buffered);
      const remaining = decoded.remaining;
      if (remaining.length > 0) {
        bufferedChunks.push(remaining);
      }
      buffered = remaining;

      for (const frame of decoded.messages) {
        if (frame.opcode === 0x8) {
          socket.end(encodeFrame("", 0x8));
          return;
        }
        if (frame.opcode === 0x9) {
          socket.write(encodeFrame(frame.text, 0xA));
          continue;
        }
        if (frame.opcode === 0xA) {
          continue;
        }
        if (frame.opcode === 0x0) {
          continue;
        }
        if (frame.opcode !== 0x1) {
          continue;
        }
        if (!frame.masked) {
          logger.warn("ws:unmasked_client_frame");
          socket.end(encodeFrame("", 0x8));
          return;
        }
        if (!checkRateLimit(socket)) {
          logger.warn("ws:rate_limit", "exceeded");
          continue;
        }
        handleClientMessage(socket, frame.text);
      }
    } catch (error) {
      logger.error("ws:decode_error", error && error.stack ? error.stack : String(error));
      sendJson(socket, { type: "error", data: "invalid_frame" });
    }
  });

  socket.on("error", (error) => {
    if (error && error.code === "ECONNRESET") {
      return;
    }
    logger.error("ws:socket_error", error && error.stack ? error.stack : String(error));
  });

  socket.on("close", () => {
    logger.info("ws:closed");
  });
});

server.listen(9999, "127.0.0.1", () => {
  logger.info("server:listening", "ws://127.0.0.1:9999");
});

function shutdown() {
  logger.info("server:shutdown", "Stopping app servers...");
  kernel.appStaticServer.stopAll();
  server.close(() => process.exit(0));
}

process.on("SIGINT", shutdown);
process.on("SIGTERM", shutdown);
