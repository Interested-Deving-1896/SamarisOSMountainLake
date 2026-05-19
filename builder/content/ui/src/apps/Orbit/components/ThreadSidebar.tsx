import React from "react";
import { MessageSquareText, Pencil, Trash2 } from "lucide-react";
import type { OrbitThread } from "../types";

export function ThreadSidebar(props: {
  threads: OrbitThread[];
  currentThreadId: string;
  onSelectThread: (threadId: string) => void;
  onRenameThread: (threadId: string) => void;
  onDeleteThread: (threadId: string) => void;
}) {
  return (
    <aside className="orbit__sidebar">
      <div className="orbit__sidebarTitle">Chats</div>
      <div className="orbit__threadList">
        {props.threads.map((thread) => (
          <button
            key={thread.id}
            type="button"
            className={`orbit__threadItem ${thread.id === props.currentThreadId ? "orbit__threadItem--active" : ""}`}
            onClick={() => props.onSelectThread(thread.id)}
          >
            <MessageSquareText size={15} strokeWidth={2.2} />
            <span>{thread.title}</span>
            <span className="orbit__threadActions">
              <button
                type="button"
                className="orbit__threadAction"
                onClick={(event) => {
                  event.stopPropagation();
                  props.onRenameThread(thread.id);
                }}
              >
                <Pencil size={12} strokeWidth={2.2} />
              </button>
              <button
                type="button"
                className="orbit__threadAction"
                onClick={(event) => {
                  event.stopPropagation();
                  props.onDeleteThread(thread.id);
                }}
              >
                <Trash2 size={12} strokeWidth={2.2} />
              </button>
            </span>
          </button>
        ))}
      </div>
    </aside>
  );
}
