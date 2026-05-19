import { MODE_BY_ID } from "../constants/modes";
import type { OrbitModeId, OrbitStrategyId, ReasoningResult, ReasoningStep } from "../types";

function confidence(seed: number) {
  return Math.max(0.58, Math.min(0.96, seed));
}

function buildChainOfThought(prompt: string, modeId: OrbitModeId): ReasoningStep[] {
  return [
    {
      type: "thought",
      content: `Frame the request inside ${MODE_BY_ID[modeId].label} mode and isolate the core objective.`,
      confidence: confidence(0.76)
    },
    {
      type: "action",
      content: `Break the prompt into constraints, desired outcome, and implementation levers: ${prompt.slice(0, 96)}.`,
      confidence: confidence(0.72)
    },
    {
      type: "observation",
      content: "The best answer should stay practical, modular, and optimized for a local Samaris OS workflow.",
      confidence: confidence(0.82)
    },
    {
      type: "conclusion",
      content: "Synthesize into a direct response with next actions and explicit tradeoffs.",
      confidence: confidence(0.88)
    }
  ];
}

function buildTreeOfThought(prompt: string): ReasoningStep[] {
  return [
    {
      type: "thought",
      content: "Generate multiple strategic approaches instead of committing to the first answer.",
      confidence: confidence(0.71)
    },
    {
      type: "action",
      content: `Compare three branches for "${prompt.slice(0, 80)}": speed-first, quality-first, and scalable-middle-ground.`,
      confidence: confidence(0.7)
    },
    {
      type: "observation",
      content: "The middle-ground usually wins because it preserves momentum without sacrificing architecture.",
      confidence: confidence(0.84)
    },
    {
      type: "conclusion",
      content: "Return the strongest branch and explain why it dominates the alternatives.",
      confidence: confidence(0.9)
    }
  ];
}

function buildSelfConsistency(prompt: string): ReasoningStep[] {
  return [
    {
      type: "thought",
      content: "Sample multiple answer shapes and search for the common core.",
      confidence: confidence(0.73)
    },
    {
      type: "action",
      content: `Cross-check short, medium, and deep responses for "${prompt.slice(0, 88)}".`,
      confidence: confidence(0.68)
    },
    {
      type: "observation",
      content: "The consistent signal is usually the answer that is both concise and operational.",
      confidence: confidence(0.83)
    },
    {
      type: "conclusion",
      content: "Present the consensus answer with the strongest stable recommendations.",
      confidence: confidence(0.89)
    }
  ];
}

function finalAnswerFor(modeId: OrbitModeId, strategy: OrbitStrategyId, prompt: string) {
  const mode = MODE_BY_ID[modeId];
  const promptTrimmed = prompt.trim();
  return [
    `${mode.label} mode is using ${strategy} to answer locally inside Samaris OS.`,
    `The request is interpreted as: ${promptTrimmed || "No prompt supplied."}`,
    "The next best response should stay structured, transparent, and ready for execution without cloud dependency.",
    "Orbit should produce a direct, useful answer with explicit next steps when they add value."
  ].join("\n\n");
}

export function runReasoning(prompt: string, modeId: OrbitModeId, strategy: OrbitStrategyId): ReasoningResult {
  let reasoningTrace: ReasoningStep[];
  if (strategy === "tree-of-thought") {
    reasoningTrace = buildTreeOfThought(prompt);
  } else if (strategy === "self-consistency") {
    reasoningTrace = buildSelfConsistency(prompt);
  } else {
    reasoningTrace = buildChainOfThought(prompt, modeId);
  }

  return {
    finalAnswer: finalAnswerFor(modeId, strategy, prompt),
    reasoningTrace
  };
}
