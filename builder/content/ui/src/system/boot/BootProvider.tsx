import React, { createContext, useContext, useEffect, useMemo, useState } from "react";
import { bootSync } from "./BootSync";

type BootContextValue = {
  phase: "booting" | "ready";
};

const BootContext = createContext<BootContextValue>({ phase: "booting" });

export function BootProvider(props: { children: React.ReactNode }) {
  const [phase, setPhase] = useState<"booting" | "ready">(() => (bootSync.isReady() ? "ready" : "booting"));

  useEffect(() => {
    if (bootSync.isReady()) {
      setPhase("ready");
      return;
    }

    const onReady = () => {
      setPhase("ready");
    };

    window.addEventListener("volt:boot-ready", onReady);
    return () => {
      window.removeEventListener("volt:boot-ready", onReady);
    };
  }, []);

  const value = useMemo(() => ({ phase }), [phase]);
  return <BootContext.Provider value={value}>{props.children}</BootContext.Provider>;
}

export function useBootPhase() {
  return useContext(BootContext);
}
