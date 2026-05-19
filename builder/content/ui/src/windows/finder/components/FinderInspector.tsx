import React, { useEffect, useState } from "react";
import { Eye, Pencil, Send, Trash2 } from "lucide-react";
import type { FsNode } from "../../../services/fs/types";
import type { FinderPreview as FinderPreviewState } from "../model";
import { formatBytes, formatDate, fileKind, joinPath } from "../utils";
import { SamarisIcon, iconNameForFile } from "../../../modules/icons";
import { getThumbnail } from "../../../services/fs/fileThumbnails";
import { useFs } from "../../../services/fs/useFs";

export const FinderInspector = React.memo(function FinderInspector(props: {
  selectedNode: FsNode | null;
  preview: FinderPreviewState;
  previewLoading: boolean;
  pathStr: string;
  selectionCount: number;
  onRename?: (name: string) => void;
  onDelete?: (name: string) => void;
  onSendToDocuments?: (path: string) => void;
}) {
  const fs = useFs();
  const [thumbnail, setThumbnail] = useState<string | null>(null);
  const node = props.selectedNode;

  useEffect(() => {
    setThumbnail(null);
    if (!node || node.kind === "dir") return;
    const name = node.name.toLowerCase();
    if (name.match(/\.(png|jpg|jpeg|gif|webp|svg)$/)) {
      let cancelled = false;
      getThumbnail(props.preview?.path || "", 200, fs).then((result) => {
        if (!cancelled) setThumbnail(result);
      });
      return () => { cancelled = true; };
    }
  }, [node, props.preview?.path, fs]);

  if (props.selectionCount === 0) {
    return (
      <aside className="finder-inspector">
        <div className="finder-inspector__header"><Eye size={13} /><span>Inspector</span></div>
        <div className="finder-inspector__empty">Select a file or folder to see details.</div>
      </aside>
    );
  }

  if (props.selectionCount > 1) {
    return (
      <aside className="finder-inspector">
        <div className="finder-inspector__header"><Eye size={13} /><span>Inspector</span></div>
        <div className="finder-inspector__empty">{props.selectionCount} items selected.</div>
      </aside>
    );
  }

  if (!node) return null;

  const fullPath = joinPath(props.pathStr, node.name);

  return (
    <aside className="finder-inspector">
      <div className="finder-inspector__header"><Eye size={13} /><span>Inspector</span></div>
      <div className="finder-inspector__body">
        <div className="finder-inspector__icon">
          <SamarisIcon name={iconNameForFile(node.name, node.kind)} size={64} variant="soft" />
        </div>
        <div className="finder-inspector__name">{node.name}</div>
        <div className="finder-inspector__kind">{node.kind === "dir" ? "Folder" : fileKind(node.name)}</div>

        {thumbnail ? (
          <div className="finder-inspector__thumb">
            <img src={thumbnail} alt="" />
          </div>
        ) : null}

        <div className="finder-inspector__meta">
          <div className="finder-inspector__metaRow"><span>Size</span><span>{node.kind === "file" ? formatBytes(node.size) : "—"}</span></div>
          <div className="finder-inspector__metaRow"><span>Modified</span><span>{formatDate(node.modifiedAt)}</span></div>
          <div className="finder-inspector__metaRow"><span>Path</span><span className="finder-inspector__metaPath">{fullPath}</span></div>
        </div>

        <div className="finder-inspector__actions">
          <button type="button" onClick={() => props.onRename?.(node.name)}><Pencil size={13} /> Rename</button>
          <button type="button" onClick={() => props.onSendToDocuments?.(fullPath)}><Send size={13} /> Move to Documents</button>
          <button type="button" className="finder-inspector__action--danger" onClick={() => props.onDelete?.(node.name)}><Trash2 size={13} /> Delete</button>
        </div>

        {props.preview?.content ? (
          <div className="finder-inspector__preview">
            {props.previewLoading ? "Loading preview…" : props.preview.content}
          </div>
        ) : null}
      </div>
    </aside>
  );
});
