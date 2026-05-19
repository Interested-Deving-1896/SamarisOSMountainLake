import React from "react";
import { Reply, Trash2, Paperclip } from "lucide-react";
import type { MailMessageDetail } from "../types";

export function MailMessageView(props: {
  message: MailMessageDetail | null;
  loading: boolean;
  onReply: () => void;
  onDelete: () => void;
}) {
  if (props.loading && !props.message) {
    return <section className="mail__reader"><div className="mail__emptyPane">Loading message…</div></section>;
  }
  if (!props.message) {
    return <section className="mail__reader"><div className="mail__emptyPane">Select a message to read it.</div></section>;
  }

  const m = props.message;

  return (
    <section className="mail__reader">
      <div className="mail__readerHead">
        <h2 className="mail__readerSubject">{m.subject}</h2>
        <div className="mail__readerActions">
          <button className="mail__readerBtn" onClick={props.onReply} title="Reply"><Reply size={16} /></button>
          <button className="mail__readerBtn" onClick={props.onDelete} title="Delete"><Trash2 size={16} /></button>
        </div>
      </div>
      <div className="mail__readerEnvelope">
        <div className="mail__readerField"><span className="mail__readerLabel">From:</span><span>{m.from}</span></div>
        <div className="mail__readerField"><span className="mail__readerLabel">To:</span><span>{m.to}</span></div>
        {m.cc ? <div className="mail__readerField"><span className="mail__readerLabel">Cc:</span><span>{m.cc}</span></div> : null}
        <div className="mail__readerField"><span className="mail__readerLabel">Date:</span><span>{m.date ? new Date(m.date).toLocaleString() : ""}</span></div>
      </div>
      {m.attachments.length > 0 ? (
        <div className="mail__readerAttachments">
          <Paperclip size={14} /><span>{m.attachments.length} attachment(s)</span>
        </div>
      ) : null}
      <div className="mail__readerBody">
        {m.html ? (
          <iframe className="mail__readerIframe" srcDoc={m.html} sandbox="" title="Email content" />
        ) : (
          <pre className="mail__readerText">{m.text}</pre>
        )}
      </div>
    </section>
  );
}
