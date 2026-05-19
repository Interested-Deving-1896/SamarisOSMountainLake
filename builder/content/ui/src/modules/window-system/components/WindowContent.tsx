import React from "react";
import { classNames } from "../utils/classNames";

export function WindowContent(props: {
  children: React.ReactNode;
  className?: string;
}) {
  return <div className={classNames("samaris-window-content", props.className)}>{props.children}</div>;
}
