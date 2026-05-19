import React from "react";
import Finder from "../../apps/Finder";
import Mail from "../../apps/Mail";
import Music from "../../apps/Music";
import Network from "../../apps/Network";
import Notes from "../../apps/Notes";
import Peregrine from "../../apps/Peregrine";
import Photos from "../../apps/Photos";
import PdfViewer from "../../apps/PdfViewer";
import Print from "../../apps/Print";
import PermissionsManager from "../../apps/PermissionsManager";
import Firewall from "../../apps/Firewall";
import AppStore from "../../apps/AppStore";
import Encryption from "../../apps/Encryption";
import Videos from "../../apps/Videos";
import Doom from "../../apps/Doom";
import Utilities from "../../apps/Utilities";
import Settings from "../../apps/Settings";
import TextEditor from "../../apps/TextEditor";
import Trash from "../../apps/Trash";
import InstalledWebApp from "../../apps/InstalledWebApp";
import { WineLauncherApp } from "../../modules/wine";
import Terminal from "../../apps/Terminal";
import Downloads from "../../apps/Downloads";
import About from "../../apps/About";
import Archive from "../../apps/Archive";
import Bench from "../../apps/Bench";

const Orbit = React.lazy(() => import("../../apps/Orbit"));

export type OSAppDefinition = {
  id: string;
  name: string;
  icon: string;
  runtime: "app" | "sandbox" | "browser";
  title: string;
  subtitle?: string;
  accent?: number;
  supportsDuplicate?: boolean;
  hiddenFromDock?: boolean;
  minWindow?: {
    w: number;
    h: number;
  };
  defaultWindow: {
    x: number;
    y: number;
    w: number;
    h: number;
  };
  component:
    | React.ComponentType<{ windowId: string }>
    | React.LazyExoticComponent<React.ComponentType<{ windowId: string }>>;
};

