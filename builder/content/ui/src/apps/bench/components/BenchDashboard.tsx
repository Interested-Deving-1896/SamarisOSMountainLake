import React from "react";
import { BenchScoreCard } from "./BenchScoreCard";
import { BenchCategoryPanel } from "./BenchCategoryPanel";
import { BenchHistory } from "./BenchHistory";
import { BenchComparison } from "./BenchComparison";
import { BenchRunButton } from "./BenchRunButton";
import { BenchOptimizerPanel } from "./BenchOptimizerPanel";
import { BenchMetricsGrid } from "./BenchMetricsGrid";
import { useBenchResult, useBenchHistory, useBenchRun } from "../hooks";
import "../styles/bench.css";

export function BenchDashboard() {
  const { result, loading, refresh } = useBenchResult();
  const { history } = useBenchHistory();
  const { running, run, devMode } = useBenchRun(refresh);

  return (
    <div className="bench">
      <div className="bench__header">
        <div>
          <div className="bench__title">Bench</div>
          <div className="bench__subtitle">Performance measurement</div>
        </div>
        <div className="bench__headerActions">
          <span className="bench__headerVersion">v1.0.0-alpha</span>
          <BenchRunButton running={running} onRun={run} devMode={devMode} />
        </div>
      </div>

      {loading ? (
        <div className="bench__state">
          <div className="bench__spinner" />
          <span>Loading...</span>
        </div>
      ) : result ? (
        <div className="bench__content">
          <div className="bench__sidebar">
            <BenchScoreCard
              score={result.overall.score}
              maxScore={result.overall.score_out_of}
              normalized={result.overall.normalized_score}
              badge={result.overall.badge}
              confidence={result.run.confidence}
              timestamp={result.timestamp}
              reliabilityFlags={result.run.reliability_flags}
            />
            <BenchOptimizerPanel
              bottlenecks={result.optimizer.bottlenecks}
              recommendations={result.optimizer.recommendations}
            />
          </div>
          <div className="bench__main">
            <BenchCategoryPanel categoryScores={result.category_scores} />
            <BenchMetricsGrid
              hardware={result.hardware}
              environment={result.environment}
              os={result.os}
            />
            {history && history.entries.length > 0 && (
              <BenchHistory entries={history.entries} />
            )}
            <BenchComparison
              baselines={result.comparison.baselines}
              validity={result.comparison.comparison_validity}
            />
          </div>
        </div>
      ) : (
        <div className="bench__state">
          <div className="bench__stateIcon">B</div>
          <div className="bench__stateTitle">Ready to measure</div>
          <div className="bench__stateText">
            {devMode
              ? "Click Copy command or run bench --quick from your terminal to benchmark Samaris OS."
              : "Run your first benchmark to see your Samaris OS performance score."}
          </div>
          <div className="bench__stateHint">
            bench --quick
          </div>
        </div>
      )}
    </div>
  );
}
