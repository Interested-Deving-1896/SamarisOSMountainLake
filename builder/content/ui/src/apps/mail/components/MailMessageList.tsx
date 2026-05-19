import React from "react";
import { Paperclip, Star } from "lucide-react";
import type { MailMessageSummary } from "../types";

function relativeDate(dateStr: string | null): string {
  if (!dateStr) return "";
  const d = new Date(dateStr);
  const now = new Date();
  const diff = now.getTime() - d.getTime();
  if (diff < 60000) return "now";
  if (diff < 3600000) return `${Math.floor(diff / 60000)}m`;
  if (diff < 86400000) return `${Math.floor(diff / 3600000)}h`;
  if (diff < 604800000) return d.toLocaleDateString(undefined, { weekday: "short" });
  return d.toLocaleDateString(undefined, { month: "short", day: "numeric" });
}

export function MailMessageList(props: {
  messages: MailMessageSummary[];
  selectedUid: number | null;
  busy: boolean;
  onSelect: (uid: number) => void;
  onDoubleClick: (uid: number) => void;
}) {
  return (
    <section className="mail__messageListSection">
      <div className="mail__messageList">
        {props.messages.length === 0 ? (
          <div className="mail__emptyPane">No messages in this folder.</div>
        ) : (
          props.messages.map((m) => (
            <button
              key={m.uid}
              className={`mail__msgRow ${m.uid === props.selectedUid ? "mail__msgRow--selected" : ""} ${!m.seen ? "mail__msgRow--unread" : ""}`}
              onClick={() => props.onSelect(m.uid)}
              onDoubleClick={() => props.onDoubleClick(m.uid)}
            >
              <div className="mail__msgTop">
                <span className="mail__msgFrom">{m.from.split("<")[0].trim() || m.from}</span>
                <span className="mail__msgDate">{relativeDate(m.date)}</span>
              </div>
              <div className="mail__msgSubject">{m.subject}</div>
              <div className="mail__msgMeta">
                {m.flagged ? <Star size={12} fill="currentColor" /> : null}
                {m.hasAttachments ? <Paperclip size={12} /> : null}
              </div>
            </button>
          ))
        )}
      </div>
    </section>
  );
}
