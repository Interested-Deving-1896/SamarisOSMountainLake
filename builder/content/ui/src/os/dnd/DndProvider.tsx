import React from "react";
import "./dnd.css";
import { isDndV2Enabled } from "./featureFlag";
import type { ConflictStrategy, DropDecision, DropPlan } from "./types";

type DndContextValue = {
  requestFileDrop(plan: DropPlan): Promise<DropDecision | null>;
  activePlan: DropPlan | null;
};

const DndContext = React.createContext<DndContextValue | null>(null);

type PendingRequest = {
  plan: DropPlan;
  resolve: (decision: DropDecision | null) => void;
};

function formatBytes(bytes: number) {
  if (!bytes) return "";
  const units = ["B", "KB", "MB", "GB", "TB"];
  const index = Math.min(Math.floor(Math.log(bytes) / Math.log(1024)), units.length - 1);
  return `${(bytes / Math.pow(1024, index)).toFixed(index === 0 ? 0 : 1)} ${units[index]}`;
}

function choiceLabel(choice: string) {
  switch (choice) {
    case "copy": return "Copy";
    case "move": return "Move";
    case "link": return "Link";
    case "import": return "Import";
    case "restore": return "Restore";
    case "trash": return "Move to Trash";
    case "open": return "Open";
    default: return choice;
  }
}

export function DndProvider(props: { children: React.ReactNode }) {
  const [pending, setPending] = React.useState<PendingRequest | null>(null);
  const [conflictStrategy, setConflictStrategy] = React.useState<ConflictStrategy>("rename");

  const requestFileDrop = React.useCallback((plan: DropPlan) => {
    if (plan.allowedChoices.length === 0 || plan.estimatedCount === 0) return Promise.resolve(null);
    if (!isDndV2Enabled()) {
      if (plan.recommendedAction === "cancel") return Promise.resolve(null);
      return Promise.resolve({
        choice: plan.recommendedAction as DropDecision["choice"],
        conflictStrategy: "rename" as ConflictStrategy
      });
    }
    setConflictStrategy(plan.conflicts.length > 0 ? "rename" : "rename");
    return new Promise<DropDecision | null>((resolve) => {
      setPending({ plan, resolve });
    });
  }, []);

  const finish = React.useCallback((decision: DropDecision | null) => {
    setPending((current) => {
      current?.resolve(decision);
      return null;
    });
  }, []);

  React.useEffect(() => {
    if (!pending) return;
    const handleKey = (event: KeyboardEvent) => {
      if (event.key === "Escape") finish(null);
    };
    window.addEventListener("keydown", handleKey);
    return () => window.removeEventListener("keydown", handleKey);
  }, [finish, pending]);

  const value = React.useMemo<DndContextValue>(() => ({
    requestFileDrop,
    activePlan: pending?.plan || null
  }), [pending?.plan, requestFileDrop]);

  return (
    <DndContext.Provider value={value}>
      {props.children}
      {pending ? (
        <div className="dnd-sheet__backdrop" role="presentation" onPointerDown={() => finish(null)}>
          <section
            className="dnd-sheet"
            role="dialog"
            aria-modal="true"
            aria-label="Confirm file drop"
            onPointerDown={(event) => event.stopPropagation()}
          >
            <header className="dnd-sheet__header">
              <div>
                <div className="dnd-sheet__title">Confirm Drop</div>
                <div className="dnd-sheet__subtitle">
                  {pending.plan.estimatedCount} item{pending.plan.estimatedCount === 1 ? "" : "s"}
                  {pending.plan.estimatedBytes ? `, ${formatBytes(pending.plan.estimatedBytes)}` : ""}
                  {pending.plan.target.label ? ` to ${pending.plan.target.label}` : ""}
                </div>
              </div>
              <button className="dnd-sheet__close" type="button" onClick={() => finish(null)} aria-label="Cancel drop">x</button>
            </header>

            <div className="dnd-sheet__items">
              {pending.plan.source.entities.slice(0, 5).map((entity) => (
                <div className="dnd-sheet__item" key={entity.id}>
                  <span className="dnd-sheet__glyph">{entity.kind === "directory" ? "DIR" : "FILE"}</span>
                  <span className="dnd-sheet__name">{entity.name}</span>
                  {entity.size ? <span className="dnd-sheet__meta">{formatBytes(entity.size)}</span> : null}
                </div>
              ))}
              {pending.plan.source.entities.length > 5 ? (
                <div className="dnd-sheet__more">+{pending.plan.source.entities.length - 5} more</div>
              ) : null}
            </div>

            {pending.plan.conflicts.length > 0 ? (
              <div className="dnd-sheet__conflicts">
                <div className="dnd-sheet__sectionTitle">{pending.plan.conflicts.length} conflict{pending.plan.conflicts.length === 1 ? "" : "s"}</div>
                <div className="dnd-sheet__conflictControls" role="radiogroup" aria-label="Conflict strategy">
                  {(["rename", "replace", "skip"] as ConflictStrategy[]).map((strategy) => (
                    <button
                      key={strategy}
                      type="button"
                      className={`dnd-sheet__seg ${conflictStrategy === strategy ? "dnd-sheet__seg--active" : ""}`}
                      onClick={() => setConflictStrategy(strategy)}
                    >
                      {strategy === "rename" ? "Rename" : strategy === "replace" ? "Replace" : "Skip"}
                    </button>
                  ))}
                </div>
              </div>
            ) : null}

            {pending.plan.warnings.length > 0 ? (
              <div className="dnd-sheet__warnings">
                {pending.plan.warnings.map((warning) => <div key={warning}>{warning}</div>)}
              </div>
            ) : null}

            <footer className="dnd-sheet__actions">
              <button className="dnd-sheet__button" type="button" onClick={() => finish(null)}>Cancel</button>
              {pending.plan.allowedChoices.map((choice) => (
                <button
                  key={choice}
                  className={`dnd-sheet__button ${choice === pending.plan.recommendedAction ? "dnd-sheet__button--primary" : ""}`}
                  type="button"
                  onClick={() => finish({ choice: choice as DropDecision["choice"], conflictStrategy })}
                >
                  {choiceLabel(choice)}
                </button>
              ))}
            </footer>
          </section>
        </div>
      ) : null}
    </DndContext.Provider>
  );
}

export function useDnd() {
  const context = React.useContext(DndContext);
  if (!context) {
    return {
      activePlan: null,
      requestFileDrop: async (plan: DropPlan) => ({
        choice: plan.recommendedAction as DropDecision["choice"],
        conflictStrategy: "rename" as ConflictStrategy
      })
    };
  }
  return context;
}
