import React from "react";

export function BenchChart(props: { data: { label: string; value: number }[]; height?: number }) {
  const max = Math.max(...props.data.map((d) => d.value), 100);
  const h = props.height || 120;

  return (
    <div className="bench__chart" style={{ height: h }}>
      {props.data.map((point, i) => {
        const pct = (point.value / max) * 100;
        return (
          <div key={i} className="bench__chartBar" style={{ height: `${pct}%` }} title={`${point.label}: ${point.value}`}>
            <div className="bench__chartTooltip">{point.value}</div>
          </div>
        );
      })}
      <div className="bench__chartLabels">
        {props.data.map((point, i) => (
          <span key={i} className="bench__chartLabel">{point.label}</span>
        ))}
      </div>
    </div>
  );
}
