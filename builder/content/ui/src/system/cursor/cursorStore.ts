export type CursorType =
  | "default"
  | "pointer"
  | "text"
  | "crosshair"
  | "help"
  | "not-allowed"
  | "move"
  | "ns-resize"
  | "ew-resize"
  | "nwse-resize"
  | "nesw-resize"
  | "grab"
  | "grabbing"
  | "wait"
  | "progress"
  | "writing"
  | "person"
  | "pin"
  | "col-resize"
  | "row-resize";

type CursorListener = () => void;

class CursorStore {
  private type: CursorType = "default";
  private listeners = new Set<CursorListener>();

  getState() {
    return this.type;
  }

  setType(type: CursorType) {
    if (this.type === type) return;
    this.type = type;
    for (const listener of this.listeners) {
      listener();
    }
  }

  subscribe(listener: CursorListener) {
    this.listeners.add(listener);
    return () => {
      this.listeners.delete(listener);
    };
  }
}

export const cursorStore = new CursorStore();

const CUR_FILE: Record<string, string> = {
  default: "arrow.cur",
  pointer: "hand.cur",
  text: "ibeam.cur",
  crosshair: "crosshair.cur",
  help: "help.cur",
  "not-allowed": "no.cur",
  move: "sizeall.cur",
  "ns-resize": "sizens.cur",
  "ew-resize": "sizewe.cur",
  "nwse-resize": "sizenwse.cur",
  "nesw-resize": "sizenesw.cur",
  grab: "hand.cur",
  grabbing: "hand.cur",
  writing: "nwpen.cur",
  person: "person.cur",
  pin: "pin.cur",
  wait: "",
  progress: "",
  "col-resize": "sizewe.cur",
  "row-resize": "sizens.cur",
};

export function resolveCursorStyle(type: CursorType, theme: "light" | "dark"): string {
  const c = CUR_FILE[type];
  if (c) return `url(/cursors/${theme}/${c}), ${type}`;
  return type;
}
