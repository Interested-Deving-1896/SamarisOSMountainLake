export {};

declare global {
  interface Window {
    electronAPI?: {
      window: {
        minimize: () => Promise<void>;
        maximize: () => Promise<void>;
        unmaximize: () => Promise<void>;
        close: () => Promise<void>;
        isMaximized: () => Promise<boolean>;
        onMaximizeChange: (callback: (value: boolean) => void) => () => void;
      };
      clipboard: {
        readText: () => Promise<string>;
        writeText: (text: string) => Promise<void>;
      };
      system: {
        platform: string;
        arch: string;
        versions: Record<string, string>;
        getMemoryInfo: () => Promise<{ total: number; free: number; used: number; usagePercent: string }>;
        getGPUInfo: () => Promise<unknown>;
      };
      browser: {
        createTab: (options?: string | { url?: string; private?: boolean; activate?: boolean }, bounds?: any, opts?: { private?: boolean }) => Promise<{ id: string; url: string; title: string; loading: boolean; canGoBack: boolean; canGoForward: boolean; favicon: string | null; active: boolean; private?: boolean; crashed?: boolean; discarded?: boolean; zoom?: number }>;
        getSnapshot: () => Promise<{ activeTabId: string | null; tabs: Array<{ id: string; url: string; title: string; loading: boolean; canGoBack: boolean; canGoForward: boolean; favicon: string | null; active: boolean; private?: boolean; crashed?: boolean; discarded?: boolean; zoom?: number }> }>;
        navigate: (tabId: string, url: string) => Promise<{ ok: boolean; error?: string }>;
        command: (tabId: string, command: "undo" | "redo" | "back" | "forward" | "reload" | "stop" | "copy" | "cut" | "paste" | "pasteAndMatchStyle" | "delete" | "selectAll" | "find" | "stopFind" | "print" | "savePage" | "openDevTools", payload?: unknown) => Promise<{ ok: boolean; error?: string; path?: string; canceled?: boolean }>;
        goBack: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        goForward: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        reload: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        stop: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        closeTab: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        getTabs: () => Promise<Array<{ id: string; url: string; title: string; loading: boolean; canGoBack: boolean; canGoForward: boolean; favicon: string | null; active: boolean }>>;
        setBounds: (tabId: string, bounds: { x: number; y: number; width: number; height: number }) => Promise<{ ok: boolean }>;
        activateTab: (tabId: string) => Promise<{ id: string; url: string; title: string; loading: boolean; canGoBack: boolean; canGoForward: boolean; favicon: string | null } | null>;
        createPrivateTab: (url?: string, bounds?: { x: number; y: number; width: number; height: number }) => Promise<{ id: string; url: string; title: string; loading: boolean; canGoBack: boolean; canGoForward: boolean; favicon: string | null }>;
        setZoom: (tabId: string, factor: number) => Promise<{ ok: boolean }>;
        showToolbarMenu: (point?: { x: number; y: number }) => Promise<{ ok: boolean }>;
        clearData: (options?: { scope?: "active" | "all"; data?: Array<"cache" | "cookies" | "storage" | "serviceWorkers" | "history"> }) => Promise<{ ok: boolean }>;
        reorderTabs: (tabIds: string[]) => Promise<{ ok: boolean }>;
        savePage: (tabId: string) => Promise<{ ok: boolean; path?: string; error?: string; canceled?: boolean }>;
        printPage: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        openDevTools: (tabId: string) => Promise<{ ok: boolean; error?: string }>;
        destroyAll: () => Promise<{ ok: boolean }>;
        onSnapshot: (callback: (data: { activeTabId: string | null; tabs: Array<{ id: string; url: string; title: string; loading: boolean; canGoBack: boolean; canGoForward: boolean; favicon: string | null; active: boolean; private?: boolean; crashed?: boolean; discarded?: boolean; zoom?: number }> }) => void) => () => void;
        onTabUpdate: (callback: (data: { id: string; title?: string; url?: string; loading?: boolean; canGoBack?: boolean; canGoForward?: boolean; favicon?: string }) => void) => () => void;
        onPermissionRequest: (callback: (data: { tabId: string | null; url: string; permission: string }) => void) => () => void;
      };
      terminal: {
        create: (id: string, options?: { shell?: string; cwd?: string; cols?: number; rows?: number }) => Promise<{ id: string; pid: number; shell: string; cols: number; rows: number }>;
        write: (id: string, data: string) => Promise<void>;
        resize: (id: string, cols: number, rows: number) => Promise<void>;
        kill: (id: string) => Promise<void>;
        onData: (callback: (data: { id: string; data: string }) => void) => () => void;
        onExit: (callback: (data: { id: string; exitCode: number | null; signal: number | null }) => void) => () => void;
      };
      downloads: {
        onStarted: (callback: (data: { id: string; tabId?: string; filename: string; totalBytes: number; url: string }) => void) => () => void;
        onProgress: (callback: (data: { id: string; tabId?: string; filename: string; received: number; total: number; totalBytes?: number }) => void) => () => void;
        onComplete: (callback: (data: { id: string; tabId?: string; filename: string; state: string; savePath?: string; success?: boolean; error?: string }) => void) => () => void;
        cancel: (id: string) => Promise<void>;
      };
      dialog: {
        save: (defaultName?: string) => Promise<{ canceled: boolean; filePath?: string }>;
        open: (options?: { properties?: Array<"openFile" | "openDirectory" | "multiSelections">; filters?: Array<{ name: string; extensions: string[] }> }) => Promise<{ canceled: boolean; filePaths: string[] }>;
      },
      shell: {
        openExternal: (url: string) => Promise<void>;
        openPath: (filePath: string) => Promise<string>;
        showItemInFolder: (filePath: string) => Promise<void>;
      };
      dnd: {
        resolveFiles: (files: File[]) => Promise<Array<{ token: string; name: string; kind: "file" | "dir"; size: number; mime?: string; lastModified?: number }>>;
        importHostFiles: (tokens: string[], destinationPath: string, options?: { conflictStrategy?: "rename" | "replace" | "skip" }) => Promise<{ ok: boolean; imported: string[]; skipped: string[]; failed: Array<{ token: string; error: string }> }>;
        startDragVirtualFiles: (files: Array<{ name: string; path: string; kind: "file" | "dir"; size: number }>) => Promise<{ ok: boolean; error?: string }>;
      };
      app: {
        quit: () => void;
        restart: () => void;
        getVersion: () => Promise<string>;
        isPackaged: () => Promise<boolean>;
      };
      cursor: {
        setCursor: (type: string, theme: "light" | "dark") => Promise<{ ok: boolean }>;
      };
      bench: {
        latest: () => Promise<Record<string, any> | null>;
        history: () => Promise<Record<string, any> | null>;
        baselines: () => Promise<string[]>;
        run: (mode: string) => Promise<Record<string, any> | null>;
        importBaseline: (path: string) => Promise<{ ok: boolean; name: string }>;
        exportJson: () => Promise<{ ok: boolean; path: string }>;
        exportCsv: () => Promise<{ ok: boolean; path: string }>;
        optimizerInput: () => Promise<Record<string, any>>;
        onProgress: (callback: (data: { text: string }) => void) => () => void;
        onComplete: (callback: (data: { code: number; result?: any; error?: string }) => void) => () => void;
      };
    };
  }
}
