'use strict';

/**
 * @typedef {Object} VoltUnifierConfig
 * @property {boolean} enabled
 * @property {string} mode
 * @property {boolean} debugHttpEnabled
 * @property {string} debugHttpBind
 * @property {number} debugHttpPort
 * @property {TransportConfig} transport
 * @property {Object.<string, ModuleEndpointConfig>} modules
 * @property {SafetyConfig} safety
 */

/**
 * @typedef {Object} TransportConfig
 * @property {boolean} unixSocketEnabled
 * @property {boolean} shmEnabled
 * @property {boolean} websocketDebugEnabled
 * @property {string} socketDir
 * @property {string} shmPath
 * @property {number} shmSizeMb
 */

/**
 * @typedef {Object} ModuleEndpointConfig
 * @property {boolean} enabled
 * @property {string} [socket]
 * @property {string} protocol
 * @property {number} [heartbeatMs]
 * @property {string} [mode]
 * @property {string} [binary]
 * @property {string} [generatedConfig]
 */

/**
 * @typedef {Object} SafetyConfig
 * @property {boolean} requireLocalhostDebug
 * @property {boolean} requireCapabilities
 * @property {boolean} denyPublicBind
 * @property {boolean} shutdownRequiresCleanVum
 */

/**
 * @typedef {'kernel-b'|'vrm'|'vum'|'vgm'|'dwp'|'asc'|'volt-boot'} VoltModuleId
 */

/**
 * @typedef {'SBP_V5'|'SBP_MEM'|'SBP_USB'|'SBP_GPU'|'CLI'} ModuleProtocol
 */

/**
 * @typedef {'offline'|'starting'|'connecting'|'online'|'degraded'|'recovering'|'error'|'fatal'} VoltModuleStatus
 */

/**
 * @typedef {Object} VoltModuleHealth
 * @property {VoltModuleStatus} status
 * @property {number} lastHeartbeatAt
 * @property {number} latencyMs
 * @property {string[]} capabilities
 * @property {number} errorCount
 * @property {number} reconnectCount
 * @property {string|null} lastError
 * @property {string|null} degradedReason
 * @property {string} [ascConfigPath]
 * @property {string} [generatedConfig]
 */

/**
 * @typedef {Object} SbpHeader
 * @property {number} magic
 * @property {number} version
 * @property {number} opcode
 * @property {number} flags
 * @property {bigint} requestId
 * @property {bigint} timestampUs
 * @property {number} payloadLen
 * @property {number} checksum
 */

/**
 * @typedef {Object} SbpMessage
 * @property {SbpHeader} header
 * @property {Buffer} payload
 */

/**
 * @typedef {Object} VoltEvent
 * @property {string} id
 * @property {string} type
 * @property {string} source
 * @property {string} severity
 * @property {number} timestamp
 * @property {*} payload
 */

/**
 * @typedef {Object} DashboardSnapshot
 * @property {number} timestamp
 * @property {SystemHealthSnapshot} health
 * @property {Object} [ram]
 * @property {Object} [usb]
 * @property {Object} [gpu]
 * @property {Object} [workers]
 * @property {Object} [adaptive]
 * @property {VoltEvent[]} events
 */

/**
 * @typedef {Object} SystemHealthSnapshot
 * @property {VoltModuleHealth} kernelB
 * @property {VoltModuleHealth} vrm
 * @property {VoltModuleHealth} vum
 * @property {VoltModuleHealth} vgm
 * @property {VoltModuleHealth} dwp
 * @property {VoltModuleHealth} asc
 * @property {string} overallStatus
 * @property {string[]} warnings
 * @property {string[]} criticalIssues
 */

/**
 * @typedef {Object} ModuleCapabilities
 * @property {VoltModuleId} module
 * @property {string} protocolVersion
 * @property {string[]} features
 * @property {string[]} permissions
 */

/**
 * @typedef {Object} AuditEvent
 * @property {number} timestamp
 * @property {string} action
 * @property {string} moduleId
 * @property {boolean} allowed
 * @property {string} reason
 * @property {string} [source]
 */

module.exports = {};
