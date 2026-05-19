'use strict';
const { EVENT_TYPES, HEALTH_TIMEOUT_MULTIPLIER, HEARTBEAT_DEFAULTS, MODULE_STATUSES } = require('../constants');
const { createEvent } = require('../events/eventTypes');

const HEALTH_CHECK_INTERVAL_MS = 2000;

class HealthMonitor {
  constructor(unifier) {
    this.unifier = unifier;
    this._timer = null;
    this._warnings = [];
    this._criticalIssues = [];
    this._overallStatus = 'unknown';
  }

  start() {
    if (this._timer) return;
    this._runCheck();
    this._timer = setInterval(() => this._runCheck(), HEALTH_CHECK_INTERVAL_MS);
  }

  stop() {
    if (this._timer) {
      clearInterval(this._timer);
      this._timer = null;
    }
  }

  _runCheck() {
    const warnings = [];
    const criticalIssues = [];
    let worstStatus = 'online';
    const modules = this.unifier.registry.getAll();

    for (const entry of modules) {
      const moduleConfig = this.unifier.config.modules[entry.id];
      if (moduleConfig && moduleConfig.enabled === false) continue;

      const heartbeatMs = (moduleConfig && moduleConfig.heartbeatMs) || HEARTBEAT_DEFAULTS[entry.id] || 5000;
      const timeoutMs = heartbeatMs * HEALTH_TIMEOUT_MULTIPLIER;
      const now = Date.now();

      if (entry.health.status === 'online' || entry.health.status === 'degraded') {
        if (entry.health.lastHeartbeatAt) {
          const elapsed = now - entry.health.lastHeartbeatAt;
          if (elapsed > timeoutMs) {
            this.unifier.registry.updateStatus(entry.id, 'degraded', {
              degradedReason: `no heartbeat for ${elapsed}ms (timeout: ${timeoutMs}ms)`,
            });
            warnings.push(`${entry.id}: missed heartbeat (${elapsed}ms since last)`);
            if (worstStatus !== 'fatal' && worstStatus !== 'error') {
              worstStatus = 'degraded';
            }
          }
        } else {
          warnings.push(`${entry.id}: no heartbeat received yet`);
          if (worstStatus !== 'fatal' && worstStatus !== 'error') {
            worstStatus = 'degraded';
          }
        }
      }

      if (entry.health.status === 'error' || entry.health.status === 'fatal') {
        criticalIssues.push(`${entry.id}: ${entry.health.status} — ${entry.health.lastError || 'unknown error'}`);
        worstStatus = entry.health.status === 'fatal' ? 'fatal' : 'error';
      }

      if (entry.health.status === 'offline' && moduleConfig && moduleConfig.enabled !== false) {
        warnings.push(`${entry.id}: offline`);
        if (worstStatus !== 'fatal' && worstStatus !== 'error') {
          worstStatus = 'degraded';
        }
      }
    }

    if (criticalIssues.length > 0 && worstStatus === 'degraded') {
      worstStatus = 'error';
    }
    if (warnings.length === 0 && criticalIssues.length === 0) {
      worstStatus = 'online';
    }

    this._warnings = warnings;
    this._criticalIssues = criticalIssues;
    this._overallStatus = worstStatus;

    const healthEntry = (id) => {
      const m = this.unifier.registry.get(id);
      return m ? { ...m.health } : { status: 'offline', lastHeartbeatAt: null, latencyMs: null, capabilities: [], errorCount: 0, reconnectCount: 0, lastError: null, degradedReason: null };
    };

    const eventPayload = {
      overallStatus: worstStatus,
      warnings,
      criticalIssues,
      kernelB: healthEntry('kernel-b'),
      vrm: healthEntry('vrm'),
      vum: healthEntry('vum'),
      vgm: healthEntry('vgm'),
      dwp: healthEntry('dwp'),
      asc: healthEntry('asc'),
    };

    this.unifier.eventBus.publish(
      createEvent(EVENT_TYPES.MODULE_HEALTH, 'health-monitor', eventPayload,
        worstStatus === 'fatal' ? 'critical' : worstStatus === 'error' ? 'error' : warnings.length > 0 ? 'warning' : 'info')
    );
  }

  getSystemHealthSnapshot() {
    const healthEntry = (id) => {
      const m = this.unifier.registry.get(id);
      return m ? { ...m.health } : { status: 'offline', lastHeartbeatAt: null, latencyMs: null, capabilities: [], errorCount: 0, reconnectCount: 0, lastError: null, degradedReason: null };
    };

    return {
      overallStatus: this._overallStatus,
      warnings: Array.from(this._warnings),
      criticalIssues: Array.from(this._criticalIssues),
      kernelB: healthEntry('kernel-b'),
      vrm: healthEntry('vrm'),
      vum: healthEntry('vum'),
      vgm: healthEntry('vgm'),
      dwp: healthEntry('dwp'),
      asc: healthEntry('asc'),
    };
  }
}

module.exports = { HealthMonitor };
