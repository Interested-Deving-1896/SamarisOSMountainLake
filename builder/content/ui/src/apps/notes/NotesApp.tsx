import React, { useCallback, useEffect, useMemo, useState, useSyncExternalStore } from "react";
import { FileText, Save, Eye, Edit3 } from "lucide-react";
import { useFs } from "../../services/fs/useFs";
import { osStore } from "../../os/core/osStore";
import { windowManager } from "../../os/core/windowManager";
import { windowCloseGuards } from "../../system/windowing/windowCloseGuards";
import { useFileDrop } from "../shared/useFileDrop";
import { commitFileDrop } from "../../os/dnd";
import "./notes.css";

const DEFAULT_NOTE_PATH = "/User/Documents/quick-note.md";

function sanitizeHtml(html: string): string {
  return html
    .replace(/<script[\s\S]*?>[\s\S]*?<\/script>/gi, "")
    .replace(/\bon\w+\s*=\s*(?:"[^"]*"|'[^']*'|[^\s>]+)/gi, " ")
    .replace(/href\s*=\s*"javascript:/gi, 'href="#"')
    .replace(/href\s*=\s*'javascript:/gi, "href='#'");
}

function mdToHtml(md: string): string {
  let h = md
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;")
    .replace(/^### (.+)$/gm, "<h3>$1</h3>")
    .replace(/^## (.+)$/gm, "<h2>$1</h2>")
    .replace(/^# (.+)$/gm, "<h1>$1</h1>")
    .replace(/\*\*(.+?)\*\*/g, "<strong>$1</strong>")
    .replace(/\*(.+?)\*/g, "<em>$1</em>")
    .replace(/```([\s\S]*?)```/g, "<pre><code>$1</code></pre>")
    .replace(/`(.+?)`/g, "<code>$1</code>")
    .replace(/^> (.+)$/gm, "<blockquote>$1</blockquote>")
    .replace(/^- (.+)$/gm, "<li>$1</li>")
    .replace(/(<li>.*<\/li>\n?)+/g, "<ul>$&</ul>")
    .replace(/^(\d+)\. (.+)$/gm, "<li>$2</li>")
    .replace(/(<li>.*<\/li>\n?)+/g, (m) => m.includes("<ul>") ? m : `<ol>${m}</ol>`)
    .replace(/\n\n/g, "</p><p>")
    .replace(/\n/g, "<br>");
  return sanitizeHtml(`<p>${h}</p>`);
}

export function NotesApp(props: { windowId: string }) {
  const fs = useFs();
  const state = useSyncExternalStore((l) => osStore.subscribe(l), () => osStore.getState());
  const notePath = (state.windows.find((w) => w.id === props.windowId)?.params?.path as string | undefined) || DEFAULT_NOTE_PATH;
  const [content, setContent] = useState("");
  const [saved, setSaved] = useState("");
  const [status, setStatus] = useState("Ready");
  const [loading, setLoading] = useState(true);
  const [preview, setPreview] = useState(false);
  const [saveAsOpen, setSaveAsOpen] = useState(false);
  const dirty = !loading && content !== saved;

  const handleSaveAs = useCallback(async () => {
    const api = window.electronAPI?.dialog;
    if (!api) { setSaveAsOpen(true); return; }
    const result = await api.save("untitled.md");
    if (!result.canceled && result.filePath) {
      try { await fs.write(result.filePath, content); } catch {}
    }
  }, [fs, content]);

  useEffect(() => {
    let dead = false;
    setLoading(true);
    fs.read(notePath).then((r) => { if (!dead) { setContent(r.content); setSaved(r.content); setStatus("Loaded"); } })
      .catch(() => { if (!dead) { const f = "# New Note\n\nStart writing here."; setContent(f); setSaved(f); setStatus("New"); } })
      .finally(() => { if (!dead) setLoading(false); });
    return () => { dead = true; };
  }, [fs, notePath]);

  useEffect(() => windowCloseGuards.register(props.windowId, () => !dirty || window.confirm("Discard unsaved changes?")), [dirty, props.windowId]);

  const noteDrop = useFileDrop({
    accept: [".md", ".txt", ".html", ".xml", ".json", ".csv", ".yaml", ".yml", ".log", ".sh", ".js", ".ts", ".py", ".css"],
    target: { id: "notes-open", label: "Notes", path: "/User/Documents", kind: "app" },
    allowedChoices: ["open", "import", "copy"],
    recommendedAction: "open",
    onDrop: async (files, context) => {
      let readableFiles = files;
      if (files.some((f) => f.source === "host")) {
        const result = await commitFileDrop(fs, context.plan, { ...context.decision, choice: "copy" });
        readableFiles = result.completed.map((path) => ({
          name: path.split("/").pop() || path,
          path,
          kind: "file" as const,
          size: 0,
          source: "samaris" as const
        }));
      }
      for (const f of readableFiles) {
        try {
          const result = await fs.read(f.path);
          setContent(result.content);
          setSaved(result.content);
          setStatus(`Loaded: ${f.name}`);
        } catch {}
      }
    }
  });

  const wc = useMemo(() => content.trim() ? content.trim().split(/\s+/).length : 0, [content]);
  const html = useMemo(() => preview ? mdToHtml(content) : "", [content, preview]);

  const saveNote = useCallback(async () => {
    setStatus("Saving…");
    try { await fs.write(notePath, content); setSaved(content); setStatus("Saved"); } catch (e) { setStatus(e instanceof Error ? e.message : "Error"); }
  }, [fs, notePath, content]);

  const saveTo = useCallback(async (p: string) => {
    setStatus("Saving…");
    try { await fs.write(p, content); windowManager.updateLocal(props.windowId, { params: { path: p } }); setSaved(content); setStatus("Saved"); setSaveAsOpen(false); } catch (e) { throw e; }
  }, [fs, content]);

  const dotClass = dirty ? "notes-statusDot--unsaved" : status === "Saving…" ? "notes-statusDot--saving" : "notes-statusDot--saved";

  return (
    <div className={`notes ${noteDrop.isDragging ? " notes--drop-target" : ""}`}
      {...noteDrop.dragProps}>
      {/* Toolbar */}
      <div className="notes-toolbar">
        <div className="notes-toolbarIcon"><FileText size={16} /></div>
        <div className="notes-toolbarInfo">
          <div className="notes-toolbarTitle">{notePath.split("/").pop() || "untitled.md"}</div>
          <div className="notes-toolbarPath">{notePath}</div>
        </div>
        <div className="notes-toolbarActions">
          <button className="notes-btn" onClick={() => void handleSaveAs()}>Save As</button>
          <button className="notes-btn notes-btn--primary" onClick={() => void saveNote()}><Save size={14} /> Save</button>
        </div>
      </div>

      {/* Editor / Preview */}
      <div className="notes-editor">
        {loading ? (
          <div className="notes-empty"><FileText size={32} /><span>Loading note…</span></div>
        ) : preview ? (
          <div className="notes-preview" dangerouslySetInnerHTML={{ __html: html }} />
        ) : (
          <textarea className="notes-textarea" value={content} onChange={(e) => setContent(e.target.value)} placeholder="Start writing your note…" spellCheck={false} />
        )}
      </div>

      {/* Status Bar */}
      <div className="notes-status">
        <div className="notes-statusLeft">
          <span className={`notes-statusDot ${dotClass}`} />
          <span>{status}{dirty ? " · Unsaved" : ""}</span>
        </div>
        <div className="notes-statusRight">
          <span className="notes-statusBadge">{wc} words</span>
          <span className="notes-statusBadge">{content.length} chars</span>
          <button className={`notes-toggleBtn ${!preview ? "notes-toggleBtn--active" : ""}`} onClick={() => setPreview(false)}><Edit3 size={12} /> Edit</button>
          <button className={`notes-toggleBtn ${preview ? "notes-toggleBtn--active" : ""}`} onClick={() => setPreview(true)}><Eye size={12} /> Preview</button>
        </div>
      </div>

      {saveAsOpen && (
        <div className="iwa" style={{ position:"fixed", inset:0, zIndex:1, background:"rgba(0,0,0,0.3)", display:"grid", placeItems:"center" }}>
          <div style={{ background:"var(--bg2)", padding:24, borderRadius:16, display:"grid", gap:12, minWidth:320 }}>
            <strong>Save As</strong>
            <input id="saveAsPath" className="sfp__field input" defaultValue={notePath} style={{ padding:"10px 12px", borderRadius:10, border:"1px solid var(--stroke)", background:"var(--bg1)", color:"var(--text)", fontSize:13, width:"100%" }} />
            <div style={{ display:"flex", gap:8, justifyContent:"flex-end" }}>
              <button className="store__btn" onClick={() => setSaveAsOpen(false)}>Cancel</button>
              <button className="store__btn store__btn--primary" onClick={() => { const v = (document.getElementById("saveAsPath") as HTMLInputElement)?.value; if (v) { void saveTo(v); } }}>Save</button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
