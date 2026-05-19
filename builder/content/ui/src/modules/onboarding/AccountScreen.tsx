import React from "react";
import { ArrowRight } from "lucide-react";
import type { OnboardingDraft } from "./onboarding.state";

export function AccountScreen(props: {
  draft: OnboardingDraft;
  error: string;
  onChange: (patch: Partial<OnboardingDraft>) => void;
  onSubmit: () => void;
}) {
  return (
    <>
      <div className="samaris-onboarding__smallLogo">
        <img src="brand/samaris-logo.png" alt="Samaris OS" className="samaris-onboarding__smallLogoImg" />
      </div>
      <h1 className="samaris-onboarding__brandTitle">Samaris OS</h1>
      <div className="samaris-onboarding__progressLine" />
      <h1 className="samaris-onboarding__title is-tight">Create your account</h1>
      <form
        className="samaris-onboarding__form"
        autoComplete="off"
        onSubmit={(event) => {
          event.preventDefault();
          props.onSubmit();
        }}
      >
        <div className="samaris-onboarding__field">
          <label htmlFor="samaris-full-name">Full name</label>
          <input
            id="samaris-full-name"
            name="ownerDisplayName"
            value={props.draft.fullName}
            onChange={(event) => props.onChange({ fullName: event.target.value })}
            autoComplete="off"
            placeholder="Full name"
          />
        </div>
        <div className="samaris-onboarding__field">
          <label htmlFor="samaris-username">Username</label>
          <input
            id="samaris-username"
            name="localHandle"
            value={props.draft.username}
            onChange={(event) => props.onChange({ username: event.target.value })}
            autoComplete="off"
            autoCorrect="off"
            autoCapitalize="none"
            spellCheck={false}
            placeholder="Username"
          />
        </div>
        <div className="samaris-onboarding__field">
          <label htmlFor="samaris-password">Password</label>
          <input
            id="samaris-password"
            type="password"
            name="devicePassphrase"
            value={props.draft.password}
            onChange={(event) => props.onChange({ password: event.target.value })}
            autoComplete="new-password"
            autoCorrect="off"
            autoCapitalize="none"
            spellCheck={false}
            placeholder="Password"
          />
        </div>
        <div className="samaris-onboarding__field">
          <label htmlFor="samaris-password-confirm">Confirm password</label>
          <input
            id="samaris-password-confirm"
            type="password"
            name="devicePassphraseConfirm"
            value={props.draft.confirmPassword}
            onChange={(event) => props.onChange({ confirmPassword: event.target.value })}
            autoComplete="new-password"
            autoCorrect="off"
            autoCapitalize="none"
            spellCheck={false}
            placeholder="Confirm password"
          />
        </div>
        <div className="samaris-onboarding__navRow">
          <button
            type="submit"
            className="samaris-onboarding__arrow samaris-onboarding__arrow--form"
            title="Continue"
            aria-label="Continue"
          >
            <ArrowRight size={26} strokeWidth={2.1} />
          </button>
        </div>
      </form>
      {props.error ? <div className="samaris-onboarding__error">{props.error}</div> : null}
    </>
  );
}
