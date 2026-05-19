import React from "react";

type DropdownOption<T extends string = string> = {
  label: string;
  value: T;
};

export function SettingsDropdown<T extends string = string>(props: {
  label: string;
  hint?: string;
  value: T;
  options: DropdownOption<T>[];
  onChange: (value: T) => void;
}) {
  return (
    <div className="settings__row">
      <div>
        <div className="settings__rowTitle">{props.label}</div>
        {props.hint ? <div className="settings__rowHint">{props.hint}</div> : null}
      </div>
      <select
        className="sts-dropdown"
        value={props.value}
        onChange={(e) => props.onChange(e.target.value as T)}
      >
        {props.options.map((opt) => (
          <option key={opt.value} value={opt.value}>{opt.label}</option>
        ))}
      </select>
    </div>
  );
}
