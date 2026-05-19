'use strict';

const { SbpMessage, HEADER_SIZE } = require('./message');
const { isSensitiveOpcode, requireCapability } = require('./permissions');
const { SBP_FLAGS, OPOCODES } = require('../constants');
const {
  ModuleOfflineError,
  ModuleTimeoutError,
  VoltUnifierError,
} = require('../errors');

const REVERSE_OPCODES = new Map();
for (const [name, code] of Object.entries(OPOCODES)) {
  REVERSE_OPCODES.set(code, name);
}

class SbpRouter {
  #unifier;
  #buffers;
  #pending;
  #subscriptions;

  constructor(unifier) {
    this.#unifier = unifier;
    this.#buffers = new Map();
    this.#pending = new Map();
    this.#subscriptions = new Map();
  }

  async send(moduleId, opcode, payload, options = {}) {
    const module = this.#unifier.getModule(moduleId);
    if (!module) {
      throw new ModuleOfflineError(moduleId, `Module ${moduleId} not found`);
    }
    if (module.status === 'offline') {
      throw new ModuleOfflineError(moduleId);
    }

    if (options.checkPermissions !== false) {
      this.#checkPermissions(moduleId, module, opcode);
    }

    const msg = SbpMessage.create(opcode, SBP_FLAGS.REQUEST, payload);
    const buffer = msg.encode();

    if (typeof module.transport.send !== 'function') {
      throw new VoltUnifierError(
        'TRANSPORT_ERROR',
        `Module ${moduleId} has no transport.send`,
        { module: moduleId }
      );
    }

    await module.transport.send(buffer);
  }

  async request(moduleId, opcode, payload, options = {}) {
    const timeout = options.timeout || 10000;
    const module = this.#unifier.getModule(moduleId);
    if (!module) {
      throw new ModuleOfflineError(moduleId, `Module ${moduleId} not found`);
    }
    if (module.status === 'offline') {
      throw new ModuleOfflineError(moduleId);
    }

    if (options.checkPermissions !== false) {
      this.#checkPermissions(moduleId, module, opcode);
    }

    const requestId = process.hrtime.bigint();
    const msg = SbpMessage.create(opcode, SBP_FLAGS.REQUEST, payload, requestId);
    const buffer = msg.encode();

    return new Promise((resolve, reject) => {
      const timer = setTimeout(() => {
        this.#pending.delete(requestId);
        reject(new ModuleTimeoutError(moduleId, timeout));
      }, timeout);

      this.#pending.set(requestId, {
        resolve,
        reject,
        timer,
        moduleId,
        opcode,
      });

      if (typeof module.transport.send !== 'function') {
        clearTimeout(timer);
        this.#pending.delete(requestId);
        reject(
          new VoltUnifierError(
            'TRANSPORT_ERROR',
            `Module ${moduleId} has no transport.send`,
            { module: moduleId }
          )
        );
        return;
      }

      module.transport.send(buffer).catch((err) => {
        clearTimeout(timer);
        this.#pending.delete(requestId);
        reject(
          new VoltUnifierError('TRANSPORT_ERROR', err.message, {
            module: moduleId,
            cause: err,
          })
        );
      });
    });
  }

  subscribe(moduleId, eventOpcode, handler) {
    if (typeof handler !== 'function') {
      throw new TypeError('Handler must be a function');
    }

    if (!this.#subscriptions.has(moduleId)) {
      this.#subscriptions.set(moduleId, new Map());
    }
    const moduleSubs = this.#subscriptions.get(moduleId);
    if (!moduleSubs.has(eventOpcode)) {
      moduleSubs.set(eventOpcode, new Set());
    }
    moduleSubs.get(eventOpcode).add(handler);

    return () => {
      const subs = moduleSubs.get(eventOpcode);
      if (subs) {
        subs.delete(handler);
        if (subs.size === 0) {
          moduleSubs.delete(eventOpcode);
        }
      }
      if (moduleSubs.size === 0) {
        this.#subscriptions.delete(moduleId);
      }
    };
  }

  handleIncoming(moduleId, rawBuffer) {
    const existing = this.#buffers.get(moduleId) || Buffer.alloc(0);
    const buf = Buffer.concat([existing, rawBuffer]);
    let processed = 0;

    while (processed < buf.length) {
      let msg;
      try {
        const result = SbpMessage.fromBuffer(buf.slice(processed));
        if (result === null) {
          break;
        }
        msg = result;
      } catch (err) {
        this.#publishError(moduleId, err);
        this.#buffers.delete(moduleId);
        return;
      }

      const messageSize = HEADER_SIZE + msg.payload.length;
      processed += messageSize;

      this.#handleMessage(moduleId, msg);
    }

    const remaining = buf.length - processed;
    if (remaining > 0) {
      this.#buffers.set(moduleId, Buffer.from(buf.slice(processed)));
    } else {
      this.#buffers.delete(moduleId);
    }
  }

  #checkPermissions(moduleId, module, opcode) {
    const opcodeName = REVERSE_OPCODES.get(opcode);
    if (!opcodeName) {
      return;
    }
    if (isSensitiveOpcode(opcode)) {
      const caps = module.capabilities || [];
      requireCapability(caps, opcodeName);
    }
  }

  #handleMessage(moduleId, msg) {
    if (msg.isResponse() || msg.isError()) {
      const pending = this.#pending.get(msg.requestId);
      if (pending) {
        clearTimeout(pending.timer);
        this.#pending.delete(msg.requestId);

        if (msg.isError()) {
          const errorText = msg.payload.length > 0
            ? msg.payload.toString('utf8')
            : 'Remote error';
          pending.reject(
            new VoltUnifierError('REMOTE_ERROR', errorText, {
              module: moduleId,
            })
          );
        } else {
          pending.resolve(msg.payload);
        }
      }
    }

    if (msg.isEvent()) {
      this.#routeEvent(moduleId, msg);

      const eventBus = this.#unifier.systemEventBus;
      if (eventBus && typeof eventBus.publish === 'function') {
        eventBus.publish({
          type: 'SBP_EVENT',
          source: moduleId,
          severity: 'info',
          timestamp: Date.now(),
          payload: {
            opcode: msg.opcode,
            opcodeName: REVERSE_OPCODES.get(msg.opcode) || null,
            data: msg.payload,
          },
        });
      }
    }
  }

  #routeEvent(moduleId, message) {
    const moduleSubs = this.#subscriptions.get(moduleId);
    if (!moduleSubs) {
      return;
    }
    const handlers = moduleSubs.get(message.opcode);
    if (!handlers) {
      return;
    }
    for (const handler of handlers) {
      try {
        handler(message);
      } catch (err) {
        this.#publishError(moduleId, err);
      }
    }
  }

  #publishError(moduleId, err) {
    const eventBus = this.#unifier.systemEventBus;
    if (eventBus && typeof eventBus.publish === 'function') {
      eventBus.publish({
        type: 'SERVICE_ERROR',
        source: 'sbp-router',
        severity: 'error',
        timestamp: Date.now(),
        payload: {
          error: err.message,
          moduleId,
          code: err.code || null,
        },
      });
    }
  }
}

module.exports = { SbpRouter };
