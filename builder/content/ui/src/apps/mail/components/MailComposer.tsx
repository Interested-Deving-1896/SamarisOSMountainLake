import React from "react";
import { X, Send } from "lucide-react";
import type { MailComposerState } from "../types";

export function MailComposer(props: {
  composer: MailComposerState;
  onChange: (c: MailComposerState) => void;
  onSend: () => void;
  onDiscard: () => void;
  busy: boolean;
}) {
  return (
    <div className="mail__composerOverlay" onClick={props.onDiscard}>
      <div className="mail__composer" onClick={(e) => e.stopPropagation()}>
        <div className="mail__composerHead">
          <span>New Message</span>
          <button className="mail__composerClose" onClick={props.onDiscard}><X size={16} /></button>
        </div>
        <div className="mail__composerBody">
          <input className="mail__composerInput" placeholder="To" value={props.composer.to} onChange={(e) => props.onChange({ ...props.composer, to: e.target.value })} />
          <input className="mail__composerInput" placeholder="Cc" value={props.composer.cc} onChange={(e) => props.onChange({ ...props.composer, cc: e.target.value })} />
          <input className="mail__composerInput" placeholder="Bcc" value={props.composer.bcc} onChange={(e) => props.onChange({ ...props.composer, bcc: e.target.value })} />
          <input className="mail__composerInput" placeholder="Subject" value={props.composer.subject} onChange={(e) => props.onChange({ ...props.composer, subject: e.target.value })} />
          <textarea className="mail__composerTextarea" placeholder="Write your message…" value={props.composer.text} onChange={(e) => props.onChange({ ...props.composer, text: e.target.value })} />
          {props.composer.attachments?.length ? (
            <div className="mail__composerAttachments">
              {props.composer.attachments.map((attachment, index) => (
                <div className="mail__composerAttachment" key={`${attachment.filename}-${index}`}>
                  <span>{attachment.filename}</span>
                  {attachment.size ? <small>{Math.round(attachment.size / 1024)} KB</small> : null}
                  <button
                    type="button"
                    onClick={() => props.onChange({
                      ...props.composer,
                      attachments: props.composer.attachments?.filter((_, i) => i !== index) || []
                    })}
                  >
                    <X size={12} />
                  </button>
                </div>
              ))}
            </div>
          ) : null}
        </div>
        <div className="mail__composerFoot">
          <button className="mail__composerSend" onClick={props.onSend} disabled={props.busy || !props.composer.to.trim()}>
            <Send size={14} /> {props.busy ? "Sending…" : "Send"}
          </button>
        </div>
      </div>
    </div>
  );
}
