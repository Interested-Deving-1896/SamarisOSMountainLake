import React from "react";
import { RefreshCw, FileEdit } from "lucide-react";
import type { MailAccount } from "../types";

export function MailToolbar(props: {
  selectedAccount: MailAccount | null;
  selectedFolder: string;
  busy: boolean;
  onRefresh: () => void;
  onCompose: () => void;
}) {
  return (
    <div className="mail__toolbar">
      <div className="mail__toolbarLeft">
        <span className="mail__toolbarTitle">{props.selectedAccount?.label || "Mail"}</span>
        <span className="mail__toolbarFolder">{props.selectedFolder}</span>
      </div>
      <div className="mail__toolbarRight">
        <button className="mail__toolBtn" onClick={props.onCompose} title="Compose (c)"><FileEdit size={16} /></button>
        <button className="mail__toolBtn" onClick={props.onRefresh} disabled={props.busy} title="Refresh (u)">
          <RefreshCw size={16} className={props.busy ? "mail__spin" : ""} />
        </button>
      </div>
    </div>
  );
}
