'use strict';
const { describe, it, before, after } = require('node:test');
const assert = require('node:assert');
const { EventEmitter } = require('node:events');

describe('SystemEventBus', () => {
  function makeKernelBus() {
    const ee = new EventEmitter();
    ee.emit = ee.emit.bind(ee);
    return ee;
  }

  it('requires kernelEventBus with emit', () => {
    const { SystemEventBus } = require('../events/eventBus');
    assert.throws(
      () => new SystemEventBus(null),
      /must expose an emit/
    );
    assert.throws(
      () => new SystemEventBus({}),
      /must expose an emit/
    );
  });

  it('publish/subscribe delivers events', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    const events = [];
    bus.subscribe('TEST_EVENT', (ev) => events.push(ev));

    bus.publish({
      id: 'evt-1', type: 'TEST_EVENT', source: 'test',
      severity: 'info', timestamp: 1000, payload: { value: 42 },
    });

    assert.strictEqual(events.length, 1);
    assert.strictEqual(events[0].payload.value, 42);
  });

  it('unsubscribe removes handler', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    const events = [];
    const id = bus.subscribe('TEST_EVENT', (ev) => events.push(ev));

    bus.publish({
      id: 'evt-1', type: 'TEST_EVENT', source: 'test',
      severity: 'info', timestamp: 1000, payload: {},
    });
    assert.strictEqual(events.length, 1);

    bus.unsubscribe(id);
    bus.publish({
      id: 'evt-2', type: 'TEST_EVENT', source: 'test',
      severity: 'info', timestamp: 1001, payload: {},
    });
    assert.strictEqual(events.length, 1);
  });

  it('unsubscribe returns false for invalid id', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());
    assert.strictEqual(bus.unsubscribe('nonexistent'), false);
    assert.strictEqual(bus.unsubscribe(null), false);
  });

  it('event history is bounded', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    for (let i = 0; i < 600; i++) {
      bus.publish({
        id: `evt-${i}`, type: 'TEST_EVENT', source: 'test',
        severity: 'info', timestamp: i, payload: {},
      });
    }

    const hist = bus.history(1000);
    assert.ok(hist.length <= 500);
    assert.strictEqual(hist.length, 500);
  });

  it('history returns last N events', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    for (let i = 0; i < 10; i++) {
      bus.publish({
        id: `evt-${i}`, type: 'TEST_EVENT', source: 'test',
        severity: 'info', timestamp: i, payload: { idx: i },
      });
    }

    const hist = bus.history(3);
    assert.strictEqual(hist.length, 3);
    assert.strictEqual(hist[0].payload.idx, 7);
    assert.strictEqual(hist[2].payload.idx, 9);
  });

  it('historyByType filters events', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    bus.publish({
      id: 'evt-1', type: 'TYPE_A', source: 'test',
      severity: 'info', timestamp: 1, payload: {},
    });
    bus.publish({
      id: 'evt-2', type: 'TYPE_B', source: 'test',
      severity: 'info', timestamp: 2, payload: {},
    });
    bus.publish({
      id: 'evt-3', type: 'TYPE_A', source: 'test',
      severity: 'info', timestamp: 3, payload: {},
    });

    const filtered = bus.historyByType('TYPE_A');
    assert.strictEqual(filtered.length, 2);
  });

  it('event normalization produces valid VoltEvent', () => {
    const { normalizeSbpEvent } = require('../events/normalizer');
    const { OPOCODES } = require('../constants');

    const event = normalizeSbpEvent('vrm', OPOCODES.RAM_PRESSURE_EVENT, Buffer.from('{"pressure":0.8}'));
    assert.ok(event);
    assert.strictEqual(event.type, 'RAM_PRESSURE');
    assert.strictEqual(event.source, 'vrm');
    assert.strictEqual(event.severity, 'warning');
    assert.strictEqual(event.payload.pressure, 0.8);
  });

  it('event normalization handles USB events', () => {
    const { normalizeSbpEvent } = require('../events/normalizer');
    const { OPOCODES } = require('../constants');

    const event = normalizeSbpEvent('vum', OPOCODES.USB_DEVICE_EVENT, Buffer.from('{"device":"sda1"}'));
    assert.ok(event);
    assert.strictEqual(event.type, 'USB_DEVICE');
    assert.strictEqual(event.source, 'vum');
  });

  it('event normalization returns null for unknown opcode', () => {
    const { normalizeSbpEvent } = require('../events/normalizer');
    const result = normalizeSbpEvent('test', 0xFF, Buffer.alloc(0));
    assert.strictEqual(result, null);
  });

  it('event normalization returns null for invalid moduleId', () => {
    const { normalizeSbpEvent } = require('../events/normalizer');
    const { OPOCODES } = require('../constants');
    assert.strictEqual(normalizeSbpEvent(null, OPOCODES.RAM_PRESSURE_EVENT, null), null);
    assert.strictEqual(normalizeSbpEvent('', OPOCODES.RAM_PRESSURE_EVENT, null), null);
  });

  it('routing table lookup returns routes', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    const routes = bus.getRouting('RAM_PRESSURE');
    assert.ok(Array.isArray(routes));
    assert.ok(routes.length > 0);
  });

  it('setRouting overrides routes for event type', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    bus.setRouting('CUSTOM_EVENT', ['foo', 'bar']);
    assert.deepStrictEqual(bus.getRouting('CUSTOM_EVENT'), ['foo', 'bar']);
  });

  it('setRouting validates input', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    assert.throws(() => bus.setRouting('', ['a']), /non-empty string/);
    assert.throws(() => bus.setRouting('T', 'not-array'), /must be an array/);
  });

  it('subscribeOnce fires once then unsubscribes', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    let count = 0;
    bus.subscribeOnce('ONCE_EVENT', () => { count++; });

    bus.publish({
      id: 'evt-1', type: 'ONCE_EVENT', source: 'test',
      severity: 'info', timestamp: 1, payload: {},
    });
    bus.publish({
      id: 'evt-2', type: 'ONCE_EVENT', source: 'test',
      severity: 'info', timestamp: 2, payload: {},
    });

    assert.strictEqual(count, 1);
  });

  it('subscribe validates arguments', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    assert.throws(() => bus.subscribe('', () => {}), /non-empty string/);
    assert.throws(() => bus.subscribe('T', null), /must be a function/);
  });

  it('publish with invalid event throws', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());
    assert.throws(() => bus.publish({}), /invalid VoltEvent/);
  });

  it('subscriberCount returns correct count', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    bus.subscribe('EVENT_A', () => {});
    bus.subscribe('EVENT_A', () => {});
    bus.subscribe('EVENT_B', () => {});

    assert.strictEqual(bus.subscriberCount('EVENT_A'), 2);
    assert.strictEqual(bus.subscriberCount('EVENT_B'), 1);
    assert.strictEqual(bus.subscriberCount('NONEXISTENT'), 0);
  });

  it('clear removes all history', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    bus.publish({
      id: 'evt-1', type: 'T', source: 'test',
      severity: 'info', timestamp: 1, payload: {},
    });
    assert.strictEqual(bus.history(10).length, 1);

    bus.clear();
    assert.strictEqual(bus.history(10).length, 0);
  });

  it('handler errors do not crash publish', () => {
    const { SystemEventBus } = require('../events/eventBus');
    const bus = new SystemEventBus(makeKernelBus());

    bus.subscribe('ERR_EVENT', () => { throw new Error('handler failed'); });
    bus.subscribe('ERR_EVENT', (ev) => { assert.ok(ev); });

    assert.doesNotThrow(() => {
      bus.publish({
        id: 'evt-1', type: 'ERR_EVENT', source: 'test',
        severity: 'info', timestamp: 1, payload: {},
      });
    });
  });
});
