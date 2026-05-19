import React from "react";
import type { ResizeDirection } from "../types";

const DIRECTIONS: ResizeDirection[] = ["n", "s", "e", "w", "ne", "nw", "se", "sw"];

export function ResizeHandles(props: {
  className?: string;
  onPointerDown?: (direction: ResizeDirection, event: React.PointerEvent<HTMLDivElement>) => void;
}) {
  return (
    <>
      {DIRECTIONS.map((direction) => (
        <div
          key={direction}
          className={`samaris-resize-handle samaris-resize-${direction}${props.className ? ` ${props.className}` : ""}`}
          role="presentation"
          aria-hidden="true"
          data-direction={direction}
          onPointerDown={(event) => props.onPointerDown?.(direction, event)}
        />
      ))}
    </>
  );
}
