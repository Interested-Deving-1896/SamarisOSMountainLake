import React from "react";
import { ClipboardList, Trash2, Copy } from "lucide-react";
import { systemClipboard, readHistory, clearHistory } from "../../os/filesystem/clipboard";

export function ClipboardHistoryPanel(props: { onClose: () => void }) {
  const [items, setItems] = React.useState(() => readHistory());

  const handleCopy = async (text: string) => {
    await systemClipboard.writeText(text);
    setItems(readHistory());
  };

  const handleClear = () => {
    clearHistory();
    setItems([]);
  };

  return (
    <div className="clipboard-history">
      <div className="clipboard-history__header">
        <ClipboardList size={14} />
        <span>Clipboard History</span>
        {items.length > 0 && (
          <button type="button" className="clipboard-history__clear" onClick={handleClear}>
            <Trash2 size={12} />
          </button>
        )}
      </div>
      <div className="clipboard-history__list">
        {items.length === 0 ? (
          <div className="clipboard-history__empty">Nothing copied yet.</div>
        ) : (
          items.map((entry, i) => (
            <button
              key={`${entry.timestamp}-${i}`}
              type="button"
              className="clipboard-history__item"
              onClick={() => handleCopy(entry.text)}
            >
              <div className="clipboard-history__text">{entry.text.length > 80 ? entry.text.slice(0, 80) + "…" : entry.text}</div>
              <div className="clipboard-history__meta">
                <Copy size={10} />
                <span>{new Date(entry.timestamp).toLocaleTimeString()}</span>
              </div>
            </button>
          ))
        )}
      </div>
    </div>
  );
}
