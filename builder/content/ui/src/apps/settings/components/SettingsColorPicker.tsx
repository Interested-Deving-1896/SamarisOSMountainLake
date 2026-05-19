import React from "react";

const PRESET_COLORS = [
  { id: "blue", color: "#2563eb", label: "Blue" },
  { id: "purple", color: "#9333ea", label: "Purple" },
  { id: "pink", color: "#db2777", label: "Pink" },
  { id: "red", color: "#dc2626", label: "Red" },
  { id: "orange", color: "#ea580c", label: "Orange" },
  { id: "amber", color: "#d97706", label: "Amber" },
  { id: "green", color: "#059669", label: "Green" },
  { id: "teal", color: "#0d9488", label: "Teal" },
];

export function SettingsColorPicker(props: {
  label: string;
  hint?: string;
  value: string;
  onChange: (color: string) => void;
}) {
  return (
    <div className="settings__row">
      <div>
        <div className="settings__rowTitle">{props.label}</div>
        {props.hint ? <div className="settings__rowHint">{props.hint}</div> : null}
      </div>
      <div className="sts-colorGrid">
        {PRESET_COLORS.map((swatch) => (
          <button
            key={swatch.id}
            type="button"
            className={`sts-colorSwatch ${props.value === swatch.color ? "sts-colorSwatch--active" : ""}`}
            style={{ backgroundColor: swatch.color }}
            onClick={() => props.onChange(swatch.color)}
            title={swatch.label}
          />
        ))}
      </div>
    </div>
  );
}
