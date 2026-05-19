import React, { startTransition } from "react";
import { MODE_BY_ID } from "../constants/modes";
import { generateLocalReasoning } from "../services/localModelProvider";
import { useOrbitStore } from "../store/useOrbitStore";
import type { OrbitMessage, OrbitModeId, OrbitThread } from "../types";

const ORBIT_CHAT_HISTORY_KEY = "samaris-os/orbit-threads";

function makeId(prefix: string) {
  return `${prefix}-${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function makeMessage(input: Partial<OrbitMessage> & Pick<OrbitMessage, "role" | "content">): OrbitMessage {
  return {
    id: makeId(input.role),
    createdAt: new Date().toISOString(),
    ...input
  };
}

function createThread(seedTitle = "New chat"): OrbitThread {
  const now = new Date().toISOString();
  return {
    id: makeId("thread"),
    title: seedTitle,
    messages: [],
    createdAt: now,
    updatedAt: now
  };
}

function loadThreadState() {
  try {
    const raw = window.localStorage.getItem(ORBIT_CHAT_HISTORY_KEY);
    if (!raw) {
      const firstThread = createThread();
      return {
        threads: [firstThread] as OrbitThread[],
        currentThreadId: firstThread.id
      };
    }
    const parsed = JSON.parse(raw) as { threads?: OrbitThread[]; currentThreadId?: string };
    const threads = Array.isArray(parsed.threads) && parsed.threads.length > 0 ? parsed.threads : [createThread()];
    const currentThreadId =
      parsed.currentThreadId && threads.some((thread) => thread.id === parsed.currentThreadId)
        ? parsed.currentThreadId
        : threads[0].id;
    return { threads, currentThreadId };
  } catch {
    const firstThread = createThread();
    return {
      threads: [firstThread] as OrbitThread[],
      currentThreadId: firstThread.id
    };
  }
}

export function useOrbitChat(initialModeId: OrbitModeId) {
  const store = useOrbitStore();
  const [busy, setBusy] = React.useState(false);
  const [threadState, setThreadState] = React.useState(loadThreadState);
  const threads = threadState.threads;
  const currentThreadId = threadState.currentThreadId;

  React.useEffect(() => {
    window.localStorage.setItem(ORBIT_CHAT_HISTORY_KEY, JSON.stringify(threadState));
  }, [threadState]);

  const activeThread = React.useMemo(() => {
    return threads.find((thread) => thread.id === currentThreadId) || threads[0] || null;
  }, [currentThreadId, threads]);

  React.useEffect(() => {
    if (activeThread && activeThread.messages.length === 0) {
      store.setModeId(initialModeId);
    }
  }, [activeThread, initialModeId, store]);

  const updateThreadMessages = React.useCallback(
    (updater: (messages: OrbitMessage[]) => OrbitMessage[]) => {
      setThreadState((current) => ({
        ...current,
        threads: current.threads.map((thread) =>
          thread.id === currentThreadId
            ? {
                ...thread,
                messages: updater(thread.messages),
                updatedAt: new Date().toISOString()
              }
            : thread
        )
      }));
    },
    [currentThreadId]
  );

  const createNewChat = React.useCallback(() => {
    const next = createThread();
    setThreadState((current) => ({
      currentThreadId: next.id,
      threads: [next, ...current.threads]
    }));
    store.setModeId(initialModeId);
  }, [initialModeId, store]);

  const selectThread = React.useCallback((threadId: string) => {
    setThreadState((current) => ({
      ...current,
      currentThreadId: threadId
    }));
  }, []);

  const renameThread = React.useCallback((threadId: string, title: string) => {
    const trimmed = title.trim();
    if (!trimmed) return;
    setThreadState((current) => ({
      ...current,
      threads: current.threads.map((thread) =>
        thread.id === threadId ? { ...thread, title: trimmed, updatedAt: new Date().toISOString() } : thread
      )
    }));
  }, []);

  const deleteThread = React.useCallback((threadId: string) => {
    setThreadState((current) => {
      const nextThreads = current.threads.filter((thread) => thread.id !== threadId);
      if (!nextThreads.length) {
        const fallback = createThread();
        return { currentThreadId: fallback.id, threads: [fallback] };
      }
      return {
        currentThreadId:
          current.currentThreadId === threadId ? nextThreads[0].id : current.currentThreadId,
        threads: nextThreads
      };
    });
  }, []);

  const sendMessage = React.useCallback(
    async (prompt: string) => {
      const trimmed = prompt.trim();
      if (!trimmed || busy || !activeThread) return;

      const userMessage = makeMessage({
        role: "user",
        content: trimmed,
        modeId: store.modeId
      });
      const assistantMessage = makeMessage({
        role: "assistant",
        content: "",
        modeId: store.modeId,
        reasoningContent: "",
        streaming: true
      });

      startTransition(() => {
        setThreadState((current) => ({
          ...current,
          threads: current.threads
            .map((thread) =>
              thread.id === currentThreadId
                ? {
                    ...thread,
                    title: thread.messages.length === 0 ? trimmed.slice(0, 48) : thread.title,
                    messages: [...thread.messages, userMessage, assistantMessage],
                    updatedAt: new Date().toISOString()
                  }
                : thread
            )
            .sort((a, b) => Date.parse(b.updatedAt) - Date.parse(a.updatedAt))
        }));
      });
      setBusy(true);

      const mode = MODE_BY_ID[store.modeId];
      try {
        const result = await generateLocalReasoning({
          prompt: trimmed,
          modeId: store.modeId,
          strategy: mode.strategy,
          onReasoning: (chunk) => {
            startTransition(() => {
              updateThreadMessages((messages) =>
                messages.map((message) =>
                  message.id === assistantMessage.id
                    ? { ...message, reasoningContent: (message.reasoningContent || "") + chunk }
                    : message
                )
              );
            });
          },
          onToken: (content) => {
            startTransition(() => {
              updateThreadMessages((messages) =>
                messages.map((message) =>
                  message.id === assistantMessage.id ? { ...message, content, streaming: true } : message
                )
              );
            });
          }
        });

        startTransition(() => {
          updateThreadMessages((messages) =>
            messages.map((message) =>
              message.id === assistantMessage.id
                ? { ...message, content: result.finalAnswer, streaming: false }
                : message
            )
          );
        });
      } catch (error) {
        startTransition(() => {
          updateThreadMessages((messages) =>
            messages.map((entry) =>
              entry.id === assistantMessage.id
                ? {
                    ...entry,
                    content: humanizeOrbitError(error),
                    streaming: false
                  }
                : entry
            )
          );
        });
      } finally {
        setBusy(false);
      }
    },
    [activeThread, busy, currentThreadId, store, updateThreadMessages]
  );

  const addUserMessage = React.useCallback(
    (text: string) => {
      const msg = makeMessage({ role: "user", content: text, modeId: store.modeId });
      startTransition(() => {
        setThreadState((current) => ({
          ...current,
          threads: current.threads.map((thread) =>
            thread.id === currentThreadId
              ? { ...thread, messages: [...thread.messages, msg], updatedAt: new Date().toISOString() }
              : thread
          ),
        }));
      });
    },
    [currentThreadId, store.modeId]
  );

  const addAssistantMessage = React.useCallback(
    (text: string) => {
      const msg = makeMessage({ role: "assistant", content: text, modeId: store.modeId, streaming: false });
      startTransition(() => {
        setThreadState((current) => ({
          ...current,
          threads: current.threads.map((thread) =>
            thread.id === currentThreadId
              ? { ...thread, messages: [...thread.messages, msg], updatedAt: new Date().toISOString() }
              : thread
          ),
        }));
      });
    },
    [currentThreadId, store.modeId]
  );

  return {
    threads,
    currentThreadId,
    selectThread,
    messages: activeThread?.messages || [],
    modeId: store.modeId,
    setModeId: store.setModeId,
    showReasoning: store.showReasoning,
    setShowReasoning: store.setShowReasoning,
    busy,
    createNewChat,
    sendMessage,
    addUserMessage,
    addAssistantMessage,
    renameThread,
    deleteThread
  };
}

function resultPreview(stepIndex: number, modeId: OrbitModeId, prompt: string) {
  const mode = MODE_BY_ID[modeId];
  const templates = [
    {
      type: "thought" as const,
      content: `Frame the request through ${mode.label} mode.`,
      confidence: 0.74
    },
    {
      type: "action" as const,
      content: `Break down the prompt: ${prompt.slice(0, 80)}`,
      confidence: 0.7
    },
    {
      type: "observation" as const,
      content: "Search for the most useful response structure.",
      confidence: 0.82
    },
    {
      type: "conclusion" as const,
      content: "Prepare a concise final answer.",
      confidence: 0.88
    }
  ];
  return templates.slice(0, stepIndex + 1);
}

function humanizeOrbitError(error: unknown) {
  const code = error instanceof Error ? error.message : String(error || "");
  switch (code) {
    case "orbit_model_not_found":
      return "Orbit could not find the local GGUF model. Make sure Qwen3-1.7B-Q8_0.gguf is in the ai-models folder.";
    case "llama_start_timeout":
      return "Orbit took too long to start. Try again in a moment.";
    case "kernel_request_timeout":
      return "Orbit is still warming the local model and the request timed out. A second try usually works once the runtime is ready.";
    default:
      return "Orbit hit a local runtime issue before it could answer. The chat stayed intact, so we can retry right away.";
  }
}
