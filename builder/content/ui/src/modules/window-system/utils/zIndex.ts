import { SAMARIS_Z_INDEX } from "../constants";

export function nextZIndex(currentHighest: number = SAMARIS_Z_INDEX.baseWindow) {
  return Math.max(SAMARIS_Z_INDEX.baseWindow, currentHighest + 1);
}

export function createZIndexManager(start: number = SAMARIS_Z_INDEX.baseWindow) {
  let current: number = start;

  return {
    peek() {
      return current;
    },
    next() {
      current = nextZIndex(current);
      return current;
    },
    reset(value: number = SAMARIS_Z_INDEX.baseWindow) {
      current = value;
    }
  };
}
