import React from "react";
import { Check, Lightbulb } from "lucide-react";
import { ALPHA_ONE_ENCRYPTION_DISABLED_WARNING } from "../../services/kernel/onboarding";

export function FinalScreen(props: {
  fullName: string;
  username: string;
  limitation?: string;
  onEnter: () => void;
  busy: boolean;
}) {
  return (
    <>
      <div className="samaris-onboarding__smallLogo">
        <img src="brand/samaris-logo.png" alt="Samaris OS" className="samaris-onboarding__smallLogoImg" />
      </div>
      <h1 className="samaris-onboarding__brandTitle">Samaris OS</h1>
      <div className="samaris-onboarding__progressLine" />
      <div className="samaris-onboarding__ready is-centered">
        <div className="samaris-onboarding__done is-large">
          <Check size={76} strokeWidth={2.5} />
        </div>
      </div>
      <p className="samaris-onboarding__finalNote">
        Your account has been created for {props.fullName || "you"} as <strong>{props.username || "user"}</strong>,
        and you&apos;re ready to start using Samaris OS.
      </p>
      <div className="samaris-onboarding__tip">
        <div className="samaris-onboarding__tipIcon"><Lightbulb size={34} strokeWidth={2.2} /></div>
        <div>
          {props.limitation
            ? props.limitation
            : ALPHA_ONE_ENCRYPTION_DISABLED_WARNING}
        </div>
      </div>
      <button type="button" className="samaris-onboarding__primary" onClick={props.onEnter} disabled={props.busy}>
        Go to desktop
      </button>
    </>
  );
}
