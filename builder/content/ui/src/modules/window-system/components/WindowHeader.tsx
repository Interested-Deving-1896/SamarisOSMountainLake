import React from "react";
import { classNames } from "../utils/classNames";
import { WindowControls } from "./WindowControls";
import { WindowTitleBlock } from "./WindowTitleBlock";
import { WindowToolbar } from "./WindowToolbar";

export function WindowHeader(props: {
  title: string;
  subtitle?: string;
  isMaximized?: boolean;
  className?: string;
  draggable?: boolean;
  toolbar?: React.ReactNode;
  onClose?: () => void;
  onMinimize?: () => void;
  onMaximize?: () => void;
  onSnapLeft?: () => void;
  onSnapRight?: () => void;
  onDuplicate?: () => void;
  showDuplicate?: boolean;
  onPointerDown?: React.PointerEventHandler<HTMLDivElement>;
  onPointerMove?: React.PointerEventHandler<HTMLDivElement>;
  onPointerUp?: React.PointerEventHandler<HTMLDivElement>;
  onPointerCancel?: React.PointerEventHandler<HTMLDivElement>;
  onDoubleClick?: React.MouseEventHandler<HTMLDivElement>;
}) {
  return (
    <div
      className={classNames("samaris-window-header", props.draggable === false ? "not-draggable" : "", props.className)}
      onPointerDown={props.onPointerDown}
      onPointerMove={props.onPointerMove}
      onPointerUp={props.onPointerUp}
      onPointerCancel={props.onPointerCancel}
      onDoubleClick={props.onDoubleClick}
    >
      <WindowControls onClose={props.onClose} onMinimize={props.onMinimize} onMaximize={props.onMaximize} isMaximized={props.isMaximized} />
      <WindowTitleBlock title={props.title} subtitle={props.subtitle} />
      {props.toolbar ?? (
        <WindowToolbar
          onSnapLeft={props.onSnapLeft}
          onSnapRight={props.onSnapRight}
          onDuplicate={props.onDuplicate}
          showDuplicate={props.showDuplicate}
        />
      )}
    </div>
  );
}
