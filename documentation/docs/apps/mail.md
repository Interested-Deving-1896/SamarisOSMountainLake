# Mail

Email client supporting **IMAP** via Node.js `imapflow` library and **SMTP** via `nodemailer`, with multi-threaded message loading.

<br>

## Features

- IMAP inbox reading with server-side search
- Compose and send messages via SMTP
- Mail parsing via `mailparser` (HTML, plaintext, attachments)
- Multi-threaded message loading for responsive UI
- Folder navigation (Inbox, Sent, Drafts, Trash, custom folders)
- Message threading by subject/In-Reply-To headers
- Attachment preview and download

<br>

## Architecture

```
MailApp (React)
├── MailSidebar (folders list with unread counts)
├── MailList (message list with sender, subject, date, snippet)
├── MailView (HTML-rendered message viewer with attachment bar)
└── MailCompose (new message editor with To/CC/BCC fields)
```

**Backend:** `mailService.js` using `imapflow` for IMAP operations and `nodemailer` for SMTP delivery. Account credentials are stored in the kernel's secure credential store.

<br>

## Related

- [Credential Store API](../apis/credential-store.md)
- [VOLT Architecture — Network Services](../architecture/volt-networking.md)

<br>

---

[← Back: Documentation Index](../index.md)
