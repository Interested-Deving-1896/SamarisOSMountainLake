import React from "react";
import { classNames } from "../utils/classNames";

export function Surface(props: {
  variant?: "hero" | "card" | "metric" | "file-card" | "setting";
  className?: string;
  children: React.ReactNode;
}) {
  const variantClass =
    props.variant === "hero"
      ? "samaris-hero"
      : props.variant === "metric"
        ? "samaris-metric"
        : props.variant === "file-card"
          ? "samaris-file-card"
          : props.variant === "setting"
            ? "samaris-setting"
            : "samaris-card";

  return <div className={classNames(variantClass, props.className)}>{props.children}</div>;
}
