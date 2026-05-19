import { useState, useCallback, useEffect } from "react";
import { TerminalToolbar } from "./TerminalToolbar";
import { TerminalSession } from "./TerminalSession";

interface Session {
  id: string;
  title: string;
}

let sessionCounter = 0;

export function TerminalApp() {
  const [sessions, setSessions] = useState<Session[]>([{ id: `term-${++sessionCounter}`, title: "bash" }]);
  const [activeSessionId, setActiveSessionId] = useState<string>(sessions[0].id);

  const newSession = useCallback(() => {
    const id = `term-${++sessionCounter}`;
    setSessions((prev) => [...prev, { id, title: "bash" }]);
    setActiveSessionId(id);
  }, []);

  const closeSession = useCallback((id: string) => {
    setSessions((prev) => {
      const next = prev.filter((s) => s.id !== id);
      if (next.length === 0) {
        const freshId = `term-${++sessionCounter}`;
        return [{ id: freshId, title: "bash" }];
      }
      return next;
    });
    setActiveSessionId((prev) => {
      if (prev === id) {
        const idx = sessions.findIndex((s) => s.id === id);
        const remaining = sessions.filter((s) => s.id !== id);
        return remaining[Math.min(idx, remaining.length - 1)]?.id || remaining[0]?.id;
      }
      return prev;
    });
  }, [sessions]);

  const selectSession = useCallback((id: string) => {
    setActiveSessionId(id);
  }, []);

  const handleTitleChange = useCallback((id: string, title: string) => {
    setSessions((prev) => prev.map((s) => s.id === id ? { ...s, title } : s));
  }, []);

  const handleCopy = useCallback(async () => {
    if (window.electronAPI) {
      const text = await window.electronAPI.clipboard.readText();
    }
  }, []);

  const handlePaste = useCallback(async () => {
    if (window.electronAPI) {
      const text = await window.electronAPI.clipboard.readText();
      if (text && activeSessionId) {
        window.electronAPI.terminal.write(activeSessionId, text);
      }
    }
  }, [activeSessionId]);

  return (
    <div className="terminal">
      <TerminalToolbar
        sessions={sessions.map((s) => ({ ...s, active: s.id === activeSessionId }))}
        activeSessionId={activeSessionId}
        onSelectSession={selectSession}
        onCloseSession={closeSession}
        onNewSession={newSession}
        onCopy={handleCopy}
        onPaste={handlePaste}
      />
      {sessions.map((s) => (
        <div key={s.id} style={{ display: s.id === activeSessionId ? "flex" : "none", flex: 1, minHeight: 0 }}>
          <TerminalSession
            sessionId={s.id}
            active={s.id === activeSessionId}
            onTitleChange={(title) => handleTitleChange(s.id, title)}
          />
        </div>
      ))}
    </div>
  );
}
