export type PeregrineQuickLink = {
  id: string;
  label: string;
  subtitle?: string;
  url: string;
};

export type PeregrineBridgeStatus = {
  connected: boolean;
  engine: "chromium";
  bridge: "native-window" | "native-attached";
  installed: boolean;
  executable: string | null;
  version: string | null;
  profileDir: string;
  attachedSupported: boolean;
  attachReason: string | null;
  platform: string;
  activeSessions: Array<{
    id: string;
    windowId: string | null;
    pid: number | null;
    url: string;
    state: string;
  }>;
};

export type PeregrineLaunchRecord = {
  id: string;
  title: string;
  url: string;
  openedAt: string;
  pid?: number | null;
};

export type PeregrineSessionState = "idle" | "starting" | "running" | "hidden" | "exited" | "fallback";

export type PeregrineTab = {
  id: string;
  url: string;
  title: string;
  loading: boolean;
  canGoBack: boolean;
  canGoForward: boolean;
  favicon: string | null;
  active: boolean;
  private?: boolean;
  crashed?: boolean;
  discarded?: boolean;
  zoom?: number;
};
