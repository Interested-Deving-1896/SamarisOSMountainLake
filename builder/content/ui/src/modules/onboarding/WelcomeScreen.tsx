import React from "react";
import { ArrowRight } from "lucide-react";

export function WelcomeScreen(props: { onNext: () => void }) {
  return (
    <div className="samaris-onboarding__intro">
      <div className="samaris-onboarding__heroLogo">
        <img src="brand/samaris-logo.png" alt="Samaris OS" className="samaris-onboarding__heroLogoImg" />
      </div>
      <h1 className="samaris-onboarding__heroTitle">
        Samaris OS
      </h1>
      <div className="samaris-onboarding__heroSub">Mountain Lake</div>
      <button
        type="button"
        className="samaris-onboarding__arrow"
        title="Continue"
        aria-label="Continue"
        onClick={props.onNext}
      >
        <ArrowRight size={26} strokeWidth={2.1} />
      </button>
    </div>
  );
}
