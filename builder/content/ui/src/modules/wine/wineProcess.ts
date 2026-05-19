import type { WineSession, WineSessionLogEvent } from "./WineTypes";

export function upsertSession(current: WineSession[], next: WineSession) {
  const existingIndex = current.findIndex((entry) => entry.sessionId === next.sessionId);
  if (existingIndex < 0) {
    return [next, ...current].sort((left, right) => right.startedAt.localeCompare(left.startedAt));
  }
  const clone = current.slice();
  clone[existingIndex] = next;
  return clone.sort((left, right) => right.startedAt.localeCompare(left.startedAt));
}

export function appendSessionLog(
  logs: Record<string, string[]>,
  event: WineSessionLogEvent
): Record<string, string[]> {
  const nextLogs = [...(logs[event.sessionId] || []), `[${event.stream}] ${event.chunk}`].slice(-400);
  return {
    ...logs,
    [event.sessionId]: nextLogs
  };
}

export function statusLabel(status: WineSession["status"]) {
  switch (status) {
    case "starting":
      return "Starting";
    case "running":
      return "Running";
    case "stopping":
      return "Stopping";
    case "failed":
      return "Failed";
    default:
      return "Exited";
  }
}
