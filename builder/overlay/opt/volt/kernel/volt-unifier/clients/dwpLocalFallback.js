'use strict';

class DwpLocalFallback {
  constructor(unifier) {
    this.unifier = unifier;
    this._queue = [];
    this._desktopPressure = 0;
    this._burstActive = false;
    this._burstStart = 0;
    this._burstCount = 0;
    this._stats = { jobsSubmitted: 0, jobsCompleted: 0, bursts: 0, yields: 0, preemptions: 0 };
  }

  submitJob(job) {
    this._stats.jobsSubmitted++;
    this._queue.push({ ...job, submittedAt: Date.now(), state: 'queued' });
    this._reorderQueue();
    return { ok: true, mode: 'local_fallback', jobId: job.id || `local-${Date.now()}` };
  }

  setDesktopPressure(pressure) {
    this._desktopPressure = Math.max(0, Math.min(1, pressure));
    if (pressure > 0.7) {
      this._queue = this._queue.filter(j => j.priority !== 'idle');
    }
  }

  requestOrbitBurst(request) {
    const now = Date.now();
    if (this._burstActive && (now - this._burstStart) < 500) {
      return { accepted: false, reason: 'burst_window_active' };
    }
    if (this._burstCount >= 2 && (now - this._burstStart) < 2000) {
      return { accepted: false, reason: 'max_consecutive_bursts_reached' };
    }
    if (this._desktopPressure > 0.6) {
      return { accepted: false, reason: 'desktop_pressure_too_high' };
    }
    this._burstActive = true;
    this._burstStart = now;
    this._burstCount++;
    this._stats.bursts++;
    this._queue.forEach(j => {
      if (j.source === 'orbit' || j.priority === 'critical') j.priority = 'critical';
    });
    this._reorderQueue();
    return { accepted: true, burstWindowMs: 500, mode: 'local_fallback' };
  }

  metrics() {
    return {
      mode: 'local_fallback',
      currentWorkers: 2,
      maxWorkers: 4,
      queueDepth: this._queue.filter(j => j.state === 'queued').length,
      desktopPressure: this._desktopPressure,
      burstActive: this._burstActive,
      ...this._stats,
    };
  }

  getSnapshot() {
    return {
      queueDepth: this._queue.length,
      desktopPressure: this._desktopPressure,
      burstActive: this._burstActive,
      burstCount: this._burstCount,
      ...this._stats,
    };
  }

  _reorderQueue() {
    const priorityOrder = { critical: 0, high: 1, normal: 2, idle: 3 };
    this._queue.sort((a, b) => {
      const pa = priorityOrder[a.priority] ?? 2;
      const pb = priorityOrder[b.priority] ?? 2;
      if (pa !== pb) return pa - pb;
      return (a.submittedAt || 0) - (b.submittedAt || 0);
    });
  }
}

module.exports = { DwpLocalFallback };
