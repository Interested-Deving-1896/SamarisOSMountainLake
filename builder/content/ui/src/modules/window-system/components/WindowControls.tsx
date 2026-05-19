import React from "react";
import { classNames } from "../utils/classNames";

export function WindowControls(props: {
  onClose?: () => void;
  onMinimize?: () => void;
  onMaximize?: () => void;
  isMaximized?: boolean;
  className?: string;
}) {
  return (
    <div className={classNames("samaris-window-controls", props.className)}>
      <button
        className="samaris-window-control close"
        type="button"
        aria-label="Close"
        onPointerDown={(event) => event.stopPropagation()}
        onClick={props.onClose}
      />
      <button
        className="samaris-window-control minimize"
        type="button"
        aria-label="Minimize"
        onPointerDown={(event) => event.stopPropagation()}
        onClick={props.onMinimize}
      />
      <button
        className="samaris-window-control maximize"
        type="button"
        aria-label={props.isMaximized ? "Restore down" : "Maximize"}
        onPointerDown={(event) => event.stopPropagation()}
        onClick={props.onMaximize}
      />
    </div>
  );
}
