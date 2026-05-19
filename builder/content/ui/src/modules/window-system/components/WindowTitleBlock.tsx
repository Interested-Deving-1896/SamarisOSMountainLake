import React from "react";
import { classNames } from "../utils/classNames";

export function WindowTitleBlock(props: {
  title: string;
  subtitle?: string;
  className?: string;
}) {
  return (
    <div className={classNames("samaris-window-title-block", props.className)}>
      <div className="samaris-window-title">{props.title}</div>
      {props.subtitle ? <div className="samaris-window-subtitle">{props.subtitle}</div> : null}
    </div>
  );
}
