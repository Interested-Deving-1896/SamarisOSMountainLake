import React from "react";
import { ORBIT_MODES } from "../constants/modes";
import type { OrbitMessage, OrbitModeId } from "../types";

export function useOrbitStore() {
  const [messages, setMessages] = React.useState<OrbitMessage[]>([]);
  const [modeId, setModeId] = React.useState<OrbitModeId>(ORBIT_MODES[0].id);

  return {
    messages,
    setMessages,
    clearMessages: () => setMessages([]),
    modeId,
    setModeId,
    showReasoning: modeId === "smart",
    setShowReasoning: () => {}
  };
}
