import React from "react";
import { DoomSidebar } from "./components/DoomSidebar";
import { DoomViewport } from "./components/DoomViewport";
import { useDoomRuntime } from "./hooks/useDoomRuntime";

export function DoomApp(_props: { windowId: string }) {
  const containerRef = React.useRef<HTMLDivElement>(null);
  const runtime = useDoomRuntime(containerRef);

  return (
    <div className="doom">
      <DoomSidebar />
      <DoomViewport containerRef={containerRef} status={runtime.status} error={runtime.error} />
    </div>
  );
}
