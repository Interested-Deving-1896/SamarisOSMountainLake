export type MailProvider = {
  id: string; label: string; authHint: string;
  imap: { host: string; port: number; secure: boolean };
  smtp: { host: string; port: number; secure: boolean };
};

export type MailAccount = {
  id: string; providerId: string; label: string; name: string;
  email: string; username: string;
  imap: { host: string; port: number; secure: boolean };
  smtp: { host: string; port: number; secure: boolean };
};

export type MailFolder = {
  path: string; name: string; specialUse: string | null;
};

export type MailAttachment = {
  filename: string; contentType: string; size?: number; content?: string;
};

export type MailMessageSummary = {
  uid: number; subject: string; from: string; to: string;
  date: string | null; seen: boolean; flagged: boolean;
  hasAttachments: boolean; snippet?: string;
};

export type MailMessageDetail = {
  uid: number; subject: string; from: string; to: string; cc: string;
  date: string | null; text: string; html: string;
  attachments: MailAttachment[];
};

export type MailComposerState = {
  to: string; cc: string; bcc: string; subject: string; text: string;
  attachments?: MailAttachment[];
};

export type MailThread = {
  uid: number; subject: string; messages: MailMessageSummary[];
};
