import React from "react";

export function SettingsToggle(props: {
  label: string;
  hint?: string;
  checked: boolean;
  onChange: (checked: boolean) => void;
}) {
  return (
    <div className="settings__row">
      <div>
        <div className="settings__rowTitle">{props.label}</div>
        {props.hint ? <div className="settings__rowHint">{props.hint}</div> : null}
      </div>
      <button
        type="button"
        className={`settings__switch ${props.checked ? "settings__switch--on" : ""}`}
        onClick={() => props.onChange(!props.checked)}
        aria-label={props.label}
      >
        <span className="settings__switchKnob" />
      </button>
    </div>
  );
}
