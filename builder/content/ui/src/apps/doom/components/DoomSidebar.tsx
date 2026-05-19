import { Gamepad2, MousePointerClick, Move } from "lucide-react";

export function DoomSidebar() {
  return (
    <aside className="doom__sidebar">
      <div className="doom__badge">
        <Gamepad2 size={16} strokeWidth={2.3} />
        <span>DOOM</span>
      </div>
      <h2>Classic FPS, right inside Samaris OS.</h2>
      <div className="doom__tips">
        <div className="doom__tip">
          <MousePointerClick size={15} strokeWidth={2.2} />
          <span>Click the game view to capture the mouse.</span>
        </div>
        <div className="doom__tip">
          <Move size={15} strokeWidth={2.2} />
          <span>Use keyboard controls exactly like the original DOS build.</span>
        </div>
      </div>
    </aside>
  );
}
