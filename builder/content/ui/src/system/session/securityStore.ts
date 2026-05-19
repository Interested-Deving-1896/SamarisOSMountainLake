import { sessionSecurityKernel, type SessionSecurityState } from "../../services/kernel/sessionSecurity";

type Listener = () => void;

const DEFAULT_STATE: SessionSecurityState = {
  guestMode: false,
  lockAfterMinutes: 10,
  locked: true,
  passwordHint: "",
  displayName: "Samaris User",
  username: "user",
  hasPassword: false,
};

class SecurityStore {
  private state: SessionSecurityState = DEFAULT_STATE;
  private listeners = new Set<Listener>();
  private initialized = false;
  private idleTimer: number | null = null;
  private idleStartedAt = Date.now();

  init() {
    if (this.initialized) return;
    this.initialized = true;
    void this.refresh();
    const reset = () => {
      this.idleStartedAt = Date.now();
    };
    for (const name of ["pointerdown", "keydown", "mousemove"]) {
      window.addEventListener(name, reset, { passive: true });
    }
    this.idleTimer = window.setInterval(() => {
      const minutes = this.state.lockAfterMinutes || 0;
      if (!minutes || this.state.locked) return;
      if (Date.now() - this.idleStartedAt >= minutes * 60_000) {
        void this.lock();
      }
    }, 15_000);
  }

  getState() {
    return this.state;
  }

  subscribe(listener: Listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  }

  private patch(next: SessionSecurityState) {
    this.state = next;
    for (const listener of this.listeners) listener();
  }

  async refresh() {
    try {
      this.patch(await sessionSecurityKernel.get());
    } catch {}
  }

  async set(payload: Partial<SessionSecurityState>) {
    this.patch(await sessionSecurityKernel.set(payload));
  }

  async lock() {
    this.patch(await sessionSecurityKernel.lock());
  }

  async unlock(password: string, username?: string) {
    const next = await sessionSecurityKernel.unlock(password, username);
    this.patch({
      ...this.state,
      ...next
    });
    return next;
  }
}

export const securityStore = new SecurityStore();
