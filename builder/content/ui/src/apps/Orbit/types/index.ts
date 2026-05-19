export type OrbitModeId = "fast" | "smart";

export type OrbitStrategyId = "chain-of-thought" | "tree-of-thought" | "self-consistency";

export type ReasoningStep = {
  type: "thought" | "action" | "observation" | "conclusion";
  content: string;
  confidence?: number;
};

export type ReasoningResult = {
  finalAnswer: string;
  reasoningTrace: ReasoningStep[];
};

export type OrbitMode = {
  id: OrbitModeId;
  label: string;
  description: string;
  strategy: OrbitStrategyId;
};

export type OrbitMessage = {
  id: string;
  role: "user" | "assistant";
  content: string;
  modeId?: OrbitModeId;
  reasoning?: ReasoningStep[];
  reasoningContent?: string;
  streaming?: boolean;
  createdAt: string;
};

export type OrbitThread = {
  id: string;
  title: string;
  messages: OrbitMessage[];
  createdAt: string;
  updatedAt: string;
};

export type OrbitModelManifest = {
  name: string;
  sizeLabel: string;
  runtimeStatus: "detected" | "loading" | "ready" | "unavailable";
  runtimeLabel: string;
  provider?: string;
  modelId?: string;
};

export type VoiceState = "idle" | "listening" | "processing" | "speaking";

export type VoiceConfig = {
  vadThreshold?: number;
  silenceTimeoutMs?: number;
  minSpeechDurationMs?: number;
  maxDurationMs?: number;
};
