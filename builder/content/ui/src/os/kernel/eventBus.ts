type EventHandler<T = unknown> = (payload: T) => void;

class EventBus {
  private listeners = new Map<string, Set<EventHandler>>();

  on<T = unknown>(eventName: string, handler: EventHandler<T>) {
    const current = this.listeners.get(eventName) || new Set<EventHandler>();
    current.add(handler as EventHandler);
    this.listeners.set(eventName, current);

    return () => {
      current.delete(handler as EventHandler);
      if (current.size === 0) {
        this.listeners.delete(eventName);
      }
    };
  }

  emit<T = unknown>(eventName: string, payload: T) {
    const handlers = this.listeners.get(eventName);
    if (!handlers) return;
    for (const handler of handlers) {
      try {
        handler(payload);
      } catch (error) {
        console.error("[SAMARIS] Event bus listener error", error);
      }
    }
  }
}

export const eventBus = new EventBus();
