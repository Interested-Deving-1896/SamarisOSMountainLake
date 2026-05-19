import React from "react";
import { BenchChart } from "./BenchChart";
import type { BenchHistoryEntry } from "../types/bench";

export function BenchHistory(props: { entries: BenchHistoryEntry[] }) {
  const recent = props.entries.slice(-20);
  const chartData = recent.map((e) => ({
    label: new Date(e.timestamp).toLocaleDateString(undefined, { month: "short", day: "numeric" }),
    value: e.score,
  }));

  return (
    <div className="bench__section">
      <div className="bench__sectionTitle">History (last {recent.length} runs)</div>
      <BenchChart data={chartData} height={100} />
      <div className="bench__historyTable">
        {recent.slice(-10).reverse().map((entry) => (
          <div key={entry.run_id} className="bench__historyRow">
            <span className="bench__historyDate">{new Date(entry.timestamp).toLocaleString()}</span>
            <span className="bench__historyMode">{entry.mode}</span>
            <span className="bench__historyScore">{entry.score}</span>
            <span className="bench__historyBadge">{entry.badge}</span>
          </div>
        ))}
      </div>
    </div>
  );
}
