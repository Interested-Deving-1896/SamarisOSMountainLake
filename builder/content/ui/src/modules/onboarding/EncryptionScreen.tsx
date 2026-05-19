import React from "react";
import { CircleAlert, LockKeyhole, ShieldCheck } from "lucide-react";

export function EncryptionScreen(props: {
  status: string;
  message: string;
  error: string;
  limitation?: string;
  busy: boolean;
}) {
  return (
    <>
      <div className="samaris-onboarding__logoDisk is-small">
        <img src="brand/samaris-logo.png" alt="Samaris OS" className="samaris-onboarding__smallLogoImg" />
      </div>
      <h1 className="samaris-onboarding__brandTitle">Samaris OS</h1>
      <div className="samaris-onboarding__progressLine" />
      <div className="samaris-onboarding__orbit" aria-hidden="true">
        <div className="samaris-onboarding__ring" />
        <div className="samaris-onboarding__orbitDot" />
        <div className="samaris-onboarding__orbitDot is-secondary" />
        <div className="samaris-onboarding__bubble">
          <LockKeyhole size={48} strokeWidth={2.4} />
        </div>
      </div>
      <h1 className="samaris-onboarding__title is-tight">Encrypting your data</h1>
      <div className="samaris-onboarding__info">
        <div className="samaris-onboarding__infoRow">
          <div className="samaris-onboarding__infoIcon"><ShieldCheck size={28} strokeWidth={2.2} /></div>
          <div>Your data is encrypted with LUKS. This protects all your files, apps, and settings.</div>
        </div>
        <div className="samaris-onboarding__infoRow">
          <div className="samaris-onboarding__infoIcon"><CircleAlert size={28} strokeWidth={2.2} /></div>
          <div>Please do not unplug your device during this process.</div>
        </div>
      </div>
      <div className="samaris-onboarding__securing">
        {props.busy ? <span className="samaris-onboarding__spinner" aria-hidden="true" /> : null}
        <span>{props.busy ? "Securing…" : props.message}</span>
      </div>
      {props.limitation ? (
        <div className="samaris-onboarding__error">
          <span style={{ display: "inline-flex", alignItems: "center", gap: 8 }}>
            <CircleAlert size={16} strokeWidth={2.2} />
            <span>{props.limitation}</span>
          </span>
        </div>
      ) : null}
      {props.error ? <div className="samaris-onboarding__error">{props.error}</div> : null}
      <div className="samaris-onboarding__meta">{props.status}</div>
    </>
  );
}
