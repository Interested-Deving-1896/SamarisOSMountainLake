import React from "react";
import { MailSetupForm } from "./components/MailSetupForm";
import { MailSidebar } from "./components/MailSidebar";
import { MailToolbar } from "./components/MailToolbar";
import { MailMessageList } from "./components/MailMessageList";
import { MailMessageView } from "./components/MailMessageView";
import { MailComposer } from "./components/MailComposer";
import { mailClient } from "./services/mailClient";
import { useMailApp } from "./hooks/useMailApp";
import { useFileDrop } from "../shared/useFileDrop";
import { commitFileDrop } from "../../os/dnd";
import "./mail.css";

export function MailApp() {
  const mail = useMailApp();
  const [droppedFiles, setDroppedFiles] = React.useState<string[]>([]);
  const mailDrop = useFileDrop({
    target: { id: "mail-attachments", label: "Mail composer", path: "/User/Downloads", kind: "app" },
    allowedChoices: ["import", "copy"],
    recommendedAction: "import",
    onDrop: async (files, context) => {
      let attachmentFiles = files;
      if (files.some((file) => file.source === "host")) {
        const result = await commitFileDrop(context.fs, context.plan, { ...context.decision, choice: "copy" });
        attachmentFiles = result.completed.map((path) => ({
          name: path.split("/").pop() || path,
          path,
          kind: "file" as const,
          size: 0,
          source: "samaris" as const
        }));
      }
      const attachments = [];
      for (const file of attachmentFiles) {
        try {
          const result = await context.fs.readDataUrl(file.path);
          const match = result.dataUrl.match(/^data:([^;]+);base64,(.*)$/);
          attachments.push({
            filename: file.name,
            contentType: match?.[1] || file.mime || "application/octet-stream",
            content: match?.[2] || "",
            size: file.size
          });
        } catch {}
      }
      if (attachments.length > 0) {
        mail.setComposer({ ...mail.composer, attachments: [...(mail.composer.attachments || []), ...attachments] });
      }
      setDroppedFiles((prev) => [...prev, ...files.map((f) => f.name)]);
      if (!mail.composerOpen) {
        mail.setComposerOpen(true);
      }
      window.setTimeout(() => setDroppedFiles([]), 3000);
    }
  });

  const mailClass = `mail${mailDrop.isDragging ? " mail--drop-target" : ""}${droppedFiles.length > 0 ? " mail--drop-notify" : ""}`;

  if (mail.loading) {
    return (
      <div className={mailClass} {...mailDrop.dragProps}><div className="mail__skeleton"><div className="mail__skeletonSpinner" /><span>Loading mail…</span></div></div>
    );
  }

  if (mail.accounts.length === 0) {
    return (
      <div className={mailClass} {...mailDrop.dragProps}>
        {mail.error ? <div className="mail__banner">{mail.error}<button className="mail__bannerClose" onClick={() => mail.setError(null)}>✕</button></div> : null}
        <MailSetupForm
          providers={mail.providers}
          busy={mail.busy}
          onSave={(account) => mail.saveAccount(account).catch(() => {})}
        />
      </div>
    );
  }

  return (
    <div className={mailClass} {...mailDrop.dragProps}>
      {mail.error ? <div className="mail__banner">{mail.error}<button className="mail__bannerClose" onClick={() => mail.setError(null)}>✕</button></div> : null}
      <MailToolbar
        selectedAccount={mail.selectedAccount}
        selectedFolder={mail.selectedFolder}
        busy={mail.busy}
        onRefresh={() => mail.fullRefresh()}
        onCompose={() => { mail.setComposerOpen(true); }}
      />
      <div className="mail__shell">
        <MailSidebar
          accounts={mail.accounts}
          selectedAccountId={mail.selectedAccountId}
          onSelectAccount={(id) => { mail.setSelectedAccountId(id); }}
          folders={mail.folders}
          selectedFolder={mail.selectedFolder}
          onSelectFolder={(folder) => { mail.setSelectedFolder(folder); }}
        />
        <MailMessageList
          messages={mail.messages}
          selectedUid={mail.selectedMessageUid}
          busy={mail.busy}
          onSelect={(uid) => { mail.setSelectedMessageUid(uid); }}
          onDoubleClick={(uid) => { mail.setSelectedMessageUid(uid); }}
        />
        <MailMessageView
          message={mail.messageDetail}
          loading={mail.busy}
          onReply={() => {
            if (mail.messageDetail) {
              mail.setComposer({ to: mail.messageDetail.from, cc: "", bcc: "", subject: `Re: ${mail.messageDetail.subject}`, text: "" });
              mail.setComposerOpen(true);
            }
          }}
          onDelete={() => {
            if (mail.selectedAccountId && mail.selectedFolder && mail.selectedMessageUid) {
              mailClient.delete(mail.selectedAccountId, mail.selectedFolder, mail.selectedMessageUid).then(() => mail.setSelectedMessageUid(null)).catch(() => {});
            }
          }}
        />
      </div>
      {mail.composerOpen ? (
        <MailComposer
          composer={mail.composer}
          onChange={mail.setComposer}
          onSend={() => mail.send()}
          onDiscard={() => { mail.setComposerOpen(false); mail.setComposer({ to: "", cc: "", bcc: "", subject: "", text: "" }); }}
          busy={mail.busy}
        />
      ) : null}
    </div>
  );
}
