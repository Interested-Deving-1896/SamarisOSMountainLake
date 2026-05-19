'use strict';
const { describe, it, before, after } = require('node:test');
const assert = require('node:assert');
const { SBP_FLAGS, OPOCODES } = require('../constants');
const { SbpMessage } = require('../sbp/message');

describe('SbpRouter', () => {
  function makeMockTransport() {
    const sent = [];
    let sendImpl;
    return {
      sent,
      send(buf) {
        sent.push(buf);
        if (sendImpl) return sendImpl(buf);
        return Promise.resolve();
      },
      _setSendImpl(fn) { sendImpl = fn; },
    };
  }

  function makeMockModule(status, caps, transport) {
    return { status, capabilities: caps || [], transport };
  }

  function makeMockUnifier(modules) {
    const publishedEvents = [];
    return {
      getModule(id) { return modules[id] || null; },
      systemEventBus: {
        publish(ev) { publishedEvents.push(ev); },
      },
      _published: publishedEvents,
    };
  }

  it('routes send to correct module', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await router.send('vrm', OPOCODES.RAM_STATUS, Buffer.from('test'));

    assert.strictEqual(transport.sent.length, 1);
    const msg = SbpMessage.fromBuffer(transport.sent[0]);
    assert.strictEqual(msg.opcode, OPOCODES.RAM_STATUS);
    assert.strictEqual(msg.payload.toString(), 'test');
  });

  it('rejects unknown module', async () => {
    const { SbpRouter } = require('../sbp/router');
    const router = new SbpRouter(makeMockUnifier({}));
    await assert.rejects(
      router.send('nonexistent', 0x01, Buffer.alloc(0)),
      /not found/
    );
  });

  it('rejects offline module', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('offline', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await assert.rejects(
      router.send('vrm', 0x01, Buffer.alloc(0)),
      /offline/
    );
  });

  it('permission check rejects sensitive opcode without capability', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await assert.rejects(
      router.send('vrm', OPOCODES.RAM_SET_QUOTA, Buffer.alloc(0)),
      /missing capability/
    );
  });

  it('permission check allows sensitive opcode with capability', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', ['RAM_SET_QUOTA'], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await router.send('vrm', OPOCODES.RAM_SET_QUOTA, Buffer.alloc(0));
    assert.strictEqual(transport.sent.length, 1);
  });

  it('permission check can be skipped with option', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await router.send('vrm', OPOCODES.RAM_SET_QUOTA, Buffer.alloc(0), { checkPermissions: false });
    assert.strictEqual(transport.sent.length, 1);
  });

  it('timeout handling on request', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    transport._setSendImpl(() => new Promise(() => {}));
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await assert.rejects(
      router.request('vrm', OPOCODES.RAM_STATUS, Buffer.alloc(0), { timeout: 100 }),
      /timed out/
    );
  });

  it('request resolves when handleIncoming receives response', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    const promise = router.request('vrm', OPOCODES.RAM_STATUS, Buffer.from('req'), { timeout: 5000 });

    assert.strictEqual(transport.sent.length, 1);
    const sentMsg = SbpMessage.fromBuffer(transport.sent[0]);

    const respMsg = SbpMessage.create(
      OPOCODES.RAM_STATUS,
      SBP_FLAGS.RESPONSE,
      Buffer.from('response-data'),
      sentMsg.requestId,
    );
    const respBuf = respMsg.encode();

    router.handleIncoming('vrm', respBuf);
    const result = await promise;
    assert.strictEqual(result.toString(), 'response-data');
  });

  it('incoming event published to eventBus', () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    const eventMsg = SbpMessage.create(
      OPOCODES.RAM_PRESSURE_EVENT,
      SBP_FLAGS.EVENT,
      Buffer.from('{"pressure":0.9}'),
    );
    router.handleIncoming('vrm', eventMsg.encode());

    assert.strictEqual(unifier._published.length, 1);
    assert.strictEqual(unifier._published[0].type, 'SBP_EVENT');
    assert.strictEqual(unifier._published[0].source, 'vrm');
    assert.strictEqual(unifier._published[0].payload.opcode, OPOCODES.RAM_PRESSURE_EVENT);
  });

  it('subscribe routes events to handler', () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    const received = [];
    router.subscribe('vrm', OPOCODES.RAM_PRESSURE_EVENT, (msg) => {
      received.push(msg);
    });

    const eventMsg = SbpMessage.create(
      OPOCODES.RAM_PRESSURE_EVENT,
      SBP_FLAGS.EVENT,
      Buffer.from('{"pressure":0.5}'),
    );
    router.handleIncoming('vrm', eventMsg.encode());

    assert.strictEqual(received.length, 1);
    assert.strictEqual(received[0].payload.toString(), '{"pressure":0.5}');
  });

  it('unsubscribe removes event handler', () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    let count = 0;
    const unsubscribe = router.subscribe('vrm', OPOCODES.RAM_PRESSURE_EVENT, () => { count++; });

    unsubscribe();
    const eventMsg = SbpMessage.create(OPOCODES.RAM_PRESSURE_EVENT, SBP_FLAGS.EVENT, Buffer.alloc(0));
    router.handleIncoming('vrm', eventMsg.encode());

    assert.strictEqual(count, 0);
  });

  it('subscribe rejects non-function handler', () => {
    const { SbpRouter } = require('../sbp/router');
    const router = new SbpRouter(makeMockUnifier({}));
    assert.throws(
      () => router.subscribe('vrm', 0x01, 'not-a-function'),
      /must be a function/
    );
  });

  it('transport error on send rejects', async () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    transport._setSendImpl(() => Promise.reject(new Error('socket closed')));
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    await assert.rejects(
      router.send('vrm', 0x01, Buffer.alloc(0)),
      /socket closed/
    );
  });

  it('handleIncoming buffers partial messages', () => {
    const { SbpRouter } = require('../sbp/router');
    const transport = makeMockTransport();
    const modules = { vrm: makeMockModule('online', [], transport) };
    const unifier = makeMockUnifier(modules);
    const router = new SbpRouter(unifier);

    const eventMsg = SbpMessage.create(OPOCODES.RAM_STATUS, SBP_FLAGS.EVENT, Buffer.alloc(0));
    const fullBuf = eventMsg.encode();
    const partial = fullBuf.slice(0, 10);

    assert.doesNotThrow(() => {
      router.handleIncoming('vrm', partial);
    });
  });
});
