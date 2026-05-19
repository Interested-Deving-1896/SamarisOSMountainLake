import React from "react";
import { ArrowLeft } from "lucide-react";
import onboardingWallpaperUrl from "../../assets/wallpapers/onboarding-mountain-lake.png";
import { securityStore } from "../../system/session/securityStore";
import {
  ALPHA_ONE_ENCRYPTION_DISABLED_WARNING,
  onboardingKernel,
  type OnboardingState,
  type OnboardingStep
} from "../../services/kernel/onboarding";
import { userKernel } from "../../services/kernel/user";
import { AccountScreen } from "./AccountScreen";
// @future: import { EncryptionScreen } from "./EncryptionScreen";
import { FinalScreen } from "./FinalScreen";
import { IntroScreen } from "./IntroScreen";
import { LicenseScreen } from "./LicenseScreen";
import {
  ONBOARDING_STEP_ORDER,
  type OnboardingDraft,
  nextStep,
  previousStep
} from "./onboarding.state";
import { clearOnboardingDraft, loadOnboardingDraft, saveOnboardingDraft } from "./onboarding.storage";
import { ONBOARDING_MOTION_MS, onboardingPhaseClass } from "./onboarding.transitions";
import { WelcomeScreen } from "./WelcomeScreen";
import "./onboarding.css";

const EMPTY_STATE: OnboardingState = {
  completed: false,
  currentStep: "welcome",
  licenseAccepted: false,
  accountCreated: false,
  fullName: "",
  username: "",
  setup: {
    finished: false,
    encryptionAvailable: false,
    encrypted: false,
    encryptionEnabled: false,
    encryptionConfigured: false,
    encryptionStatus: "disabled-alpha",
    limitation: ALPHA_ONE_ENCRYPTION_DISABLED_WARNING
  }
};

function alphaDisabledSetup() {
  return {
    finished: true,
    encryptionAvailable: false,
    encrypted: false,
    encryptionEnabled: false,
    encryptionConfigured: false,
    encryptionStatus: "disabled-alpha",
    limitation: ALPHA_ONE_ENCRYPTION_DISABLED_WARNING
  };
}

function validateDraft(draft: OnboardingDraft) {
  if (draft.fullName.trim().length < 2) return "Please enter your full name.";
  if (!/^[a-z0-9][a-z0-9._-]{1,31}$/i.test(draft.username.trim())) {
    return "Username must use letters, numbers, dots, dashes, or underscores.";
  }
  if (draft.password.length < 4) return "Password must contain at least 4 characters.";
  if (draft.password !== draft.confirmPassword) return "Passwords do not match.";
  return "";
}

