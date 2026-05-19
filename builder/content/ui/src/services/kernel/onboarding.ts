import { kernelClient } from "../../os/kernel/kernelClient";

export type OnboardingStep = "welcome" | "intro" | "license" | "account" | "encryption" | "final";

export const ALPHA_ONE_ENCRYPTION_DISABLED_WARNING =
  "Encryption is temporarily disabled in Alpha One to simplify public testing.";

export type OnboardingState = {
  completed: boolean;
  currentStep: OnboardingStep;
  licenseAccepted: boolean;
  accountCreated: boolean;
  fullName: string;
  username: string;
  setup: {
    finished: boolean;
    encryptionAvailable: boolean;
    encrypted: boolean;
    encryptionEnabled?: boolean;
    encryptionConfigured?: boolean;
    encryptionStatus?: "disabled-alpha" | string;
    limitation: string;
  };
};

export const onboardingKernel = {
  async get() {
    const response = await kernelClient.request<OnboardingState>({ type: "onboarding.get", data: {} });
    if (!response.data) throw new Error("onboarding_get_missing");
    return response.data;
  },
  async patch(payload: Partial<OnboardingState>) {
    const response = await kernelClient.request<OnboardingState>({ type: "onboarding.patch", data: payload });
    if (!response.data) throw new Error("onboarding_patch_missing");
    return response.data;
  },
  async createAccount(payload: { fullName: string; username: string; password: string }) {
    const response = await kernelClient.request<{
      ok: boolean;
      code?: string;
      message?: string;
      state?: OnboardingState;
    }>({ type: "onboarding.createAccount", data: payload });
    if (!response.data) throw new Error("onboarding_create_account_missing");
    return response.data;
  },
  async evaluateSetup(payload: { password: string; username: string; fullName: string }) {
    const response = await kernelClient.request<{
      ok: boolean;
      state: OnboardingState;
      encryption: {
        available: boolean;
        encrypted: boolean;
        note: string;
        status?: string;
        platform?: string;
      };
      message?: string;
      storage?: {
        dryRun?: boolean;
        message?: string;
      };
    }>({ type: "onboarding.evaluateSetup", data: payload });
    if (!response.data) throw new Error("onboarding_setup_missing");
    return response.data;
  },
  async complete() {
    const response = await kernelClient.request<OnboardingState>({ type: "onboarding.complete", data: {} });
    if (!response.data) throw new Error("onboarding_complete_missing");
    return response.data;
  }
};
