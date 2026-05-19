import React from "react";
import { Orbit, Sparkles } from "lucide-react";
import { ORBIT_QUICK_PROMPTS } from "../constants/prompts";
import { MessageBubble } from "./MessageBubble";
import type { OrbitMessage } from "../types";

export function ChatContainer(props: {
  messages: OrbitMessage[];
  showReasoning: boolean;
  onUsePrompt: (value: string) => void;
}) {
  const scrollerRef = React.useRef<HTMLDivElement | null>(null);

  React.useEffect(() => {
    const node = scrollerRef.current;
    if (!node) return;
    node.scrollTo({
      top: node.scrollHeight,
      behavior: "smooth"
    });
  }, [props.messages]);

  return (
    <div ref={scrollerRef} className="orbit__chat">
      {props.messages.length === 0 ? (
        <section className="orbit__emptyState">
          <div className="orbit__emptyBadge">
            <Orbit size={15} strokeWidth={2.2} />
            <span>Orbit for Samaris OS</span>
          </div>
          <h2 className="orbit__emptyTitle">Ask anything. Keep it local.</h2>
          <p className="orbit__emptyCopy">
            Orbit runs on your machine and drops straight into a fast local conversation, with no cloud dependency.
          </p>
          <div className="orbit__promptGrid">
            {ORBIT_QUICK_PROMPTS.map((prompt) => (
              <button
                key={prompt}
                type="button"
                className="orbit__promptCard"
                onClick={() => props.onUsePrompt(prompt)}
              >
                <Sparkles size={16} strokeWidth={2.2} />
                <span>{prompt}</span>
              </button>
            ))}
          </div>
        </section>
      ) : null}

      {props.messages.map((message) => (
        <MessageBubble key={message.id} message={message} showReasoning={props.showReasoning} />
      ))}
    </div>
  );
}
