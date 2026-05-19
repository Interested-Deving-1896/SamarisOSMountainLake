'use strict';

const { OPOCODES, EVENT_TYPES } = require('../constants');
const { createEvent } = require('./eventTypes');

const OPC_TO_EVENT = {
  [OPOCODES.RAM_PRESSURE_EVENT]: EVENT_TYPES.RAM_PRESSURE,
  [OPOCODES.RAM_GC_SIGNAL]: EVENT_TYPES.RAM_GC_REQUEST,
  [OPOCODES.USB_WRITE_ACK_EVENT]: EVENT_TYPES.USB_DURABILITY,
  [OPOCODES.USB_DEVICE_EVENT]: EVENT_TYPES.USB_DEVICE,
  [OPOCODES.GPU_THERMAL_STATUS]: EVENT_TYPES.GPU_THERMAL,
  [OPOCODES.GPU_RENDER_FRAME]: EVENT_TYPES.GPU_FRAME_PRESSURE,
  [OPOCODES.HEARTBEAT]: EVENT_TYPES.KERNEL_HEARTBEAT,
};

const OPC_TO_SEVERITY = {
  [OPOCODES.RAM_PRESSURE_EVENT]: 'warning',
  [OPOCODES.RAM_GC_SIGNAL]: 'info',
  [OPOCODES.USB_WRITE_ACK_EVENT]: 'info',
  [OPOCODES.USB_DEVICE_EVENT]: 'info',
  [OPOCODES.GPU_THERMAL_STATUS]: 'warning',
  [OPOCODES.GPU_RENDER_FRAME]: 'info',
  [OPOCODES.HEARTBEAT]: 'debug',
};

function normalizeSbpEvent(moduleId, opcode, rawPayload) {
  if (!moduleId || typeof moduleId !== 'string') {
    return null;
  }
  if (opcode === undefined || opcode === null || typeof opcode !== 'number') {
    return null;
  }

  const eventType = OPC_TO_EVENT[opcode];
  if (!eventType) {
    return null;
  }

  const severity = OPC_TO_SEVERITY[opcode] || 'info';

  let payload = null;
  if (rawPayload !== undefined && rawPayload !== null) {
    if (Buffer.isBuffer(rawPayload)) {
      try {
        payload = JSON.parse(rawPayload.toString('utf8'));
      } catch (_) {
        payload = { raw: rawPayload.toString('base64') };
      }
    } else if (typeof rawPayload === 'object') {
      payload = rawPayload;
    } else {
      payload = { value: rawPayload };
    }
  }

  return createEvent(eventType, moduleId, payload, severity);
}

module.exports = { normalizeSbpEvent };
