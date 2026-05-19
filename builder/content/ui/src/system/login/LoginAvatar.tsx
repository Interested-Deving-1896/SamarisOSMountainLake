import React from "react";

function initialsFromName(name: string) {
  const parts = String(name || "")
    .split(/\s+/)
    .filter(Boolean)
    .slice(0, 2);
  if (!parts.length) return "S";
  return parts.map((part) => part[0]?.toUpperCase() || "").join("");
}

export function LoginAvatar(props: { displayName: string; variant: "login" | "lock" }) {
  const initials = initialsFromName(props.displayName);

  return (
    <div className={`samaris-login__avatarWrap samaris-login__avatarWrap--${props.variant}`}>
      <div className="samaris-login__avatarCore" aria-hidden="true">
        <span className="samaris-login__avatarInitials">{initials}</span>
      </div>
    </div>
  );
}
