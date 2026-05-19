import React from "react";
import { Trash2 } from "lucide-react";

export function TrashEmptyState() {
  return (
    <div className="trash__empty">
      <div className="trash__emptyGlyph">
        <Trash2 size={72} strokeWidth={1.4} />
      </div>
      <h2 className="trash__emptyTitle">Trash is Empty</h2>
      <p className="trash__emptyText">
        Deleted items will appear here. You can restore them to their
        original location or permanently remove them.
      </p>
    </div>
  );
}
