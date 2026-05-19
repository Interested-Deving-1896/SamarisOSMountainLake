import React from "react";
import type { SnapTarget } from "../types";
import { classNames } from "../utils/classNames";

export function SnapOverlay(props: {
  activeTarget: SnapTarget;
  className?: string;
}) {
  return (
    <div className={classNames("samaris-snap-overlay", props.className)} aria-hidden="true">
      <div className={classNames("samaris-snap-zone", "samaris-snap-left", props.activeTarget === "left" ? "active" : "")} />
      <div className={classNames("samaris-snap-zone", "samaris-snap-right", props.activeTarget === "right" ? "active" : "")} />
      <div className={classNames("samaris-snap-zone", "samaris-snap-top", props.activeTarget === "top" ? "active" : "")} />
    </div>
  );
}
