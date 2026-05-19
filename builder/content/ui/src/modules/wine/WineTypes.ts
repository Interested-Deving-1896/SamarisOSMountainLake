export type WineSessionStatus = "starting" | "running" | "stopping" | "exited" | "failed";

export type WineSession = {
  sessionId: string;
  command: string;
  exePath: string;
  prefixName: string;
  prefixPath: string;
  kind: "exe" | "config";
  status: WineSessionStatus;
  startedAt: string;
  endedAt: string | null;
  exitCode: number | null;
  pid: number | null;
  lastMessage: string | null;
};

export type WineStatus = {
  installed: boolean;
  version: string | null;
  prefixPath: string;
  recentExecutables: string[];
  sessions: WineSession[];
};

export type WineLaunchOptions = {
  prefix?: string;
};

export type WineSessionLogs = {
  sessionId: string;
  logs: string[];
};

export type WineSessionLogEvent = {
  sessionId: string;
  stream: "stdout" | "stderr";
  chunk: string;
};

export type WineSessionUpdateEvent = {
  session: WineSession;
};
