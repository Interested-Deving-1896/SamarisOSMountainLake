import React from "react";
import { useFs } from "../../services/fs/useFs";
import type { DragFilePayload, DropDecision, FileDropHandler, FileDropOptions } from "../../os/dnd";
import {
  buildFileDropPlan,
  filterAcceptedFiles,
  getActiveDndSource,
  hasSamarisFileDrop,
  nativeDndBridge,
  readDndFiles,
  readDndSource,
  useDnd
} from "../../os/dnd";

export function useFileDrop(options: {
  onDrop: FileDropHandler;
  accept?: string[];
} & FileDropOptions) {
  const fs = useFs();
  const dnd = useDnd();
  const [isDragging, setIsDragging] = React.useState(false);
  const dragCounter = React.useRef(0);

  const shouldIgnoreSourceId = React.useCallback((sourceId?: string) => {
    if (!sourceId || !options.ignoreSourceIds) return false;
    if (typeof options.ignoreSourceIds === "function") return options.ignoreSourceIds(sourceId);
    return options.ignoreSourceIds.includes(sourceId);
  }, [options.ignoreSourceIds]);

  const getSourceId = React.useCallback((dt: DataTransfer) => {
    const activeSource = getActiveDndSource();
    if (activeSource) return activeSource.id;
    try {
      return readDndSource(dt)?.id;
    } catch {
      return undefined;
    }
  }, []);

  const ignoreCurrentDrag = React.useCallback((dt: DataTransfer) => {
    return shouldIgnoreSourceId(getSourceId(dt));
  }, [getSourceId, shouldIgnoreSourceId]);

  const hasSupportedDrop = React.useCallback((dt: DataTransfer) => {
    if (ignoreCurrentDrag(dt)) return false;
    if (hasSamarisFileDrop(dt)) return true;
    return dt.types.includes("Files");
  }, [ignoreCurrentDrag]);

  const handleDragOver = React.useCallback((e: React.DragEvent) => {
    if (!hasSupportedDrop(e.dataTransfer)) {
      if (isDragging) {
        dragCounter.current = 0;
        setIsDragging(false);
      }
      return;
    }
    e.preventDefault();
    e.stopPropagation();
    e.dataTransfer.dropEffect = "copy";
  }, [hasSupportedDrop, isDragging]);

  const handleDragEnter = React.useCallback((e: React.DragEvent) => {
    if (!hasSupportedDrop(e.dataTransfer)) return;
    e.preventDefault();
    e.stopPropagation();
    dragCounter.current++;
    if (dragCounter.current === 1) {
      setIsDragging(true);
    }
  }, [hasSupportedDrop]);

  const handleDragLeave = React.useCallback((e: React.DragEvent) => {
    if (!hasSupportedDrop(e.dataTransfer)) return;
    e.preventDefault();
    e.stopPropagation();
    dragCounter.current--;
    if (dragCounter.current <= 0) {
      dragCounter.current = 0;
      setIsDragging(false);
    }
  }, [hasSupportedDrop]);

  const handleDrop = React.useCallback(
    (e: React.DragEvent) => {
      e.preventDefault();
      e.stopPropagation();
      dragCounter.current = 0;
      setIsDragging(false);

      if (ignoreCurrentDrag(e.dataTransfer)) return;
      const internalFiles = readDndFiles(e.dataTransfer);
      const externalFileList = Array.from(e.dataTransfer.files || []);

      void (async () => {
        const externalFiles = internalFiles.length > 0 ? [] : await nativeDndBridge.resolveExternalFiles(externalFileList);
        const files = filterAcceptedFiles([...internalFiles, ...externalFiles], options.accept);
        if (files.length === 0) return;

        const target = typeof options.target === "function" ? options.target() : options.target;
        const plan = await buildFileDropPlan(
          fs,
          files,
          target || { id: "drop-zone", label: "this app", kind: "app" },
          {
            allowedChoices: options.allowedChoices,
            recommendedAction: options.recommendedAction
          }
        );

        if (plan.recommendedAction === "cancel") return;
        const decision: DropDecision | null = options.requireConfirmation === false
          ? { choice: plan.recommendedAction, conflictStrategy: "rename" }
          : await dnd.requestFileDrop(plan);
        if (!decision) return;
        await options.onDrop(files, { plan, decision, fs });
      })();
    },
    [dnd, fs, ignoreCurrentDrag, options]
  );

  return {
    isDragging,
    dragProps: {
      onDragOver: handleDragOver,
      onDragEnter: handleDragEnter,
      onDragLeave: handleDragLeave,
      onDrop: handleDrop,
    },
  };
}
