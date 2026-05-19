import type { OrbitMode, OrbitModeId } from "../types";

export const ORBIT_MODES: OrbitMode[] = [
  {
    id: "fast",
    label: "Orbit Fast",
    description: "Quick local answers with minimal deliberation.",
    strategy: "self-consistency"
  },
  {
    id: "smart",
    label: "Orbit Smart",
    description: "Deeper local reasoning for harder prompts.",
    strategy: "chain-of-thought"
  }
];

export const MODE_BY_ID: Record<OrbitModeId, OrbitMode> = Object.fromEntries(
  ORBIT_MODES.map((mode) => [mode.id, mode])
) as Record<OrbitModeId, OrbitMode>;
