import React from "react";
import { encryptionKernel } from "../../services/kernel/encryption";

export function EncryptionApp(_props: { windowId: string }) {
  const [status, setStatus] = React.useState<Awaited<ReturnType<typeof encryptionKernel.status>> | null>(null);
  const [note, setNote] = React.useState("");

  React.useEffect(() => {
    void encryptionKernel.status().then(setStatus);
  }, []);

  return (
    <div className="enc">
      <div className="enc__title">Encryption</div>
      <div className="enc__subtitle">LUKS user partition controls and recovery surfaces.</div>
      <div className="enc__card">
        <div className="enc__row"><span>Status</span><strong>{status?.encrypted ? "Encrypted" : "Not encrypted"}</strong></div>
        <div className="enc__row"><span>Platform</span><strong>{status?.platform || "--"}</strong></div>
        <div className="enc__note">{note || status?.note || ""}</div>
        <div className="enc__actions">
          <button type="button" className="enc__btn" onClick={() => void encryptionKernel.changePassphrase().then((result) => setNote(result.note))}>Change passphrase</button>
          <button type="button" className="enc__btn" onClick={() => void encryptionKernel.backupRecoveryPhrase().then((result) => setNote(result.note))}>Backup recovery phrase</button>
          <button type="button" className="enc__btn" onClick={() => void encryptionKernel.integrityCheck().then((result) => setNote(result.note))}>Integrity check</button>
        </div>
      </div>
    </div>
  );
}
