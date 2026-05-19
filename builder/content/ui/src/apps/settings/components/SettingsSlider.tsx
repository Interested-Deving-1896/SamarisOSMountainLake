import React from "react";

export function SettingsSlider(props: {
  label: string;
  hint?: string;
  min: number;
  max: number;
  step?: number;
  value: number;
  onChange: (value: number) => void;
  formatValue?: (value: number) => string;
  disabled?: boolean;
}) {
  const pct = ((props.value - props.min) / (props.max - props.min)) * 100;
  const display = props.formatValue ? props.formatValue(props.value) : String(Math.round(props.value));

  return (
    <div className="settings__row">
      <div>
        <div className="settings__rowTitle">{props.label}</div>
        {props.hint ? <div className="settings__rowHint">{props.hint}</div> : null}
      </div>
      <div className="sts-sliderWrap">
        <input
          type="range"
          className="sts-slider"
          min={props.min}
          max={props.max}
          step={props.step ?? 1}
          value={props.value}
          onChange={(e) => props.onChange(Number(e.target.value))}
          disabled={props.disabled}
        />
        <span className="sts-sliderVal">{display}</span>
      </div>
    </div>
  );
}
