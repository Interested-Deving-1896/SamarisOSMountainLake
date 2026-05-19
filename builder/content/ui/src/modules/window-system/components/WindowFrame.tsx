import React from "react";
import type { ResizeDirection, SamarisWindowVisualState, WindowGeometry, WindowSidebarItem } from "../types";
import { classNames } from "../utils/classNames";
import { CornerGrip } from "./CornerGrip";
import { ResizeHandles } from "./ResizeHandles";
import { WindowBody } from "./WindowBody";
import { WindowHeader } from "./WindowHeader";
import { WindowSidebar } from "./WindowSidebar";

export type WindowFrameProps = {
  id: string;
  title: string;
  subtitle?: string;
  accent?: number;
  focused?: boolean;
  state?: SamarisWindowVisualState;
  geometry?: WindowGeometry;
  sidebarLabel?: string;
  sidebarItems?: WindowSidebarItem[];
  sidebar?: React.ReactNode;
  children: React.ReactNode;
  className?: string;
  contentClassName?: string;
  style?: React.CSSProperties;
  frameRef?: React.Ref<HTMLElement>;
  onFocus?: () => void;
  onClose?: () => void;
  onMinimize?: () => void;
  onMaximize?: () => void;
  onSnapLeft?: () => void;
  onSnapRight?: () => void;
  onDuplicate?: () => void;
  onResizePointerDown?: (direction: ResizeDirection, event: React.PointerEvent<HTMLDivElement>) => void;
  onHeaderPointerDown?: React.PointerEventHandler<HTMLDivElement>;
  onHeaderPointerMove?: React.PointerEventHandler<HTMLDivElement>;
  onHeaderPointerUp?: React.PointerEventHandler<HTMLDivElement>;
  onHeaderPointerCancel?: React.PointerEventHandler<HTMLDivElement>;
  onHeaderDoubleClick?: React.MouseEventHandler<HTMLDivElement>;
  onSidebarSelect?: (item: WindowSidebarItem) => void;
};

export function WindowFrame(props: WindowFrameProps) {
  const visualState = props.state ?? (props.focused ? "focused" : "inactive");
  const inlineStyle = props.geometry
    ? ({
        left: props.geometry.left,
        top: props.geometry.top,
        width: props.geometry.width,
        height: props.geometry.height
      } as React.CSSProperties)
    : undefined;
  const sidebarNode =
    props.sidebar ??
    (props.sidebarItems?.length ? (
      <WindowSidebar label={props.sidebarLabel} items={props.sidebarItems} onSelect={props.onSidebarSelect} />
    ) : undefined);

  return (
    <section
      id={props.id}
      ref={props.frameRef}
      className={classNames(
        "samaris-window",
        visualState,
        !sidebarNode ? "no-sidebar" : "",
        props.className
      )}
      style={
        props.accent !== undefined
          ? ({ ...inlineStyle, ...props.style, ["--accent" as string]: props.accent } as React.CSSProperties)
          : { ...inlineStyle, ...props.style }
      }
      onPointerDown={props.onFocus}
      role="dialog"
      aria-label={props.title}
    >
      <WindowHeader
        title={props.title}
        subtitle={props.subtitle}
        onClose={props.onClose}
        onMinimize={props.onMinimize}
        onMaximize={props.onMaximize}
        onSnapLeft={props.onSnapLeft}
        onSnapRight={props.onSnapRight}
        onDuplicate={props.onDuplicate}
        showDuplicate={Boolean(props.onDuplicate)}
        isMaximized={visualState === "maximized"}
        onPointerDown={props.onHeaderPointerDown}
        onPointerMove={props.onHeaderPointerMove}
        onPointerUp={props.onHeaderPointerUp}
        onPointerCancel={props.onHeaderPointerCancel}
        onDoubleClick={props.onHeaderDoubleClick}
      />
      <WindowBody sidebar={sidebarNode} contentClassName={props.contentClassName}>
        {props.children}
      </WindowBody>
      <ResizeHandles onPointerDown={props.onResizePointerDown} />
      <CornerGrip />
    </section>
  );
}
