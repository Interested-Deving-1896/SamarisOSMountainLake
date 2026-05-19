import React from "react";
import { LoaderCircle, Orbit, UserRound, Volume2 } from "lucide-react";
import { kernelClient } from "../../../os/kernel/kernelClient";
import { MODE_BY_ID } from "../constants/modes";
import { ReasoningTrace } from "./ReasoningTrace";
import type { OrbitMessage } from "../types";

function sanitizeHtml(html: string): string {
  return html
    .replace(/<script[\s\S]*?>[\s\S]*?<\/script>/gi, "")
    .replace(/\bon\w+\s*=\s*(?:"[^"]*"|'[^']*'|[^\s>]+)/gi, " ")
    .replace(/href\s*=\s*"javascript:/gi, 'href="#"')
    .replace(/href\s*=\s*'javascript:/gi, "href='#'");
}

function escapeHtml(text: string): string {
  return text
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/"/g, "&quot;");
}

function mdToHtml(md: string): string {
  let h = escapeHtml(md);

  h = h.replace(/^### (.+)$/gm, "<h3>$1</h3>");
  h = h.replace(/^## (.+)$/gm, "<h2>$1</h2>");
  h = h.replace(/^# (.+)$/gm, "<h1>$1</h1>");

  h = h.replace(/^---$/gm, "<hr />");

  h = h.replace(/^> (.+)$/gm, "<blockquote>$1</blockquote>");

  h = h.replace(/^(\d+)\. (.+)$/gm, "<li>$2</li>");
  h = h.replace(/((<li>.*<\/li>\n?)+)/g, "<ol>$1</ol>");

  h = h.replace(/^[-*] (.+)$/gm, "<li>$1</li>");
  h = h.replace(/((<li>.*<\/li>\n?)+)/g, (match) => {
    if (!match.includes("<ol>")) return `<ul>${match}</ul>`;
    return match;
  });

  h = h.replace(/```(\w*)\n?([\s\S]*?)```/g, (_, lang, code) => {
    const langClass = lang ? ` class="language-${escapeHtml(lang)}"` : "";
    return `<pre><code${langClass}>${code
      .replace(/&amp;/g, "&")
      .replace(/&lt;/g, "<")
      .replace(/&gt;/g, ">")
      .replace(/&quot;/g, '"')}</code></pre>`;
  });

  h = h.replace(/`([^`]+)`/g, "<code>$1</code>");

  h = h.replace(/\[([^\]]+)\]\(([^)]+)\)/g, '<a href="$2" target="_blank" rel="noopener noreferrer">$1</a>');

  h = h.replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>");
  h = h.replace(/(?<!\*)\*(?!\*)(.+?)(?<!\*)\*(?!\*)/g, "<em>$1</em>");

  h = h.replace(/\n\n/g, "</p><p>");
  h = h.replace(/\n/g, "<br />");

  return sanitizeHtml(`<p>${h}</p>`);
}

export function MessageBubble(props: {
  message: OrbitMessage;
  showReasoning: boolean;
}) {
  const mode = props.message.modeId ? MODE_BY_ID[props.message.modeId] : null;
  const [speaking, setSpeaking] = React.useState(false);

  function playAudio(audioBase64: string): Promise<void> {
    return new Promise((resolve, reject) => {
      const el = document.createElement("audio");
      el.src = `data:audio/wav;base64,${audioBase64}`;
      el.onended = () => {
        document.body.removeChild(el);
        resolve();
      };
      el.onerror = (e) => {
        document.body.removeChild(el);
        reject(e);
      };
      document.body.appendChild(el);
      el.play().catch((err) => {
        document.body.removeChild(el);
        reject(err);
      });
    });
  }

  async function speak() {
    const text = props.message.content.trim();
    if (!text || speaking) return;
    setSpeaking(true);
    try {
      const response = await kernelClient.request<{ audioBase64: string; mimeType: string }>(
        { type: "tts.speak", data: { text, voice: "en_US-lessac-high" } },
        { timeoutMs: 120000 }
      );
      const audioBase64 = response.data?.audioBase64;
      if (!audioBase64) return;
      await playAudio(audioBase64);
    } catch (err) {
      console.error("[Orbit] TTS speak failed:", err);
    } finally {
      setSpeaking(false);
    }
  }

  const hasReasoningContent = props.message.role === "assistant" && props.message.reasoningContent?.trim();

  return (
    <article className={`orbit__message orbit__message--${props.message.role}`}>
      <div className={`orbit__avatar orbit__avatar--${props.message.role}`}>
        {props.message.role === "user" ? <UserRound size={16} strokeWidth={2.2} /> : <Orbit size={16} strokeWidth={2.2} />}
      </div>
      <div className={`orbit__bubble orbit__bubble--${props.message.role}`}>
        <div className="orbit__bubbleMeta">
          <span>{props.message.role === "user" ? "You" : "Orbit"}</span>
          {mode ? <span>{mode.label}</span> : null}
          {props.message.role === "assistant" && !props.message.streaming ? (
            <button type="button" className="orbit__speakBtn" title="Read aloud" onClick={() => void speak()} disabled={speaking}>
              {speaking ? <LoaderCircle size={13} className="orbit__spinner" strokeWidth={2.4} /> : <Volume2 size={13} strokeWidth={2.3} />}
            </button>
          ) : null}
        </div>
        {hasReasoningContent && props.showReasoning ? (
          <details className="orbit__reasoningDetails" open>
            <summary className="orbit__reasoningSummary">Show thinking</summary>
            <div className="orbit__reasoningContent">{props.message.reasoningContent}</div>
          </details>
        ) : null}
        {props.message.role === "assistant" && !hasReasoningContent ? (
          <ReasoningTrace steps={props.message.reasoning || []} visible={props.showReasoning} />
        ) : null}
        <div className="orbit__bubbleContent">
          {props.message.role === "assistant" ? (
            <div dangerouslySetInnerHTML={{ __html: mdToHtml(props.message.content || (props.message.streaming ? "Thinking…" : "")) }} />
          ) : (
            props.message.content
          )}
          {props.message.streaming ? <span className="orbit__caret" /> : null}
        </div>
      </div>
    </article>
  );
}