export const appRegistry: Record<string, OSAppDefinition> = {
  about: {
    id: "about",
    name: "About",
    icon: "AB",
    runtime: "app",
    title: "About Samaris OS",
    subtitle: "Version & credits",
    accent: 220,
    defaultWindow: { x: 120, y: 60, w: 640, h: 640 },
    component: About,
  },
  archive: {
    id: "archive",
    name: "Archive",
    icon: "AR",
    runtime: "app",
    title: "Archive",
    subtitle: "Extract archives",
    accent: 30,
    defaultWindow: { x: 180, y: 80, w: 720, h: 560 },
    component: Archive,
  },
  finder: {
    id: "finder",
    name: "Finder",
    icon: "FI",
    runtime: "app",
    title: "Finder",
    subtitle: "Browse files",
    accent: 212,
    supportsDuplicate: true,
    minWindow: {
      w: 980,
      h: 600
    },
    defaultWindow: {
      x: 84,
      y: 64,
      w: 1120,
      h: 700
    },
    component: Finder
  },
  mail: {
    id: "mail",
    name: "Mail",
    icon: "MA",
    runtime: "app",
    title: "Mail",
    subtitle: "Inbox",
    accent: 205,
    minWindow: {
      w: 1180,
      h: 760
    },
    defaultWindow: {
      x: 76,
      y: 42,
      w: 1320,
      h: 820
    },
    component: Mail
  },
  notes: {
    id: "notes",
    name: "Notes",
    icon: "NO",
    runtime: "app",
    title: "Notes",
    subtitle: "Quick writing",
    accent: 34,
    supportsDuplicate: true,
    minWindow: {
      w: 760,
      h: 520
    },
    defaultWindow: {
      x: 160,
      y: 90,
      w: 880,
      h: 620
    },
    component: Notes
  },
  textedit: {
    id: "textedit",
    name: "TextEdit",
    icon: "TE",
    runtime: "app",
    title: "TextEdit",
    subtitle: "Plain text editor",
    accent: 34,
    supportsDuplicate: true,
    minWindow: {
      w: 760,
      h: 520
    },
    defaultWindow: {
      x: 168,
      y: 88,
      w: 900,
      h: 620
    },
    hiddenFromDock: true,
    component: TextEditor
  },
  "installed-web-app": {
    id: "installed-web-app",
    name: "Installed App",
    icon: "AS",
    runtime: "app",
    title: "Installed App",
    subtitle: "App Store",
    accent: 260,
    supportsDuplicate: true,
    hiddenFromDock: true,
    minWindow: {
      w: 860,
      h: 560
    },
    defaultWindow: {
      x: 92,
      y: 54,
      w: 1180,
      h: 780
    },
    component: InstalledWebApp
  },
  music: {
    id: "music",
    name: "Music",
    icon: "MU",
    runtime: "app",
    title: "Music",
    subtitle: "Library",
    accent: 286,
    minWindow: {
      w: 980,
      h: 620
    },
    defaultWindow: {
      x: 112,
      y: 72,
      w: 1180,
      h: 720
    },
    component: Music
  },
  videos: {
    id: "videos",
    name: "Videos",
    icon: "VI",
    runtime: "app",
    title: "Videos",
    subtitle: "Playback",
    accent: 198,
    minWindow: {
      w: 1080,
      h: 720
    },
    defaultWindow: {
      x: 86,
      y: 58,
      w: 1320,
      h: 820
    },
    component: Videos
  },
  peregrine: {
    id: "peregrine",
    name: "Peregrine",
    icon: "PE",
    runtime: "browser",
    title: "Peregrine",
    subtitle: "Web browser",
    accent: 215,
    supportsDuplicate: true,
    minWindow: {
      w: 1040,
      h: 680
    },
    defaultWindow: {
      x: 88,
      y: 54,
      w: 1260,
      h: 780
    },
    component: Peregrine
  },
  photos: {
    id: "photos",
    name: "Photos",
    icon: "PH",
    runtime: "app",
    title: "Photos",
    subtitle: "Gallery",
    accent: 188,
    minWindow: {
      w: 1040,
      h: 680
    },
    defaultWindow: {
      x: 96,
      y: 62,
      w: 1600,
      h: 870
    },
    component: Photos
  },
  "pdf-viewer": {
    id: "pdf-viewer",
    name: "PDF Viewer",
    icon: "PD",
    runtime: "app",
    title: "PDF Viewer",
    subtitle: "Documents",
    accent: 210,
    hiddenFromDock: true,
    minWindow: { w: 980, h: 680 },
    defaultWindow: { x: 92, y: 56, w: 1260, h: 800 },
    component: PdfViewer
  },
  utilities: {
    id: "utilities",
    name: "Utilities",
    icon: "UT",
    runtime: "app",
    title: "Utilities",
    subtitle: "System tools",
    accent: 210,
    minWindow: {
      w: 920,
      h: 640
    },
    defaultWindow: {
      x: 144,
      y: 72,
      w: 1120,
      h: 720
    },
    component: Utilities
  },
  doom: {
    id: "doom",
    name: "DOOM",
    icon: "DO",
    runtime: "app",
    title: "DOOM",
    hiddenFromDock: true,
    minWindow: {
      w: 1180,
      h: 720
    },
    defaultWindow: {
      x: 60,
      y: 42,
      w: 1360,
      h: 820
    },
    component: Doom
  },
  orbit: {
    id: "orbit",
    name: "Orbit",
    icon: "OR",
    runtime: "app",
    title: "Orbit",
    subtitle: "Local reasoning",
    accent: 222,
    supportsDuplicate: true,
    minWindow: {
      w: 800,
      h: 600
    },
    defaultWindow: {
      x: 60,
      y: 38,
      w: 1600,
      h: 850
    },
    component: Orbit
  },
  settings: {
    id: "settings",
    name: "Settings",
    icon: "SE",
    runtime: "app",
    title: "Settings",
    subtitle: "System preferences",
    accent: 220,
    minWindow: {
      w: 640,
      h: 460
    },
    defaultWindow: {
      x: 210,
      y: 108,
      w: 820,
      h: 560
    },
    component: Settings
  },
  print: {
    id: "print",
    name: "Print",
    icon: "PR",
    runtime: "app",
    title: "Print",
    subtitle: "Printers",
    accent: 210,
    hiddenFromDock: true,
    minWindow: { w: 980, h: 620 },
    defaultWindow: { x: 128, y: 82, w: 1160, h: 720 },
    component: Print
  },
  "permissions-manager": {
    id: "permissions-manager",
    name: "Permissions Manager",
    icon: "PM",
    runtime: "app",
    title: "Permissions Manager",
    subtitle: "App access",
    accent: 210,
    hiddenFromDock: true,
    minWindow: { w: 940, h: 620 },
    defaultWindow: { x: 138, y: 88, w: 1140, h: 720 },
    component: PermissionsManager
  },
  firewall: {
    id: "firewall",
    name: "Firewall",
    icon: "FW",
    runtime: "app",
    title: "Firewall",
    subtitle: "Network rules",
    accent: 210,
    hiddenFromDock: true,
    minWindow: { w: 940, h: 620 },
    defaultWindow: { x: 138, y: 88, w: 1140, h: 720 },
    component: Firewall
  },
  "app-store": {
    id: "app-store",
    name: "App Store",
    icon: "AS",
    runtime: "app",
    title: "App Store",
    subtitle: "Install apps",
    accent: 268,
    minWindow: { w: 1040, h: 660 },
    defaultWindow: { x: 108, y: 76, w: 1240, h: 760 },
    component: AppStore
  },
  encryption: {
    id: "encryption",
    name: "Encryption",
    icon: "EN",
    runtime: "app",
    title: "Encryption",
    subtitle: "LUKS security",
    accent: 210,
    hiddenFromDock: true,
    minWindow: { w: 860, h: 540 },
    defaultWindow: { x: 164, y: 106, w: 980, h: 620 },
    component: Encryption
  },
  network: {
    id: "network",
    name: "Network",
    icon: "NW",
    runtime: "app",
    title: "Network",
    subtitle: "Interfaces",
    accent: 205,
    minWindow: {
      w: 900,
      h: 620
    },
    defaultWindow: {
      x: 148,
      y: 84,
      w: 1120,
      h: 720
    },
    component: Network
  },
  wine: {
    id: "wine",
    name: "Wine",
    icon: "WI",
    runtime: "app",
    title: "Wine",
    subtitle: "Windows compatibility",
    accent: 220,
    supportsDuplicate: true,
    minWindow: {
      w: 980,
      h: 640
    },
    defaultWindow: {
      x: 118,
      y: 74,
      w: 1220,
      h: 760
    },
    component: WineLauncherApp
  },
  bench: {
    id: "bench",
    name: "Bench",
    icon: "BE",
    runtime: "app",
    title: "Bench",
    subtitle: "Performance benchmarks",
    accent: 160,
    minWindow: { w: 860, h: 580 },
    defaultWindow: { x: 130, y: 60, w: 1100, h: 740 },
    component: Bench,
  },
  downloads: {
    id: "downloads",
    name: "Downloads",
    icon: "DL",
    runtime: "app",
    title: "Downloads",
    subtitle: "Download manager",
    accent: 200,
    hiddenFromDock: true,
    minWindow: { w: 720, h: 480 },
    defaultWindow: { x: 160, y: 80, w: 840, h: 560 },
    component: Downloads,
  },
  terminal: {
    id: "terminal",
    name: "Terminal",
    icon: "TE",
    runtime: "app",
    title: "Terminal",
    subtitle: "bash shell",
    accent: 120,
    supportsDuplicate: true,
    minWindow: {
      w: 760,
      h: 480
    },
    defaultWindow: {
      x: 120,
      y: 60,
      w: 960,
      h: 600
    },
    component: Terminal
  },
  trash: {
    id: "trash",
    name: "Trash",
    icon: "TR",
    runtime: "app",
    title: "Trash",
    subtitle: "Recently deleted items",
    accent: 0,
    minWindow: {
      w: 800,
      h: 520
    },
    defaultWindow: {
      x: 126,
      y: 78,
      w: 1100,
      h: 680
    },
    component: Trash
  }
};
