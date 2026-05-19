import { Terminal } from "lucide-react";

export function TerminalToolbar(props: {
  sessions: Array<{ id: string; title: string; active: boolean }>;
  activeSessionId: string | null;
  onSelectSession: (id: string) => void;
  onCloseSession: (id: string) => void;
  onNewSession: () => void;
  onCopy: () => void;
  onPaste: () => void;
}) {
  return (
    <div className="terminal__toolbar">
      <div className="terminal__brandPill">
        <Terminal size={15} strokeWidth={2.3} />
        <span>Terminal</span>
      </div>
      <div className="terminal__tabStrip">
        {props.sessions.map((s) => (
          <div
            key={s.id}
            className={`terminal__tab ${s.id === props.activeSessionId ? "terminal__tab--active" : ""}`}
            onClick={() => props.onSelectSession(s.id)}
          >
            <span>{s.title}</span>
            <button
              type="button"
              className="terminal__tabClose"
              onClick={(e) => { e.stopPropagation(); props.onCloseSession(s.id); }}
            >
              ✕
            </button>
          </div>
        ))}
        <button type="button" className="terminal__newTab" onClick={props.onNewSession}>+</button>
      </div>
      <div className="terminal__actions">
        <button type="button" className="terminal__actionBtn" onClick={props.onCopy} title="Copy">📋</button>
        <button type="button" className="terminal__actionBtn" onClick={props.onPaste} title="Paste">📄</button>
      </div>
    </div>
  );
}
