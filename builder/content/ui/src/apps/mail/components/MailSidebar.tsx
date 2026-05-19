import React from "react";
import { ChevronRight, Inbox, Send, FileText, AlertTriangle, Trash2, Archive } from "lucide-react";
import type { MailAccount, MailFolder } from "../types";

const FOLDER_ICONS: Record<string, React.ReactNode> = {
  inbox: <Inbox size={16} strokeWidth={2} />,
  sent: <Send size={16} strokeWidth={2} />,
  drafts: <FileText size={16} strokeWidth={2} />,
  spam: <AlertTriangle size={16} strokeWidth={2} />,
  junk: <AlertTriangle size={16} strokeWidth={2} />,
  trash: <Trash2 size={16} strokeWidth={2} />,
  archive: <Archive size={16} strokeWidth={2} />,
};

function folderIcon(f: MailFolder): React.ReactNode {
  if (!f.specialUse) return <ChevronRight size={16} strokeWidth={2} />;
  const key = f.specialUse.toLowerCase().replace(/\\/g, "");
  return FOLDER_ICONS[key] || <ChevronRight size={16} strokeWidth={2} />;
}

export function MailSidebar(props: {
  accounts: MailAccount[];
  selectedAccountId: string | null;
  folders: MailFolder[];
  selectedFolder: string;
  onSelectAccount: (id: string) => void;
  onSelectFolder: (folder: string) => void;
}) {
  return (
    <aside className="mail__sidebar">
      <div className="mail__sidebarSection">
        <div className="mail__sideTitle">Accounts</div>
        {props.accounts.map((a) => (
          <button key={a.id} className={`mail__sideBtn ${props.selectedAccountId === a.id ? "mail__sideBtn--active" : ""}`} onClick={() => props.onSelectAccount(a.id)}>
            <span className="mail__sideDot" style={{ background: a.providerId === "gmail" ? "#ea4335" : a.providerId === "outlook" ? "#0078d4" : a.providerId === "icloud" ? "#555" : "#666" }} />
            <span className="mail__sideLabel">{a.label}</span>
          </button>
        ))}
      </div>
      <div className="mail__sidebarSection">
        <div className="mail__sideTitle">Folders</div>
        <div className="mail__folderList">
          {props.folders.map((f) => (
            <button key={f.path} className={`mail__folderBtn ${props.selectedFolder === f.path ? "mail__folderBtn--active" : ""}`} onClick={() => props.onSelectFolder(f.path)}>
              {folderIcon(f)}
              <span className="mail__folderName">{f.name}</span>
            </button>
          ))}
        </div>
      </div>
    </aside>
  );
}
