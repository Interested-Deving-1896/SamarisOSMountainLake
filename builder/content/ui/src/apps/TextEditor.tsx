import React, { useEffect, useMemo, useState, useSyncExternalStore } from "react";
import { osStore } from "../os/core/osStore";
import { useFs } from "../services/fs/useFs";
import { TextEditorArea } from "./textedit/components/TextEditorArea";
import { TextEditorToolbar } from "./textedit/components/TextEditorToolbar";
import { windowManager } from "../os/core/windowManager";
import { windowCloseGuards } from "../system/windowing/windowCloseGuards";
import "./textedit/textedit.css";

const DEFAULT_PATH = "/User/Desktop/untitled.txt";

export default function TextEditor(props: { windowId: string }) {
  const fs = useFs();
  const state = useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const path =
    (state.windows.find((win) => win.id === props.windowId)?.params?.path as string | undefined) || DEFAULT_PATH;
  const [content, setContent] = useState("");
  const [savedContent, setSavedContent] = useState("");
  const [status, setStatus] = useState("Ready");
  const [loading, setLoading] = useState(true);
  const dirty = !loading && content !== savedContent;

  useEffect(() => {
    let cancelled = false;
    setLoading(true);
    fs.read(path)
      .then((result) => {
        if (cancelled) return;
        setContent(result.content);
        setSavedContent(result.content);
        setStatus("Loaded");
      })
      .catch(() => {
        if (cancelled) return;
        setContent("");
        setSavedContent("");
        setStatus("New file");
      })
      .finally(() => {
        if (!cancelled) setLoading(false);
      });

    return () => {
      cancelled = true;
    };
  }, [fs, path]);

  const lineCount = useMemo(() => content.split("\n").length, [content]);

  useEffect(() => {
    return windowCloseGuards.register(props.windowId, () => {
      if (!dirty) return true;
      return window.confirm("Discard unsaved changes in Text Editor?");
    });
  }, [dirty, props.windowId]);

  async function save() {
    try {
      setStatus("Saving...");
      await fs.write(path, content);
      setSavedContent(content);
      setStatus(`Saved • ${lineCount} lines`);
    } catch (error) {
      setStatus(error instanceof Error ? `Save failed: ${error.message}` : "Save failed");
    }
  }

  async function saveToPath(nextPath: string) {
    try {
      setStatus("Saving...");
      await fs.write(nextPath, content);
      windowManager.updateLocal(props.windowId, { params: { path: nextPath } });
      setSavedContent(content);
      setStatus(`Saved • ${lineCount} lines`);
    } catch (error) {
      throw error instanceof Error ? error : new Error("Save failed");
    }
  }

  async function saveAs() {
    const api = window.electronAPI?.dialog;
    if (api) {
      const result = await api.save("untitled.txt");
      if (!result.canceled && result.filePath) {
        await saveToPath(result.filePath);
      }
    } else {
      const p = prompt("Save as path:", path);
      if (p) await saveToPath(p);
    }
  }

  return (
    <>
      <div className="textedit">
        <TextEditorToolbar path={path} status={status} dirty={dirty} onSave={() => void save()} onSaveAs={() => void saveAs()} />
        <TextEditorArea value={content} loading={loading} onChange={setContent} />
      </div>
    </>
  );
}
