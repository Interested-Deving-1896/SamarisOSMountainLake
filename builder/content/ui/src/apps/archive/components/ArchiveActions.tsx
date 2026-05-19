import { Download } from "lucide-react";

export function ArchiveActions(props: {
  archivePath: string;
  destination: string;
  onExtract: () => void;
  extracting: boolean;
}) {
  return (
    <div className="archive__toolbar">
      <div className="archive__toolbarTitle">{props.archivePath.split("/").pop()}</div>
      <button type="button" className="archive__extractBtn" disabled={props.extracting} onClick={props.onExtract}>
        <Download size={14} strokeWidth={2.2} />
        {props.extracting ? "Extracting…" : "Extract All"}
      </button>
    </div>
  );
}
