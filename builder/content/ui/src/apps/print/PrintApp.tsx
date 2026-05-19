import React from "react";
import { printKernel } from "../../services/kernel/print";
import { PromptModal } from "../../components/PromptModal";

export function PrintApp(_props: { windowId: string }) {
  const [state, setState] = React.useState<Awaited<ReturnType<typeof printKernel.list>> | null>(null);
  const [adding, setAdding] = React.useState(false);
  const [note, setNote] = React.useState<string>("");

  const refresh = React.useCallback(async () => setState(await printKernel.list()), []);
  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  return (
    <div className="print">
      <div className="print__hero">
        <div className="print__title">Print</div>
        <button type="button" className="print__btn print__btn--primary" onClick={() => setAdding(true)}>
          Add printer
        </button>
      </div>
      {note ? <div className="print__note">{note}</div> : null}
      <div className="print__grid">
        <section className="print__panel">
          <div className="print__panelTitle">Printers</div>
          <div className="print__list">
            {state?.printers.map((printer) => (
              <div key={printer.id} className="print__row">
                <div>
                  <div className="print__rowTitle">{printer.name}</div>
                  <div className="print__rowMeta">{printer.status} • {printer.source}</div>
                </div>
                <button type="button" className="print__btn" onClick={() => void printKernel.remove(printer.id).then(() => refresh())}>
                  Remove
                </button>
              </div>
            ))}
          </div>
        </section>
        <section className="print__panel">
          <div className="print__panelTitle">Queue</div>
          <div className="print__list">
            {state?.queue.length ? state.queue.map((job) => (
              <div key={job.jobId} className="print__row">
                <div>
                  <div className="print__rowTitle">{job.jobId}</div>
                  <div className="print__rowMeta">{job.summary}</div>
                </div>
              </div>
            )) : <div className="print__rowMeta">No queued jobs.</div>}
          </div>
        </section>
      </div>
      {adding ? (
        <PromptModal
          title="Add printer"
          subtitle="Use Name|URI|Protocol (example: Studio|ipp://printer.local/ipp/print|ipp)"
          placeholder="Studio|ipp://printer.local/ipp/print|ipp"
          confirmLabel="Save"
          onCancel={() => setAdding(false)}
          onConfirm={(value) => {
            setAdding(false);
            const [name, uri, protocol] = value.split("|");
            void printKernel.add({ name, uri, protocol }).then((result) => {
              setNote(result.note);
              void refresh();
            });
          }}
        />
      ) : null}
    </div>
  );
}
