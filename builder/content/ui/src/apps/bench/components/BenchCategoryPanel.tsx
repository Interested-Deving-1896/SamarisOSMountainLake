import React from "react";
import type { BenchCategoryScores } from "../types/bench";
import { BADGE_COLORS, CATEGORY_LABELS } from "../types/bench";

export function BenchCategoryPanel(props: { categoryScores: BenchCategoryScores }) {
  const entries = Object.entries(props.categoryScores) as [string, number][];

  return (
    <div className="bench__section">
      <div className="bench__sectionTitle">Category Scores</div>
      <div className="bench__categoryGrid">
        {entries.map(([key, score]) => {
          const label = CATEGORY_LABELS[key] || key;
          const color = score >= 90 ? "#10b981" : score >= 80 ? "#3b82f6" : score >= 70 ? "#8b5cf6" : score >= 60 ? "#f97316" : "#ef4444";
          return (
            <div key={key} className="bench__categoryCard">
              <div className="bench__categoryHeader">
                <span className="bench__categoryName">{label}</span>
                <span className="bench__categoryScore" style={{ color }}>{score}</span>
              </div>
              <div className="bench__categoryBar">
                <div className="bench__categoryBarFill" style={{ width: `${score}%`, background: color }} />
              </div>
            </div>
          );
        })}
      </div>
    </div>
  );
}
