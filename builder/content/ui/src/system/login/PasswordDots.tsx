import React from "react";

export function PasswordDots(props: { count: number; focused: boolean }) {
  const dotCount = Math.max(0, Math.min(12, props.count));

  return (
    <div className={`samaris-login__passwordDisplay ${props.focused ? "is-focused" : ""}`} aria-hidden="true">
      {Array.from({ length: dotCount }).map((_, index) => (
        <span
          key={index}
          className="samaris-login__passwordDot"
          style={{ animationDelay: `${Math.min(index * 24, 180)}ms` }}
        />
      ))}
    </div>
  );
}
