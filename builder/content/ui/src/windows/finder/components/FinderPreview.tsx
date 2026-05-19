import React, { useEffect, useState } from "react";
import { Eye } from "lucide-react";
import type { FsNode } from "../../../services/fs/types";
import type { FinderPreview as FinderPreviewState } from "../model";
import { formatBytes, formatDate, fileKind } from "../utils";
import { SamarisIcon, iconNameForFile } from "../../../modules/icons";
import { getThumbnail } from "../../../services/fs/fileThumbnails";
import { useFs } from "../../../services/fs/useFs";

export const FinderPreview = React.memo(function FinderPreview(props: {
  selectedNode: FsNode | null;
  preview: FinderPreviewState;
  previewLoading: boolean;
}) {
  const fs = useFs();
  const [thumbnail, setThumbnail] = useState<string | null>(null);

  useEffect(() => {
    setThumbnail(null);
    if (!props.selectedNode || props.selectedNode.kind === "dir") return;
    const name = props.selectedNode.name.toLowerCase();
    if (name.match(/\.(png|jpg|jpeg|gif|webp|svg)$/)) {
      const path = props.preview?.path || "";
      let cancelled = false;
      getThumbnail(path, 120, fs).then((result) => {
        if (!cancelled) setThumbnail(result);
      });
      return () => { cancelled = true; };
    }
  }, [props.selectedNode, props.preview?.path, fs]);

  return (
    <aside className="finder__preview">
      <div className="finder__previewHeader">
        <Eye size={13} strokeWidth={2.2} />
        <span>Quick Look</span>
      </div>

      {props.selectedNode ? (
        <div className="finder__previewMeta">
          <div className="finder__previewTitleRow">
            <SamarisIcon
              className="finder__previewIcon"
              name={iconNameForFile(props.selectedNode.name, props.selectedNode.kind)}
              size={28}
              variant="soft"
              surface="bare"
            />
            <div className="finder__previewTitle">{props.selectedNode.name}</div>
          </div>
          <div className="finder__previewSubtle">
            {props.selectedNode.kind === "dir" ? "Folder" : fileKind(props.selectedNode.name)}
            {props.selectedNode.size && props.selectedNode.kind === "file" ? ` — ${formatBytes(props.selectedNode.size)}` : ""}
            {props.selectedNode.modifiedAt ? ` — ${formatDate(props.selectedNode.modifiedAt)}` : ""}
          </div>
          {(thumbnail || props.selectedNode.kind === "dir") && (
            <div className="finder__previewThumb">
              {thumbnail ? <img src={thumbnail} alt="" className="finder__previewThumbImg" /> : (
                <SamarisIcon name="folder" size={48} variant="soft" surface="bare" />
              )}
            </div>
          )}
        </div>
      ) : (
        <div className="finder__previewMeta">
          <div className="finder__previewTitle">No selection</div>
          <div className="finder__previewSubtle">Select a file or folder to inspect it.</div>
        </div>
      )}

      <div className="finder__previewBody">
        {props.previewLoading ? "Loading preview…" : props.preview?.content || "No preview yet."}
      </div>
    </aside>
  );
});
