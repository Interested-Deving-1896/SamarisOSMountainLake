import React, { useSyncExternalStore, useCallback } from "react";
import { osStore } from "../../os/core/osStore";
import { kernelClient } from "../../os/kernel/kernelClient";
import { suggestModeFromApp } from "./services/contextResolver";
import { useOrbitChat } from "./hooks/useOrbitChat";
import { useVoiceMode } from "./hooks/useVoiceMode";
import { OrbitShell } from "./components/OrbitShell";
import "./orbit.css";

export default function Orbit(_props: { windowId: string }) {
  const osState = useSyncExternalStore(
    (listener) => osStore.subscribe(listener),
    () => osStore.getState()
  );
  const focusedWindow = osState.windows.find((window) => window.focused);
  const suggestedMode = suggestModeFromApp(focusedWindow?.appId);
  const orbit = useOrbitChat(suggestedMode);

  const voice = useVoiceMode(
    useCallback(
      (text: string) => orbit.addUserMessage(text),
      [orbit]
    ),
    useCallback(
      (text: string) => orbit.addAssistantMessage(text),
      [orbit]
    )
  );

  React.useEffect(() => {
    return () => {
      void kernelClient.request({ type: "orbit.shutdown", data: {} }, { timeoutMs: 5000 }).catch(() => {});
    };
  }, []);

  const handleVoiceToggle = useCallback(async () => {
    if (voice.isVoiceActive) {
      voice.stopVoiceMode();
    } else {
      orbit.createNewChat();
      await voice.startVoiceMode();
    }
  }, [voice, orbit]);

  return (
    <OrbitShell
      modeId={orbit.modeId}
      messages={orbit.messages}
      threads={orbit.threads}
      currentThreadId={orbit.currentThreadId}
      busy={orbit.busy}
      showReasoning={orbit.showReasoning}
      voiceState={voice.voiceState}
      audioLevel={voice.audioLevel}
      onSelectMode={orbit.setModeId}
      onToggleReasoning={() => orbit.setShowReasoning()}
      onSelectThread={orbit.selectThread}
      onRenameThread={orbit.renameThread}
      onDeleteThread={orbit.deleteThread}
      onNewChat={orbit.createNewChat}
      onSubmit={(value) => void orbit.sendMessage(value)}
      onVoiceToggle={handleVoiceToggle}
    />
  );
}
