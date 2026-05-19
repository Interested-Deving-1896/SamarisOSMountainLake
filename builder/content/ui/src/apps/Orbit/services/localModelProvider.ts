import { kernelClient } from "../../../os/kernel/kernelClient";
import { ORBIT_MODEL_MANIFEST } from "../constants/model";
import type { OrbitModeId, OrbitStrategyId, OrbitModelManifest, ReasoningResult } from "../types";

export async function resolveOrbitModelManifest(): Promise<OrbitModelManifest> {
  try {
    const response = await kernelClient.request<OrbitModelManifest>(
      {
        type: "orbit.status",
        data: {}
      },
      { timeoutMs: 20000 }
    );

    return {
      ...ORBIT_MODEL_MANIFEST,
      ...response.data
    };
  } catch {
    return {
      ...ORBIT_MODEL_MANIFEST,
      runtimeStatus: "unavailable",
      runtimeLabel: "Offline model unavailable"
    };
  }
}

export async function generateLocalReasoning(input: {
  prompt: string;
  modeId: OrbitModeId;
  strategy: OrbitStrategyId;
  onToken?: (chunk: string) => void;
  onReasoning?: (chunk: string) => void;
}): Promise<ReasoningResult> {
  const requestId = `orbit-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;

  const detachDelta = kernelClient.on<{
    requestId?: string;
    content?: string;
  }>("orbit.stream.delta", (payload) => {
    if (payload?.requestId !== requestId) return;
    input.onToken?.(String(payload.content || ""));
  });

  const detachReasoning = kernelClient.on<{
    requestId?: string;
    delta?: string;
  }>("orbit.stream.reasoning", (payload) => {
    if (payload?.requestId !== requestId) return;
    input.onReasoning?.(String(payload.delta || ""));
  });

  const request = kernelClient.request<{ finalAnswer: string }>(
    {
      type: "orbit.generate",
      data: {
        prompt: input.prompt,
        modeId: input.modeId,
        strategy: input.strategy
      },
      requestId
    },
    { timeoutMs: 300000 }
  );

  try {
    const response = await request;
    return {
      reasoningTrace: [],
      finalAnswer: String(response.data?.finalAnswer || "")
    };
  } finally {
    detachDelta();
    detachReasoning();
  }
}
