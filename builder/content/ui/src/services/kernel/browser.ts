export type BrowserBounds = {
  left: number;
  top: number;
  width: number;
  height: number;
};

export type BrowserNavigateResult = {
  ok: boolean;
  engine: "chromium";
  bridge: "native-window" | "native-attached";
  installed: boolean;
  executable: string | null;
  version: string | null;
  profileDir: string;
  url: string;
  title: string;
  startPage: boolean;
  attached?: boolean;
  sessionId?: string | null;
  nativeWindowId?: string | null;
  pid?: number | null;
  message?: string;
  attachReason?: string | null;
  state?: string;
};

export type BrowserStatusResult = {
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

export type BrowserSessionSyncResult = {
  ok: boolean;
  found: boolean;
  state?: string;
  message?: string;
};

const BASE_URL = "http://127.0.0.1:9999";

async function parseJson<T>(response: Response) {
  if (!response.ok) {
    throw new Error(`browser_bridge_http_${response.status}`);
  }
  return (await response.json()) as T;
}

async function postJson<T>(url: string, payload: unknown) {
  return await parseJson<T>(
    await fetch(`${BASE_URL}${url}`, {
      method: "POST",
      headers: {
        "content-type": "application/json"
      },
      body: JSON.stringify(payload)
    })
  );
}

export const browserKernel = {
  async navigate(url: string) {
    return await postJson<BrowserNavigateResult>("/api/peregrine/open", { url });
  },
  async openAttachedSession(payload: {
    url: string;
    sessionId?: string | null;
    windowId: string;
    bounds: BrowserBounds;
    focused: boolean;
    minimized: boolean;
  }) {
    return await postJson<BrowserNavigateResult>("/api/peregrine/session/open", payload);
  },
  async syncAttachedSession(payload: {
    sessionId: string;
    bounds: BrowserBounds;
    focused: boolean;
    minimized: boolean;
  }) {
    return await postJson<BrowserSessionSyncResult>("/api/peregrine/session/sync", payload);
  },
  async closeAttachedSession(sessionId: string) {
    return await postJson<BrowserSessionSyncResult>("/api/peregrine/session/close", { sessionId });
  },
  async status() {
    return await parseJson<BrowserStatusResult>(await fetch(`${BASE_URL}/api/peregrine/status`));
  }
};
