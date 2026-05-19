import React from "react";
import type { ReasoningStep } from "../types";

export function ReasoningTrace(props: {
  steps: ReasoningStep[];
  visible: boolean;
}) {
  if (!props.visible || props.steps.length === 0) return null;

  return (
    <div className="orbit__trace">
      <div className="orbit__traceTitle">Reasoning Trace</div>
      <div className="orbit__traceList">
        {props.steps.map((step, index) => (
          <div key={`${step.type}-${index}`} className="orbit__traceItem">
            <div className="orbit__traceMeta">
              <span className="orbit__traceType">{step.type}</span>
              {typeof step.confidence === "number" ? (
                <span className="orbit__traceConfidence">{Math.round(step.confidence * 100)}%</span>
              ) : null}
            </div>
            <div className="orbit__traceContent">{step.content}</div>
          </div>
        ))}
      </div>
    </div>
  );
}
