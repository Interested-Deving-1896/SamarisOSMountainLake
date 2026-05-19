import React from "react";
import { Globe, Mail, ShieldCheck } from "lucide-react";
import type { MailProvider } from "../types";

export function MailSetupForm(props: {
  providers: MailProvider[];
  busy: boolean;
  onSave: (payload: Record<string, unknown>) => Promise<void>;
}) {
  const [step, setStep] = React.useState<"account" | "server">("account");
  const [providerId, setProviderId] = React.useState("gmail");
  const provider = props.providers.find((entry) => entry.id === providerId) || props.providers[0];
  const [form, setForm] = React.useState({
    label: "",
    name: "",
    email: "",
    username: "",
    password: "",
    imapHost: provider?.imap.host || "",
    imapPort: String(provider?.imap.port || 993),
    imapSecure: provider?.imap.secure ?? true,
    smtpHost: provider?.smtp.host || "",
    smtpPort: String(provider?.smtp.port || 465),
    smtpSecure: provider?.smtp.secure ?? true
  });

  React.useEffect(() => {
    const p = props.providers.find((e) => e.id === providerId);
    if (!p) return;
    setForm((current) => ({
      ...current,
      imapHost: p.imap.host,
      imapPort: String(p.imap.port),
      imapSecure: p.imap.secure,
      smtpHost: p.smtp.host,
      smtpPort: String(p.smtp.port),
      smtpSecure: p.smtp.secure
    }));
  }, [providerId]); // eslint-disable-line react-hooks/exhaustive-deps

  async function submit() {
    await props.onSave({
      providerId,
      label: form.label || form.email,
      name: form.name || form.email,
      email: form.email,
      username: form.username || form.email,
      password: form.password,
      imap: { host: form.imapHost, port: Number(form.imapPort), secure: form.imapSecure },
      smtp: { host: form.smtpHost, port: Number(form.smtpPort), secure: form.smtpSecure }
    });
  }

  const [testStatus, setTestStatus] = React.useState<string | null>(null);
  async function testConnection() {
    setTestStatus("Testing…");
    try {
      const { mailClient } = await import("../services/mailClient");
      await mailClient.verifyAccount({
        providerId, email: form.email, username: form.username || form.email, password: form.password,
        imap: { host: form.imapHost, port: Number(form.imapPort), secure: form.imapSecure },
        smtp: { host: form.smtpHost, port: Number(form.smtpPort), secure: form.smtpSecure }
      });
      setTestStatus("✅ Connection successful");
    } catch (e) {
      setTestStatus("❌ " + (e instanceof Error ? e.message : "Connection failed"));
    }
  }

  return (
    <div className="mail__setup">
      <div className="mail__setupCard">
        <div className="mail__setupHero">
          <div className="mail__setupBadge">
            <Mail size={16} strokeWidth={2.2} />
            <span>Mail</span>
          </div>
          <h2>Connect your inbox</h2>
          <p>Use a known provider preset or enter a custom IMAP/SMTP configuration.</p>
        </div>

        <div className="mail__providerGrid">
          {props.providers.map((entry) => (
            <button
              key={entry.id}
              type="button"
              className={`mail__providerCard ${providerId === entry.id ? "mail__providerCard--active" : ""}`}
              onClick={() => setProviderId(entry.id)}
            >
              <div className="mail__providerLabel">{entry.label}</div>
              <div className="mail__providerHint">{entry.authHint}</div>
            </button>
          ))}
        </div>

        <div className="mail__setupStepper">
          <button type="button" className={`mail__stepChip ${step === "account" ? "mail__stepChip--active" : ""}`} onClick={() => setStep("account")}>
            Account
          </button>
          <button type="button" className={`mail__stepChip ${step === "server" ? "mail__stepChip--active" : ""}`} onClick={() => setStep("server")}>
            Servers
          </button>
        </div>

        {step === "account" ? (
          <div className="mail__formGrid">
            <label className="mail__field">
              <span>Display name</span>
              <input value={form.name} onChange={(event) => setForm({ ...form, name: event.target.value })} />
            </label>
            <label className="mail__field">
              <span>Account label</span>
              <input value={form.label} onChange={(event) => setForm({ ...form, label: event.target.value })} />
            </label>
            <label className="mail__field">
              <span>Email</span>
              <input type="email" value={form.email} onChange={(event) => setForm({ ...form, email: event.target.value })} />
            </label>
            <label className="mail__field">
              <span>Username</span>
              <input value={form.username} onChange={(event) => setForm({ ...form, username: event.target.value })} />
            </label>
            <label className="mail__field mail__field--full">
              <span>Password / app password</span>
              <input type="password" value={form.password} onChange={(event) => setForm({ ...form, password: event.target.value })} />
            </label>
          </div>
        ) : (
          <div className="mail__serverGrid">
            <div className="mail__serverCard">
              <div className="mail__serverTitle"><Globe size={15} /> IMAP</div>
              <label className="mail__field"><span>Host</span><input value={form.imapHost} onChange={(event) => setForm({ ...form, imapHost: event.target.value })} /></label>
              <div className="mail__fieldRow">
                <label className="mail__field"><span>Port</span><input value={form.imapPort} onChange={(event) => setForm({ ...form, imapPort: event.target.value })} /></label>
                <label className="mail__check"><input type="checkbox" checked={form.imapSecure} onChange={(event) => setForm({ ...form, imapSecure: event.target.checked })} /> Secure TLS</label>
              </div>
            </div>
            <div className="mail__serverCard">
              <div className="mail__serverTitle"><ShieldCheck size={15} /> SMTP</div>
              <label className="mail__field"><span>Host</span><input value={form.smtpHost} onChange={(event) => setForm({ ...form, smtpHost: event.target.value })} /></label>
              <div className="mail__fieldRow">
                <label className="mail__field"><span>Port</span><input value={form.smtpPort} onChange={(event) => setForm({ ...form, smtpPort: event.target.value })} /></label>
                <label className="mail__check"><input type="checkbox" checked={form.smtpSecure} onChange={(event) => setForm({ ...form, smtpSecure: event.target.checked })} /> Secure TLS</label>
              </div>
            </div>
          </div>
        )}

        <div className="mail__setupActions">
          {step === "server" ? (
            <button type="button" className="mail__secondaryBtn" onClick={() => setStep("account")}>Back</button>
          ) : null}
          {step === "account" ? (
            <button type="button" className="mail__secondaryBtn" onClick={() => setStep("server")}>Next</button>
          ) : null}
          <button type="button" className="mail__secondaryBtn" onClick={() => void testConnection()} disabled={props.busy || !form.email || !form.password}>
            Test
          </button>
          <button type="button" className="mail__primaryBtn" onClick={() => void submit()} disabled={props.busy || !form.email || !form.password}>
            {props.busy ? "Connecting…" : "Add account"}
          </button>
        </div>
        {testStatus ? <div style={{ fontSize: 12, marginTop: 8, color: testStatus.includes("✅") ? "var(--accent)" : "#b91c1c" }}>{testStatus}</div> : null}
      </div>
    </div>
  );
}
