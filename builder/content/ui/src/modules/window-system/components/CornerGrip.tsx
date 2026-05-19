import React from "react";
import { classNames } from "../utils/classNames";

export function CornerGrip(props: { className?: string }) {
  return <div className={classNames("samaris-corner-grip", props.className)} aria-hidden="true" />;
}
