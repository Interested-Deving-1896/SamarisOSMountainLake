'use strict';
const { ServiceHealthClient } = require('./serviceHealthClient');
const { OPOCODES } = require('../constants');

class VgmClient extends ServiceHealthClient {
  constructor(unifier) {
    super(unifier, 'vgm', unifier.config.modules.vgm);
  }

  async status() {
    return this.request(OPOCODES.GPU_STATUS, Buffer.alloc(0));
  }

  async allocResource(resource) {
    return this.request(OPOCODES.GPU_ALLOC_RESOURCE, Buffer.from(JSON.stringify(resource)));
  }

  async freeResource(resourceId) {
    return this.request(OPOCODES.GPU_FREE_RESOURCE, Buffer.from(String(resourceId)));
  }

  async execCompute(job) {
    return this.request(OPOCODES.GPU_EXEC_COMPUTE, Buffer.from(JSON.stringify(job)));
  }

  async renderFrame(frame) {
    return this.request(OPOCODES.GPU_RENDER_FRAME, Buffer.from(JSON.stringify(frame)));
  }

  async thermalStatus() {
    return this.request(OPOCODES.GPU_THERMAL_STATUS, Buffer.alloc(0));
  }

  async switchDevice(policy) {
    return this.request(OPOCODES.GPU_SWITCH_DEVICE, Buffer.from(JSON.stringify({ policy })));
  }

  async shaderCompile(shader) {
    return this.request(OPOCODES.GPU_SHADER_COMPILE, Buffer.from(JSON.stringify(shader)));
  }

  async vramStatus() {
    return this.request(OPOCODES.GPU_VRAM_STATUS, Buffer.alloc(0));
  }

  async batchSubmit(batch) {
    return this.request(OPOCODES.GPU_BATCH_SUBMIT, Buffer.from(JSON.stringify(batch)));
  }

  async prefetchShaders() {
    return this.request(OPOCODES.GPU_PREFETCH_SHADERS, Buffer.alloc(0));
  }

  async compressResource(resourceId) {
    return this.request(OPOCODES.GPU_COMPRESS_RESOURCE, Buffer.from(String(resourceId)));
  }

  async restoreResource(resourceId) {
    return this.request(OPOCODES.GPU_RESTORE_RESOURCE, Buffer.from(String(resourceId)));
  }

  async evictResource(resourceId) {
    return this.request(OPOCODES.GPU_EVICT_RESOURCE, Buffer.from(String(resourceId)));
  }

  async metricsSnapshot() {
    return this.request(OPOCODES.GPU_METRICS_SNAPSHOT, Buffer.alloc(0));
  }
}

module.exports = { VgmClient };
