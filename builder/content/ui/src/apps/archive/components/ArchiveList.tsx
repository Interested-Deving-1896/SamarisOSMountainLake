import { File, Folder, Loader } from "lucide-react";
import type { ArchiveEntry } from "../../../services/kernel/archive";

export function ArchiveList(props: { entries: ArchiveEntry[]; loading: boolean; error: string | null; successNotice: string | null }) {
  if (props.loading) {
    return (
      <div className="archive__center">
        <Loader size={18} className="archive__spin" />
        <span>Reading archive…</span>
      </div>
    );
  }

  if (props.error) {
    return <div className="archive__center archive__error">{props.error}</div>;
  }

  if (props.successNotice) {
    return <div className="archive__center archive__success">{props.successNotice}</div>;
  }

  if (!props.entries.length) {
    return <div className="archive__center">Archive is empty.</div>;
  }

  return (
    <div className="archive__list">
      {props.entries.map((entry, index) => (
        <div key={entry.name + index} className="archive__row">
          {entry.kind === "dir" ? <Folder size={16} /> : <File size={16} />}
          <span className="archive__rowName">{entry.name}</span>
          {entry.size != null && (
            <span className="archive__rowSize">
              {entry.size > 1024 * 1024
                ? `${(entry.size / 1024 / 1024).toFixed(1)} MB`
                : `${(entry.size / 1024).toFixed(0)} KB`}
            </span>
          )}
        </div>
      ))}
    </div>
  );
}
