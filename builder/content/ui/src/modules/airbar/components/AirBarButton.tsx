import React from "react";

export const AirBarButton = React.memo(
  React.forwardRef<
    HTMLButtonElement,
    {
      className?: string;
      active?: boolean;
      title?: string;
      ariaLabel?: string;
      ariaExpanded?: boolean;
      onClick?: () => void;
      children: React.ReactNode;
    }
  >(function AirBarButton(props, ref) {
    return (
      <button
        ref={ref}
        type="button"
        className={`air-button ${props.active ? "active" : ""} ${props.className || ""}`.trim()}
        onClick={props.onClick}
        title={props.title}
        aria-label={props.ariaLabel}
        aria-expanded={props.ariaExpanded}
      >
        {props.children}
      </button>
    );
  })
);
