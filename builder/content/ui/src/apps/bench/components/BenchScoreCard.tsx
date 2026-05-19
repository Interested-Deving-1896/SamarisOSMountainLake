import React from "react";
import { BADGE_COLORS } from "../types/bench";

export function BenchScoreCard(props: {
  score: number;
  maxScore: number;
  normalized: number;
  badge: string;
  confidence: string;
  timestamp: string;
  reliabilityFlags: string[];
}) {
  const pct = (props.score / props.maxScore) * 100;
  const color = BADGE_COLORS[props.badge] || "#6b7280";

  return (
    <div className="bench__scoreCard" style={{ borderTopColor: color }}>
      <div className="bench__scoreValue" style={{ color }}>{props.score}</div>
      <div className="bench__scoreDenom">/ {props.maxScore}</div>
      <div className="bench__scoreBadge" style={{ background: `${color}18`, color }}>{props.badge}</div>
      <div className="bench__scoreNormalized">{props.normalized.toFixed(1)} / 100 normalized</div>
      <div className="bench__scoreMeta">
        <span>Confidence: <strong>{props.confidence}</strong></span>
        <span>Run: {new Date(props.timestamp).toLocaleString()}</span>
      </div>
      {props.reliabilityFlags.length > 0 && (
        <div className="bench__scoreFlags">
          {props.reliabilityFlags.map((f) => (
            <span key={f} className="bench__flag bench__flag--warn">{f}</span>
          ))}
        </div>
      )}
    </div>
  );
}
