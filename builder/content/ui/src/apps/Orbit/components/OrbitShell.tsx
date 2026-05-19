import React from "react";
import { PanelLeftClose, PanelLeftOpen } from "lucide-react";
import { MODE_BY_ID } from "../constants/modes";
import type { OrbitMessage, OrbitModeId, OrbitThread, VoiceState } from "../types";
import { ChatContainer } from "./ChatContainer";
import { ChatInput } from "./ChatInput";
import { ModeSelector } from "./ModeSelector";
import { ThreadSidebar } from "./ThreadSidebar";
import { VoiceButton } from "./VoiceButton";

export function OrbitShell(props: {
  modeId: OrbitModeId;
  messages: OrbitMessage[];
  threads: OrbitThread[];
  currentThreadId: string;
  busy: boolean;
  showReasoning: boolean;
  voiceState: VoiceState;
  audioLevel: number;
  onSelectMode: (modeId: OrbitModeId) => void;
  onToggleReasoning: () => void;
  onSelectThread: (threadId: string) => void;
  onRenameThread: (threadId: string, title: string) => void;
  onDeleteThread: (threadId: string) => void;
  onNewChat: () => void;
  onSubmit: (value: string) => void;
  onVoiceToggle: () => void;
}) {
  const activeMode = MODE_BY_ID[props.modeId];
  const [sidebarOpen, setSidebarOpen] = React.useState(true);

  return (
    <div className="orbit">
      <header className="orbit__header">
        <div className="orbit__headerLeft">
          <button type="button" className="orbit__ghostBtn" onClick={() => setSidebarOpen((current) => !current)}>
            {sidebarOpen ? <PanelLeftClose size={15} strokeWidth={2.2} /> : <PanelLeftOpen size={15} strokeWidth={2.2} />}
            <span>{sidebarOpen ? "Hide chats" : "Show chats"}</span>
          </button>
          <button type="button" className="orbit__ghostBtn" onClick={props.onNewChat}>
            <span>New chat</span>
          </button>
          <ModeSelector activeModeId={props.modeId} onSelect={props.onSelectMode} />
        </div>
        <div className="orbit__headerCenter">Orbit</div>
        <div className="orbit__headerRight">
          <VoiceButton voiceState={props.voiceState} audioLevel={props.audioLevel} onToggle={props.onVoiceToggle} />
        </div>
      </header>

      <div className={`orbit__workspace ${sidebarOpen ? "" : "orbit__workspace--sidebarHidden"}`}>
        {sidebarOpen ? (
          <ThreadSidebar
            threads={props.threads}
            currentThreadId={props.currentThreadId}
            onSelectThread={props.onSelectThread}
            onRenameThread={(threadId) => {
              const current = props.threads.find((thread) => thread.id === threadId);
              const nextTitle = window.prompt("Rename conversation", current?.title || "");
              if (nextTitle) {
                props.onRenameThread(threadId, nextTitle);
              }
            }}
            onDeleteThread={props.onDeleteThread}
          />
        ) : null}

        <section className="orbit__mainPane">
          <div className="orbit__workspaceHead">
            <div className="orbit__workspaceTitle">{activeMode.label}</div>
            <div className="orbit__workspaceSubline">Private local chat</div>
          </div>

          <ChatContainer
            messages={props.messages}
            showReasoning={props.showReasoning}
            onUsePrompt={props.onSubmit}
          />

          <ChatInput busy={props.busy} onSubmit={props.onSubmit} />
        </section>
      </div>
    </div>
  );
}
