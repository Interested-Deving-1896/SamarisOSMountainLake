import React from "react";
import { FileText, Save } from "lucide-react";

export function TextEditorToolbar(props: {
  path: string;
  status: string;
  dirty?: boolean;
  onSave: () => void;
  onSaveAs: () => void;
}) {
  return (
    <div className="textedit__toolbar">
      <div className="textedit__meta">
        <span className="textedit__icon" aria-hidden="true">
          <FileText size={16} strokeWidth={2.2} />
        </span>
        <div className="textedit__titles">
          <div className="textedit__title">Text Editor</div>
          <div className="textedit__path">{props.path}</div>
        </div>
      </div>
      <div className="textedit__actions">
        <div className="textedit__status">{props.status}</div>
        <button type="button" className="textedit__save" onClick={props.onSave}>
          <Save size={14} strokeWidth={2.2} />
          <span>Save</span>
        </button>
        <button type="button" className="textedit__save" onClick={props.onSaveAs}>
          <span>Save As</span>
        </button>
        {props.dirty ? <div className="textedit__status">Unsaved changes</div> : null}
      </div>
    </div>
  );
}