export function OnboardingShell(props?: { onCompleted?: () => void }) {
  const [loaded, setLoaded] = React.useState(false);
  const [state, setState] = React.useState<OnboardingState>(EMPTY_STATE);
  const [activeStep, setActiveStep] = React.useState<OnboardingStep>("welcome");
  const [draft, setDraft] = React.useState<OnboardingDraft>(() => loadOnboardingDraft());
  const [error, setError] = React.useState("");
  const [entering, setEntering] = React.useState(true);
  const [busy, setBusy] = React.useState(false);
  const [setupStatus, setSetupStatus] = React.useState("idle");
  const [setupMessage, setSetupMessage] = React.useState("Preparing Samaris");
  const [completedTransition, setCompletedTransition] = React.useState(false);
  const [direction, setDirection] = React.useState<"forward" | "back">("forward");

  React.useEffect(() => {
    let cancelled = false;
    const fallbackId = window.setTimeout(() => {
      if (cancelled) return;
      setState(EMPTY_STATE);
      setActiveStep("welcome");
      setLoaded(true);
      window.setTimeout(() => setEntering(false), 30);
    }, 2500);
    void onboardingKernel
      .get()
      .then(async (next) => {
        if (cancelled) return;
        window.clearTimeout(fallbackId);
        // If onboarding was not completed, always restart from welcome
        if (!next.completed) {
          setState({ ...EMPTY_STATE, currentStep: "welcome" });
          setActiveStep("welcome");
          try { await onboardingKernel.patch({ currentStep: "welcome" }); } catch {}
        } else {
          setState(next);
          setActiveStep("final");
        }
        setDraft((current) => ({
          ...current,
          fullName: next.fullName || current.fullName,
          username: next.username || current.username
        }));
      })
      .catch(() => {
        if (cancelled) return;
        window.clearTimeout(fallbackId);
        setState(EMPTY_STATE);
        setActiveStep("welcome");
      })
      .finally(() => {
        if (cancelled) return;
        window.clearTimeout(fallbackId);
        setLoaded(true);
        window.setTimeout(() => setEntering(false), 30);
      });
    return () => {
      cancelled = true;
      window.clearTimeout(fallbackId);
    };
  }, []);

  React.useEffect(() => {
    saveOnboardingDraft(draft);
  }, [draft]);

  React.useEffect(() => {
    if (activeStep !== "encryption") return;
    if (state.setup.finished || state.accountCreated) {
      setSetupStatus("done");
      setSetupMessage("Setup complete");
      void onboardingKernel
        .patch({ currentStep: "final", setup: alphaDisabledSetup() })
        .then((next) => {
          setState(next);
          setActiveStep("final");
        })
        .catch(() => {});
      return;
    }

    let cancelled = false;

    async function run() {
      const validation = validateDraft(draft);
      if (validation) {
        setError(validation);
        setActiveStep("account");
        return;
      }

      setBusy(true);
      setError("");

      try {
        setSetupStatus("preparing");
        setSetupMessage("Preparing Samaris");
        await new Promise((resolve) => window.setTimeout(resolve, 380));
        if (cancelled) return;

        setSetupStatus("creating-account");
        setSetupMessage("Creating local account");
        const created = await onboardingKernel.createAccount({
          fullName: draft.fullName.trim(),
          username: draft.username.trim(),
          password: draft.password
        });
        if (cancelled) return;
        if (!created.ok || !created.state) {
          throw new Error(created.message || "Unable to create the local account.");
        }
        setState(created.state);

        setSetupStatus("securing-storage");
        setSetupMessage("Applying Alpha One setup");
        await new Promise((resolve) => window.setTimeout(resolve, 420));
        if (cancelled) return;

        const setup = await onboardingKernel.evaluateSetup({
          password: draft.password,
          username: draft.username.trim(),
          fullName: draft.fullName.trim()
        });
        if (cancelled) return;
        if (!setup.ok) {
          throw new Error(setup.message || "Unable to prepare Samaris storage.");
        }
        setState(setup.state);

        setSetupStatus("finalizing");
        setSetupMessage("Finalizing environment");
        await new Promise((resolve) => window.setTimeout(resolve, 380));
        if (cancelled) return;

        setSetupStatus("done");
        setSetupMessage("Setup complete");
        const next = await onboardingKernel.patch({ currentStep: "final" });
        if (cancelled) return;
        setState(next);
        setActiveStep("final");
      } catch (nextError) {
        if (cancelled) return;
        setSetupStatus("error");
        setSetupMessage("Setup needs attention");
        setError(nextError instanceof Error ? nextError.message : "Setup could not be completed.");
      } finally {
        if (!cancelled) setBusy(false);
      }
    }

    void run();

    return () => {
      cancelled = true;
    };
  }, [activeStep, draft, state.accountCreated, state.setup.finished]);

  React.useEffect(() => {
    function handleKeyDown(event: KeyboardEvent) {
      if (!loaded || busy || completedTransition) return;
      if (event.key === "Enter" && activeStep === "welcome") {
        event.preventDefault();
        void goNext();
      }
      if (event.key === "Escape" && activeStep !== "welcome" && activeStep !== "encryption") {
        event.preventDefault();
        void goBack();
      }
    }
    window.addEventListener("keydown", handleKeyDown);
    return () => window.removeEventListener("keydown", handleKeyDown);
  }, [activeStep, busy, completedTransition, loaded, state.licenseAccepted]);

  async function goNext() {
    if (busy) return;
    setError("");

    if (activeStep === "license" && !state.licenseAccepted) {
      setError("Please accept the agreement before continuing.");
      return;
    }

    if (activeStep === "account") {
      const validation = validateDraft(draft);
      if (validation) {
        setError(validation);
        return;
      }

      setBusy(true);
      setSetupStatus("creating-account");
      setSetupMessage("Creating local account");
      try {
        const created = await onboardingKernel.createAccount({
          fullName: draft.fullName.trim(),
          username: draft.username.trim(),
          password: draft.password
        });
        if (!created.ok || !created.state) {
          throw new Error(created.message || "Unable to create the local account.");
        }
        const nextState = await onboardingKernel.patch({
          currentStep: "final",
          setup: alphaDisabledSetup()
        });
        setDirection("forward");
        setState(nextState);
        setActiveStep("final");
        setSetupStatus("done");
        setSetupMessage("Setup complete");
      } catch (nextError) {
        setSetupStatus("error");
        setSetupMessage("Setup needs attention");
        setError(nextError instanceof Error ? nextError.message : "Setup could not be completed.");
      } finally {
        setBusy(false);
      }
      return;
    }

    const next = nextStep(activeStep);
    setDirection("forward");
    setActiveStep(next);
    setState((current) => ({ ...current, currentStep: next }));
    await onboardingKernel.patch({ currentStep: next });
  }

  async function goBack() {
    if (busy || activeStep === "welcome") return;
    const next = previousStep(activeStep);
    setError("");
    setDirection("back");
    setActiveStep(next);
    setState((current) => {
      // Reset to a clean state when going back, keeping only what's needed
      return {
        ...EMPTY_STATE,
        currentStep: next,
        licenseAccepted: current.licenseAccepted,
        fullName: current.fullName,
        username: current.username,
      };
    });
    try {
      await onboardingKernel.patch({ currentStep: next, completed: false });
    } catch {}
  }

  async function toggleLicense(nextValue: boolean) {
    setError("");
    const next = await onboardingKernel.patch({
      licenseAccepted: nextValue,
      currentStep: "license"
    });
    setState(next);
  }

  async function enterSamaris() {
    setBusy(true);
    setError("");
    try {
      // Try creating the user — skip if already exists (from a previous attempt)
      try {
        await userKernel.create(draft.username, draft.fullName, draft.password);
      } catch (createErr) {
        const msg = createErr instanceof Error ? createErr.message : "";
        if (msg.includes("user_exists")) {
          // User already exists from a previous attempt — continue
        } else if (msg.includes("permission_denied")) {
          // Permission denied — try login directly
        } else {
          throw createErr;
        }
      }
      // Log in automatically
      await userKernel.login(draft.username, draft.password);
      // Complete onboarding
      await onboardingKernel.complete();
      await securityStore.refresh();
      clearOnboardingDraft();
      setCompletedTransition(true);
      window.setTimeout(() => {
        setState((current) => ({ ...current, completed: true }));
        props?.onCompleted?.();
      }, ONBOARDING_MOTION_MS.exit);
    } catch (nextError) {
      setError(nextError instanceof Error ? nextError.message : "Unable to finish onboarding.");
      setBusy(false);
    }
  }

  if (!loaded) {
    return (
      <div className="samaris-onboarding">
        <div className="samaris-onboarding__wallpaper" style={{ backgroundImage: `url(${onboardingWallpaperUrl})` }} />
        <div className="samaris-onboarding__veil" />
        <div className="samaris-onboarding__mist" />
        <div className="samaris-onboarding__loader">
          <div className="samaris-onboarding__loaderSpinner" />
          <div className="samaris-onboarding__loaderText">Preparing setup…</div>
        </div>
      </div>
    );
  }

  if (state.completed) return null;
  const activeIndex = ONBOARDING_STEP_ORDER.indexOf(activeStep);

  return (
    <div
      className={`samaris-onboarding ${onboardingPhaseClass(entering)} ${completedTransition ? "samaris-onboarding--completed" : ""}`}
    >
      <div className="samaris-onboarding__wallpaper" style={{ backgroundImage: `url(${onboardingWallpaperUrl})` }} />
      <div className="samaris-onboarding__veil" />
      <div className="samaris-onboarding__mist" />

      <div className="samaris-onboarding__shell">
        <section className={`samaris-onboarding__screenFrame is-${activeStep} is-${direction}`} key={activeStep}>
          {activeStep === "welcome" ? (
            <WelcomeScreen onNext={() => void goNext()} />
          ) : (
            <div className={`samaris-onboarding__card ${activeStep === "intro" ? "is-introCard" : ""}`}>
              <button type="button" className="samaris-onboarding__cardBack" onClick={() => void goBack()} aria-label="Go back">
                <ArrowLeft size={18} strokeWidth={2.5} />
              </button>
              <main className="samaris-onboarding__screen">
                <div className="samaris-onboarding__panel">
                  {activeStep === "intro" ? <IntroScreen onNext={() => void goNext()} /> : null}
                  {activeStep === "license" ? (
                    <LicenseScreen
                      accepted={state.licenseAccepted}
                      onToggleAccepted={toggleLicense}
                      onNext={() => void goNext()}
                    />
                  ) : null}
                  {activeStep === "account" ? (
                    <AccountScreen
                      draft={draft}
                      error={error}
                      onChange={(patch) => setDraft((current) => ({ ...current, ...patch }))}
                      onSubmit={() => void goNext()}
                    />
                  ) : null}
                  {/* @future: encryption step
                  {activeStep === "encryption" ? (
                    <EncryptionScreen
                      status={setupStatus}
                      message={setupMessage}
                      error={error}
                      limitation={state.setup.limitation}
                      busy={busy}
                    />
                  ) : null} */}
                  {activeStep === "final" ? (
                    <FinalScreen
                      fullName={state.fullName || draft.fullName}
                      username={state.username || draft.username}
                      limitation={state.setup.limitation}
                      onEnter={() => void enterSamaris()}
                      busy={busy}
                    />
                  ) : null}
                </div>
              </main>
            </div>
          )}
        </section>
      </div>

      <div className="samaris-onboarding__dots" aria-label="Onboarding progress">
        {ONBOARDING_STEP_ORDER.map((step, index) => (
          <span
            key={step}
            className={`samaris-onboarding__dot ${index === activeIndex ? "is-active" : ""}`}
            aria-hidden="true"
          />
        ))}
      </div>
    </div>
  );
}
