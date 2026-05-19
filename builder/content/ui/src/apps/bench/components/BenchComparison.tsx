import React from "react";

export function BenchComparison(props: { baselines: any[]; validity: string }) {
  return (
    <div className="bench__section">
      <div className="bench__sectionTitle">Comparison</div>
      {props.validity === "none" && (
        <div className="bench__emptySmall">No baselines imported. Use <code>bench --import-baseline</code> to add one.</div>
      )}
      {props.validity === "reference_only" && (
        <div className="bench__warning">
          Comparison is <strong>reference only</strong> — baselines were not measured on this hardware.
        </div>
      )}
      {props.validity === "same_hardware" && (
        <div className="bench__success">Comparison measured on the <strong>same hardware</strong>.</div>
      )}
      {props.baselines.length > 0 && (
        <div className="bench__comparisonList">
          {props.baselines.map((b, i) => (
            <div key={i} className="bench__comparisonRow">
              <span>{b.import_label || `Baseline ${i + 1}`}</span>
              <span>{b.overall?.score ?? "?"} / 10000</span>
            </div>
          ))}
        </div>
      )}
    </div>
  );
}
