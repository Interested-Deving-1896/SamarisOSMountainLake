import React from "react";
import { ArrowRight, Globe2, LockKeyhole, ShieldCheck, Smartphone } from "lucide-react";

export function IntroScreen(props: { onNext: () => void }) {
  return (
    <div className="samaris-onboarding__introPanel">
      <div className="samaris-onboarding__smallLogo">
        <img src="brand/samaris-logo.png" alt="Samaris OS" className="samaris-onboarding__smallLogoImg" />
      </div>
      <h1 className="samaris-onboarding__brandTitle">Samaris OS</h1>
      <div className="samaris-onboarding__subtitle">Welcome to your new OS.</div>
      <div className="samaris-onboarding__progressLine" />
      <p className="samaris-onboarding__lead">
        This is your first boot on this USB key.
        <br />
        We&apos;ll set up your account in a few steps.
      </p>
      <p className="samaris-onboarding__lead samaris-onboarding__lead--compact">
        No cloud. No account. No telemetry.
        <br />
        Everything stays on this USB drive — encrypted.
      </p>
      <div className="samaris-onboarding__navRow">
        <button type="button" className="samaris-onboarding__arrow" title="Continue" aria-label="Continue" onClick={props.onNext}>
          <ArrowRight size={26} strokeWidth={2.1} />
        </button>
      </div>
      <div className="samaris-onboarding__features">
        <span className="samaris-onboarding__feature"><ShieldCheck size={20} strokeWidth={1.9} /> Security</span>
        <span className="samaris-onboarding__sep" aria-hidden="true" />
        <span className="samaris-onboarding__feature"><LockKeyhole size={20} strokeWidth={1.9} /> Privacy</span>
        <span className="samaris-onboarding__sep" aria-hidden="true" />
        <span className="samaris-onboarding__feature"><Smartphone size={20} strokeWidth={1.9} /> Portable</span>
        <span className="samaris-onboarding__sep" aria-hidden="true" />
        <span className="samaris-onboarding__feature"><Globe2 size={20} strokeWidth={1.9} /> Freedom</span>
      </div>
    </div>
  );
}
