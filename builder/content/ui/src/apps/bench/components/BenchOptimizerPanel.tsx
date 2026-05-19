import React from "react";

export function BenchOptimizerPanel(props: {
  bottlenecks: string[];
  recommendations: any[];
}) {
  return (
    <div className="bench__section">
      <div className="bench__sectionTitle">Optimizer</div>
      {props.bottlenecks.length === 0 && (
        <div className="bench__emptySmall">No bottlenecks detected.</div>
      )}
      {props.bottlenecks.length > 0 && (
        <>
          <div className="bench__subTitle">Bottlenecks</div>
          <div className="bench__bottleneckList">
            {props.bottlenecks.map((b, i) => (
              <div key={i} className="bench__bottleneckItem">{b}</div>
            ))}
          </div>
        </>
      )}
      {props.recommendations.length > 0 && (
        <>
          <div className="bench__subTitle" style={{ marginTop: 12 }}>Recommendations</div>
          <div className="bench__recommendationList">
            {props.recommendations.map((r, i) => (
              <div key={i} className="bench__recommendationItem">
                <div className="bench__recommendationArea">{r.area}</div>
                <div className="bench__recommendationAction">{r.action}</div>
                <div className="bench__recommendationPriority">{r.priority} priority</div>
              </div>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
