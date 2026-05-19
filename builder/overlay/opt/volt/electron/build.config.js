/**
 * electron-builder configuration for Samaris OS Desktop.
 * Builds the Electron shell for development (macOS) and production (Linux AppImage).
 */
const config = {
  appId: "tech.samaris.desktop",
  productName: "Samaris OS",
  directories: {
    output: "dist",
    buildResources: "build-resources",
  },
  files: [
    "main.js",
    "preload.js",
    "launcher.js",
    "ipc/**/*",
    "services/**/*",
    "package.json",
  ],
  extraResources: [
    {
      from: "../volt-kernel",
      to: "kernel",
      filter: ["**/*", "!node_modules"],
    },
    {
      from: "../overlay/opt/volt/desktop/app",
      to: "ui",
      filter: ["**/*"],
    },
    {
      from: "../ai-models",
      to: "ai-models",
      filter: ["**/*.gguf"],
    },
  ],
  asar: true,
  compression: "maximum",
  removePackageScripts: true,

  // Linux targets
  linux: {
    target: [
      {
        target: "AppImage",
        arch: ["x64"],
      },
    ],
    category: "Utility",
    icon: "../overlay/opt/volt/desktop/app/brand/samaris-logo.png",
    executableName: "samaris-desktop",
    synopsis: "Samaris OS - Native WebOS Desktop",
    description: "A portable, local-first Native WebOS desktop environment.",
  },

  // macOS development targets
  mac: {
    target: [
      {
        target: "dmg",
        arch: ["x64", "arm64"],
      },
    ],
    icon: "../overlay/opt/volt/desktop/app/brand/samaris-logo.png",
    category: "public.app-category.utilities",
    hardenedRuntime: true,
    gatekeeperAssess: false,
  },

  // AppImage configuration
  appImage: {
    systemIntegration: "doNotAsk",
    syncHermes: true,
  },

  // DMG configuration
  dmg: {
    contents: [
      { x: 130, y: 220, type: "file" },
      { x: 410, y: 220, type: "link", path: "/Applications" },
    ],
    window: { width: 540, height: 440 },
  },
};

module.exports = config;
