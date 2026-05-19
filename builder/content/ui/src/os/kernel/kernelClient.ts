import { eventBus } from "./eventBus";

type KernelMessage<T = unknown> = {
  type: string;
  data?: T;
  requestId?: string;
  appId?: string;
};

type KernelEventHandler<T = unknown> = (data: T) => void;

type PendingRequest = {
  resolve: (value: KernelMessage) => void;
  reject: (reason?: unknown) => void;
};

const RECONNECT_DELAY_MS = 2000;
const RECONNECT_MAX_DELAY_MS = 30000;
let reconnectAttempt = 0;
const REQUEST_TIMEOUT_MS = 8000;

function kernelUrl() {
  return "ws://localhost:9999";
}

export class KernelClient {
  private ws: WebSocket | null = null;
  private isConnected = false;
  private listeners: Record<string, KernelEventHandler[]> = {};
  private pending = new Map<string, PendingRequest>();
  private connectPromise: Promise<void> | null = null;
  private reconnectTimer: number | null = null;
  private requestSeq = 0;
  private appId = "volt.desktop";

  setAppId(id: string) { this.appId = id; }

  private withAppId<T = unknown>(payload: KernelMessage<T>): KernelMessage<T> {
    return payload.appId ? payload : { ...payload, appId: this.appId };
  }

  connect() {
    if (this.ws || this.connectPromise) return this.connectPromise ?? Promise.resolve();

    console.log("[SAMARIS] Connecting to kernel...");

    this.connectPromise = new Promise<void>((resolve, reject) => {
      const ws = new WebSocket(kernelUrl());
      this.ws = ws;

      ws.onopen = () => {
        this.isConnected = true;
        this.connectPromise = null;
        console.log("[SAMARIS] Kernel connected");
        reconnectAttempt = 0;
        this.emit("connected", true);
        eventBus.emit("kernel:connected", true);
        this.send({ type: "system.ping", data: {} });
        resolve();
      };

      ws.onmessage = (event) => {
        this.handleIncoming(event.data);
      };

      ws.onclose = () => {
        console.warn("[SAMARIS] Kernel disconnected");
        this.handleDisconnect();
      };

      ws.onerror = (err) => {
        console.error("[SAMARIS] WS Error", err);
        this.emit("error", err);
        eventBus.emit("kernel:error", err);
        if (this.connectPromise) {
          this.connectPromise = null;
          reject(new Error("kernel_connection_failed"));
        }
      };
    });

    return this.connectPromise;
  }

  connected() {
    return this.isConnected;
  }

  send(payload: KernelMessage) {
    if (!this.ws || !this.isConnected) {
      console.warn("[SAMARIS] Not connected");
      return false;
    }

    const envelope = this.withAppId(payload);
    if (import.meta.env.DEV) console.log("[SAMARIS] ->", envelope);
    this.ws.send(JSON.stringify(envelope));
    return true;
  }

  async request<T = unknown>(payload: KernelMessage, options: { timeoutMs?: number } = {}): Promise<KernelMessage<T>> {
    await this.connect();

    if (!this.ws || !this.isConnected) {
      throw new Error("kernel_not_connected");
    }

    const requestId = payload.requestId || `req-${Date.now()}-${++this.requestSeq}`;
    const envelope = this.withAppId({ ...payload, requestId });

    return new Promise<KernelMessage<T>>((resolve, reject) => {
      const timeoutId = window.setTimeout(() => {
        this.pending.delete(requestId);
        reject(new Error("kernel_request_timeout"));
      }, options.timeoutMs ?? REQUEST_TIMEOUT_MS);

      this.pending.set(requestId, {
        resolve: (value) => {
          window.clearTimeout(timeoutId);
          resolve(value as KernelMessage<T>);
        },
        reject: (reason) => {
          window.clearTimeout(timeoutId);
          reject(reason);
        }
      });

      this.send(envelope);
    });
  }

  on<T = unknown>(type: string, handler: KernelEventHandler<T>) {
    if (!this.listeners[type]) {
      this.listeners[type] = [];
    }

    this.listeners[type].push(handler as KernelEventHandler);

    return () => {
      this.listeners[type] = (this.listeners[type] || []).filter((entry) => entry !== handler);
    };
  }

  emit(type: string, data: unknown) {
    if (!this.listeners[type]) return;
    for (const handler of this.listeners[type]) {
      try {
        handler(data);
      } catch (error) {
        console.error("[SAMARIS] Listener error", error);
      }
    }
  }

  private handleIncoming(raw: unknown) {
    try {
      const message = JSON.parse(String(raw)) as KernelMessage;
      if (import.meta.env.DEV) console.log("[SAMARIS] <-", message);

      const isTerminalResponse = message.type === "error" || message.type.endsWith(".result");

      if (message.requestId && isTerminalResponse && this.pending.has(message.requestId)) {
        const pending = this.pending.get(message.requestId)!;
        this.pending.delete(message.requestId);

        if (message.type === "error") {
          pending.reject(new Error(String(message.data ?? "kernel_error")));
        } else {
          pending.resolve(message);
        }
      }

      this.emit(message.type, message.data);
      // eventBus is for cross-component events only — avoid double emission
    } catch {
      console.error("[SAMARIS] Invalid JSON", raw);
    }
  }

  private handleDisconnect() {
    this.isConnected = false;
    this.ws = null;
    this.connectPromise = null;

    for (const [, pending] of this.pending) {
      pending.reject(new Error("kernel_connection_closed"));
    }
    this.pending.clear();

    this.emit("disconnected", true);
    eventBus.emit("kernel:disconnected", true);

    if (this.reconnectTimer !== null) {
      window.clearTimeout(this.reconnectTimer);
    }

    this.reconnectTimer = window.setTimeout(() => {
      this.reconnectTimer = null;
      reconnectAttempt++;
      void this.connect();
    }, Math.min(RECONNECT_DELAY_MS * Math.pow(2, reconnectAttempt), RECONNECT_MAX_DELAY_MS));

    this.emit("disconnected", false);
    eventBus.emit("kernel:disconnected", false);
  }
}

export const kernelClient = new KernelClient();
