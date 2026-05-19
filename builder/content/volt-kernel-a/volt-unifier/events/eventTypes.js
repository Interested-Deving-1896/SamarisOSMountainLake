'use strict';

const { EVENT_TYPES, EVENT_SEVERITIES } = require('../constants');

let _eventId = 0;

function createEvent(type, source, payload, severity) {
  if (!type || typeof type !== 'string') {
    throw new Error('Event type must be a non-empty string');
  }
  if (!source || typeof source !== 'string') {
    throw new Error('Event source must be a non-empty string');
  }

  const resolvedSeverity = severity || 'info';
  if (!EVENT_SEVERITIES.includes(resolvedSeverity)) {
    throw new Error(`Invalid event severity: ${resolvedSeverity}`);
  }

  return {
    id: `evt-${Date.now()}-${++_eventId}`,
    type,
    source,
    severity: resolvedSeverity,
    timestamp: Date.now(),
    payload: payload !== undefined ? payload : null,
  };
}

function isVoltEvent(obj) {
  return (
    obj &&
    typeof obj === 'object' &&
    typeof obj.id === 'string' &&
    typeof obj.type === 'string' &&
    typeof obj.source === 'string' &&
    typeof obj.severity === 'string' &&
    typeof obj.timestamp === 'number'
  );
}

module.exports = { createEvent, isVoltEvent, EVENT_TYPES };
