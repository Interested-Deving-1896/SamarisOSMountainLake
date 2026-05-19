'use strict';

const DEFAULT_CAPABILITIES = {
  'kernel-b': {
    features: ['compute', 'gpu-render', 'thermal-monitor'],
    permissions: ['compute.execute', 'gpu.render'],
  },
  vrm: {
    features: ['memory-management', 'compression', 'dedup', 'pressure-events'],
    permissions: ['memory.*'],
  },
  vum: {
    features: ['storage', 'journal', 'writeback', 'fuse'],
    permissions: ['storage.*'],
  },
  vgm: {
    features: ['compute', 'render', 'shader-compile', 'vram-management'],
    permissions: ['gpu.*'],
  },
  dwp: {
    features: ['scheduling', 'adaptive-scaling', 'orbit-burst'],
    permissions: ['schedule.*'],
  },
  asc: {
    features: ['hardware-probe', 'config-generation', 'budget-allocation'],
    permissions: ['system.config'],
  },
};

function getDefaultCapabilities(moduleId) {
  if (!moduleId || typeof moduleId !== 'string') {
    return null;
  }
  const caps = DEFAULT_CAPABILITIES[moduleId];
  if (!caps) {
    return null;
  }
  return {
    features: Array.from(caps.features),
    permissions: Array.from(caps.permissions),
  };
}

function hasFeature(moduleId, feature) {
  const caps = DEFAULT_CAPABILITIES[moduleId];
  if (!caps) {
    return false;
  }
  return caps.features.includes(feature);
}

function hasPermission(moduleId, permission) {
  const caps = DEFAULT_CAPABILITIES[moduleId];
  if (!caps) {
    return false;
  }
  return caps.permissions.some(p => {
    if (p.endsWith('.*')) {
      const prefix = p.slice(0, -1);
      return permission.startsWith(prefix);
    }
    return p === permission;
  });
}

module.exports = { getDefaultCapabilities, hasFeature, hasPermission };
