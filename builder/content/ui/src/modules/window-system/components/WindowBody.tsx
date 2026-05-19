import React from "react";
import { classNames } from "../utils/classNames";
import { WindowContent } from "./WindowContent";

export function WindowBody(props: {
  className?: string;
  sidebar?: React.ReactNode;
  contentClassName?: string;
  children: React.ReactNode;
}) {
  return (
    <div className={classNames("samaris-window-body", props.className)}>
      {props.sidebar}
      <WindowContent className={props.contentClassName}>{props.children}</WindowContent>
    </div>
  );
}
