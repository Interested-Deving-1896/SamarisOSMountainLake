import React, { useCallback, useEffect, useMemo, useRef, useState } from "react";
import { kernelClient } from "../../os/kernel/kernelClient";
import { Copy, Check, ArrowUp, LoaderCircle } from "lucide-react";

function sanitizeHtml(html: string): string {
  return html
    .replace(/<script[\s\S]*?>[\s\S]*?<\/script>/gi, "")
    .replace(/\bon\w+\s*=\s*(?:"[^"]*"|'[^']*'|[^\s>]+)/gi, " ")
    .replace(/href\s*=\s*"javascript:/gi, 'href="#"')
    .replace(/href\s*=\s*'javascript:/gi, "href='#'");
}

function mdToHtml(md: string): string {
  let h = md
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/^### (.+)$/gm, "<h3>$1</h3>")
    .replace(/^## (.+)$/gm, "<h2>$1</h2>")
    .replace(/^# (.+)$/gm, "<h1>$1</h1>")
    .replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>")
    .replace(/\*(.+?)\*/g, "<em>$1</em>")
    .replace(/```(\w*)\n?([\s\S]*?)```/g, '<pre><code class="language-$1">$2</code></pre>')
    .replace(/`(.+?)`/g, "<code>$1</code>")
    .replace(/^> (.+)$/gm, "<blockquote>$1</blockquote>")
    .replace(/^- (.+)$/gm, "<li>$1</li>")
    .replace(/(<li>.*<\/li>\n?)+/g, "<ul>$&</ul>")
    .replace(/^(\d+)\. (.+)$/gm, "<li>$2</li>")
    .replace(/(<li>.*<\/li>\n?)+/g, (m) => m.includes("<ul>") ? m : `<ol>${m}</ol>`)
    .replace(/\n\n/g, "</p><p>")
    .replace(/\n/g, "<br>");
  return sanitizeHtml(`<p>${h}</p>`);
}

type ChatMessage = {
  id: string;
  role: "user" | "assistant";
  content: string;
  streaming?: boolean;
};

function makeId() {
  return `msg-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function SamarisAvatar({ pulse, side }: { pulse?: boolean; side: "left" | "right" }) {
  return (
    <div className={`orbit-avatar orbit-avatar--${side}${pulse ? " orbit-avatar--pulse" : ""}`}>
      <svg width="22" height="22" viewBox="0 0 100 100" fill="none" xmlns="http://www.w3.org/2000/svg">
        <circle cx="50" cy="50" r="46" stroke="currentColor" strokeWidth="6" fill="none" />
        <path d="M30 55 L50 32 L70 55" stroke="currentColor" strokeWidth="5" strokeLinecap="round" strokeLinejoin="round" fill="none" />
        <path d="M35 60 L50 42 L65 60" stroke="currentColor" strokeWidth="4" strokeLinecap="round" strokeLinejoin="round" fill="none" opacity="0.6" />
        <circle cx="50" cy="50" r="12" stroke="currentColor" strokeWidth="3" fill="none" opacity="0.3" />
      </svg>
    </div>
  );
}

function CopyButton({ text }: { text: string }) {
  const [copied, setCopied] = useState(false);

  return (
    <button
      className="orbit-copy-btn"
      onClick={() => {
        navigator.clipboard.writeText(text).then(() => {
          setCopied(true);
          setTimeout(() => setCopied(false), 1800);
        }).catch(() => {});
      }}
      title="Copy"
    >
      {copied ? <Check size={13} strokeWidth={2.5} /> : <Copy size={13} strokeWidth={2.2} />}
    </button>
  );
}

function TypingDots() {
  return (
    <span className="orbit-typing">
      <span className="orbit-typing__dot" style={{ animationDelay: "0ms" }} />
      <span className="orbit-typing__dot" style={{ animationDelay: "150ms" }} />
      <span className="orbit-typing__dot" style={{ animationDelay: "300ms" }} />
    </span>
  );
}

export default function OrbitApp(_props: { windowId: string }) {
  const [messages, setMessages] = useState<ChatMessage[]>([]);
  const [input, setInput] = useState("");
  const [busy, setBusy] = useState(false);
  const chatRef = useRef<HTMLDivElement | null>(null);

  useEffect(() => {
    const node = chatRef.current;
    if (!node) return;
    node.scrollTo({ top: node.scrollHeight, behavior: "smooth" });
  }, [messages]);

  const sendMessage = useCallback(async () => {
    const trimmed = input.trim();
    if (!trimmed || busy) return;

    const userMsg: ChatMessage = { id: makeId(), role: "user", content: trimmed };
    const assistantMsg: ChatMessage = { id: makeId(), role: "assistant", content: "", streaming: true };

    setMessages((prev) => [...prev, userMsg, assistantMsg]);
    setInput("");
    setBusy(true);

    const requestId = `orbit-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

    const detachDelta = kernelClient.on<{ requestId?: string; content?: string; delta?: string }>("orbit.stream.delta", (payload) => {
      if (payload?.requestId !== requestId) return;
      setMessages((prev) =>
        prev.map((m) =>
          m.id === assistantMsg.id ? { ...m, content: payload.content || "", streaming: true } : m
        )
      );
    });

    try {
      const response = await kernelClient.request<{ finalAnswer: string }>(
        { type: "orbit.generate", data: { prompt: trimmed, modeId: "general", strategy: "self-consistency" }, requestId },
        { timeoutMs: 180000 }
      );

      const finalContent = response.data?.finalAnswer || "";
      setMessages((prev) =>
        prev.map((m) =>
          m.id === assistantMsg.id ? { ...m, content: finalContent, streaming: false } : m
        )
      );
    } catch {
      setMessages((prev) =>
        prev.map((m) =>
          m.id === assistantMsg.id
            ? { ...m, content: "Orbit encountered an error. Please try again.", streaming: false }
            : m
        )
      );
    } finally {
      detachDelta();
      setBusy(false);
    }
  }, [input, busy]);

  const htmlFor = useMemo(() => {
    const map = new Map<string, string>();
    for (const m of messages) {
      if (m.role === "assistant") {
        map.set(m.id, mdToHtml(m.content));
      }
    }
    return map;
  }, [messages]);

  return (
    <div className="orbit-app">
      <style>{`
.orbit-app {
  height: 100%;
  display: grid;
  grid-template-rows: 1fr auto;
  overflow: hidden;
  background: rgba(248,250,252,0.6);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  color: #0f172a;
}

.orbit-app__chat {
  min-height: 0;
  overflow-y: auto;
  padding: 20px 20px 8px;
  display: grid;
  gap: 16px;
  align-content: start;
}

.orbit-app__empty {
  place-self: center;
  text-align: center;
  padding: 48px 20px;
  color: #94a3b8;
  max-width: 400px;
}

.orbit-app__empty svg {
  margin: 0 auto 16px;
  opacity: 0.5;
  color: #94a3b8;
}

.orbit-app__empty h2 {
  font-size: 22px;
  font-weight: 700;
  color: #334155;
  margin: 0 0 8px;
}

.orbit-app__empty p {
  font-size: 14px;
  line-height: 1.6;
  margin: 0;
}

.orbit-message {
  display: grid;
  grid-template-columns: 36px 1fr;
  gap: 12px;
  align-items: start;
  max-width: 780px;
}

.orbit-message--user {
  grid-template-columns: 1fr 36px;
  margin-left: auto;
}

.orbit-message--user .orbit-avatar { order: 2; }
.orbit-message--user .orbit-bubble { order: 1; }

.orbit-avatar {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 12px;
  flex-shrink: 0;
}

.orbit-avatar--left {
  background: linear-gradient(160deg, rgba(29,78,216,0.12), rgba(56,189,248,0.12));
  color: #1d4ed8;
}

.orbit-avatar--right {
  background: rgba(15,23,42,0.06);
  color: #475569;
}

.orbit-avatar--pulse {
  animation: orbit-pulse 2s ease-in-out infinite;
}

@keyframes orbit-pulse {
  0%, 100% { box-shadow: 0 0 0 0 rgba(29,78,216,0.2); }
  50% { box-shadow: 0 0 0 10px rgba(29,78,216,0); }
}

.orbit-bubble {
  padding: 14px 18px;
  border-radius: 20px;
  background: rgba(255,255,255,0.82);
  border: 1px solid rgba(15,23,42,0.07);
  box-shadow: 0 8px 24px rgba(15,23,42,0.04);
  position: relative;
}

.orbit-message--user .orbit-bubble {
  background: linear-gradient(180deg, rgba(37,99,235,0.08), rgba(37,99,235,0.02));
}

.orbit-bubble__header {
  display: flex;
  align-items: center;
  justify-content: space-between;
  margin-bottom: 8px;
  font-size: 11px;
  font-weight: 700;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  color: #64748b;
}

.orbit-bubble__content {
  font-size: 15px;
  line-height: 1.75;
  color: #0f172a;
  user-select: text;
  word-wrap: break-word;
}

.orbit-bubble__content p { margin: 0 0 8px; }
.orbit-bubble__content p:last-child { margin-bottom: 0; }
.orbit-bubble__content h1 { font-size: 1.4em; margin: 12px 0 6px; }
.orbit-bubble__content h2 { font-size: 1.2em; margin: 10px 0 5px; }
.orbit-bubble__content h3 { font-size: 1.1em; margin: 8px 0 4px; }
.orbit-bubble__content strong { font-weight: 700; }
.orbit-bubble__content em { font-style: italic; }
.orbit-bubble__content code {
  padding: 2px 6px;
  border-radius: 6px;
  background: rgba(15,23,42,0.06);
  font-family: "SFMono-Regular", ui-monospace, Menlo, monospace;
  font-size: 0.9em;
}
.orbit-bubble__content pre {
  margin: 8px 0;
  padding: 12px 14px;
  border-radius: 14px;
  background: rgba(15,23,42,0.05);
  border: 1px solid rgba(15,23,42,0.07);
  overflow-x: auto;
  font-size: 13px;
  line-height: 1.6;
}
.orbit-bubble__content pre code {
  background: none;
  padding: 0;
  border-radius: 0;
}
.orbit-bubble__content blockquote {
  margin: 8px 0;
  padding-left: 12px;
  border-left: 3px solid rgba(37,99,235,0.3);
  color: #64748b;
}
.orbit-bubble__content ul, .orbit-bubble__content ol {
  margin: 6px 0;
  padding-left: 20px;
}
.orbit-bubble__content li { margin: 3px 0; }

.orbit-copy-btn {
  display: inline-flex;
  align-items: center;
  justify-content: center;
  width: 26px;
  height: 26px;
  border-radius: 8px;
  border: 0;
  background: rgba(15,23,42,0.04);
  color: #64748b;
  cursor: pointer;
  transition: background 0.15s, color 0.15s;
  flex-shrink: 0;
}
.orbit-copy-btn:hover {
  background: rgba(15,23,42,0.08);
  color: #0f172a;
}

.orbit-typing {
  display: inline-flex;
  align-items: center;
  gap: 4px;
  margin: 4px 0;
}
.orbit-typing__dot {
  width: 6px;
  height: 6px;
  border-radius: 50%;
  background: #94a3b8;
  animation: orbit-typing-bounce 1.2s ease-in-out infinite;
}
@keyframes orbit-typing-bounce {
  0%, 60%, 100% { transform: translateY(0); opacity: 0.5; }
  30% { transform: translateY(-6px); opacity: 1; }
}

.orbit-app__input-area {
  padding: 12px 20px 20px;
  background: linear-gradient(180deg, transparent, rgba(238,242,247,0.7) 40%);
}

.orbit-app__input-inner {
  display: flex;
  align-items: flex-end;
  gap: 10px;
  padding: 10px 14px;
  border-radius: 22px;
  background: rgba(255,255,255,0.88);
  border: 1px solid rgba(15,23,42,0.08);
  box-shadow: 0 8px 28px rgba(15,23,42,0.06);
  max-width: 780px;
  margin: 0 auto;
}

.orbit-app__input-inner textarea {
  flex: 1;
  min-height: 24px;
  max-height: 160px;
  resize: none;
  border: 0;
  background: transparent;
  font: inherit;
  font-size: 15px;
  line-height: 1.6;
  color: #0f172a;
  outline: none;
  user-select: text;
}

.orbit-app__input-inner textarea::placeholder {
  color: #94a3b8;
}

.orbit-app__send-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 36px;
  height: 36px;
  border-radius: 999px;
  border: 0;
  background: linear-gradient(160deg, #1d4ed8, #38bdf8);
  color: white;
  box-shadow: 0 8px 18px rgba(37,99,235,0.25);
  flex-shrink: 0;
  cursor: pointer;
  transition: opacity 0.15s, transform 0.1s;
}

.orbit-app__send-btn:active { transform: scale(0.92); }
.orbit-app__send-btn:disabled { opacity: 0.5; cursor: default; }
.orbit-app__send-btn:disabled:active { transform: none; }

[data-theme="dark"] .orbit-app {
  background: rgba(10,16,28,0.7);
  backdrop-filter: blur(20px);
  -webkit-backdrop-filter: blur(20px);
  color: #e2e8f0;
}

[data-theme="dark"] .orbit-bubble {
  background: rgba(15,23,42,0.75);
  border-color: rgba(148,163,184,0.12);
}

[data-theme="dark"] .orbit-message--user .orbit-bubble {
  background: rgba(37,99,235,0.07);
}

[data-theme="dark"] .orbit-bubble__content {
  color: #e2e8f0;
}

[data-theme="dark"] .orbit-bubble__content code {
  background: rgba(15,23,42,0.8);
}

[data-theme="dark"] .orbit-bubble__content pre {
  background: rgba(15,23,42,0.8);
  border-color: rgba(148,163,184,0.1);
}

[data-theme="dark"] .orbit-app__input-inner {
  background: rgba(15,23,42,0.7);
  border-color: rgba(148,163,184,0.12);
}

[data-theme="dark"] .orbit-app__input-inner textarea {
  color: #f1f5f9;
}

[data-theme="dark"] .orbit-app__empty h2 {
  color: #cbd5e1;
}
`}</style>

      <div ref={chatRef} className="orbit-app__chat">
        {messages.length === 0 ? (
          <div className="orbit-app__empty">
            <SamarisAvatar pulse side="left" />
            <h2>Orbit AI</h2>
            <p>Your private local assistant. Ask anything — Orbit runs entirely on your machine.</p>
          </div>
        ) : (
          messages.map((msg) => (
            <div key={msg.id} className={`orbit-message orbit-message--${msg.role}`}>
              {msg.role === "assistant" ? (
                <SamarisAvatar pulse={msg.streaming} side="left" />
              ) : (
                <SamarisAvatar side="right" />
              )}
              <div className="orbit-bubble">
                <div className="orbit-bubble__header">
                  <span>{msg.role === "user" ? "You" : "Orbit"}</span>
                  {msg.role === "assistant" && !msg.streaming && msg.content ? (
                    <CopyButton text={msg.content} />
                  ) : null}
                </div>
                <div className="orbit-bubble__content">
                  {msg.role === "assistant" && msg.streaming && !msg.content ? (
                    <TypingDots />
                  ) : msg.role === "assistant" ? (
                    <div dangerouslySetInnerHTML={{ __html: htmlFor.get(msg.id) || "" }} />
                  ) : (
                    msg.content
                  )}
                </div>
              </div>
            </div>
          ))
        )}
      </div>

      <div className="orbit-app__input-area">
        <div className="orbit-app__input-inner">
          <textarea
            rows={1}
            value={input}
            placeholder="Message Orbit…"
            onChange={(e) => setInput(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey) {
                e.preventDefault();
                void sendMessage();
              }
            }}
          />
          <button
            className="orbit-app__send-btn"
            disabled={busy || !input.trim()}
            onClick={() => void sendMessage()}
          >
            {busy ? (
              <LoaderCircle size={16} strokeWidth={2.5} className="orbit-spinner" />
            ) : (
              <ArrowUp size={16} strokeWidth={2.5} />
            )}
          </button>
        </div>
      </div>
    </div>
  );
}
