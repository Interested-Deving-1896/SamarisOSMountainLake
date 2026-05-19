import React from "react";
import type { BenchmarkMode } from "../types/bench";

const MODE_INFO: Record<BenchmarkMode, { label: string; desc: string }> = {
  quick: { label: "Quick", desc: "10–30 sec basic check" },
  full: { label: "Full", desc: "Complete benchmark, 5 iterations" },
  stress: { label: "Stress", desc: "Thermal & memory pressure test" },
  ci: { label: "CI", desc: "Build validation, strict exit code" },
};

export function BenchRunButton(props: { running: boolean; onRun: (mode: BenchmarkMode) => void; devMode?: boolean }) {
  const [menuOpen, setMenuOpen] = React.useState(false);
  const [copied, setCopied] = React.useState(false);

  const handleRun = (mode: BenchmarkMode) => {
    setMenuOpen(false);
    if (props.devMode) {
      setCopied(true);
      setTimeout(() => setCopied(false), 2000);
    }
    props.onRun(mode);
  };

  return (
    <div className="bench__runWrap">
      <div className="bench__runPill">
        <button
          className={`bench__runBtn${props.devMode ? " bench__runBtn--dev" : ""}`}
          disabled={props.running}
          onClick={() => handleRun("quick")}
        >
          {props.running ? "Running..." :
           copied ? "Copied! Run in terminal" :
           props.devMode ? "Copy command" : "Run Benchmark"}
        </button>
        {!props.running && (
          <button
            className="bench__runChevron"
            onClick={() => setMenuOpen(!menuOpen)}
            aria-label="Select benchmark mode"
          >
            <svg width="10" height="10" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="3" strokeLinecap="round" strokeLinejoin="round">
              <polyline points="6 9 12 15 18 9" />
            </svg>
          </button>
        )}
      </div>

      {menuOpen && (
        <>
          <div className="bench__runOverlay" onClick={() => setMenuOpen(false)} />
          <div className="bench__runMenu">
            <div className="bench__runMenuTitle">Benchmark mode{props.devMode ? " — copies to clipboard" : ""}</div>
            {(Object.entries(MODE_INFO) as [BenchmarkMode, typeof MODE_INFO["quick"]][]).map(([mode, info]) => (
              <button
                key={mode}
                className="bench__runMenuItem"
                onClick={() => handleRun(mode)}
              >
                <span className="bench__runMenuItemLabel">{info.label}</span>
                <span className="bench__runMenuItemDesc">{info.desc}</span>
              </button>
            ))}
          </div>
        </>
      )}
    </div>
  );
}
