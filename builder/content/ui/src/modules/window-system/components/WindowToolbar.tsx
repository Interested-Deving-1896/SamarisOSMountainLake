import React from "react";
import { CopyPlus, PanelLeft, PanelRight } from "lucide-react";
import { classNames } from "../utils/classNames";

export function WindowToolbar(props: {
  className?: string;
  onSnapLeft?: () => void;
  onSnapRight?: () => void;
  onDuplicate?: () => void;
  showDuplicate?: boolean;
}) {
  return (
    <div className={classNames("samaris-window-toolbar", props.className)}>
      <button
        className="samaris-window-icon-button"
        type="button"
        aria-label="Snap left"
        onPointerDown={(event) => event.stopPropagation()}
        onClick={props.onSnapLeft}
      >
        <PanelLeft size={15} strokeWidth={2.1} aria-hidden="true" />
      </button>
      <button
        className="samaris-window-icon-button"
        type="button"
        aria-label="Snap right"
        onPointerDown={(event) => event.stopPropagation()}
        onClick={props.onSnapRight}
      >
        <PanelRight size={15} strokeWidth={2.1} aria-hidden="true" />
      </button>
      {props.showDuplicate ? (
        <button
          className="samaris-window-icon-button"
          type="button"
          aria-label="Duplicate window"
          onPointerDown={(event) => event.stopPropagation()}
          onClick={props.onDuplicate}
        >
          <CopyPlus size={15} strokeWidth={2.1} aria-hidden="true" />
        </button>
      ) : null}
    </div>
  );
}
