type WindowCloseGuard = () => boolean | Promise<boolean>;

class WindowCloseGuards {
  private guards = new Map<string, WindowCloseGuard>();

  register(windowId: string, guard: WindowCloseGuard) {
    this.guards.set(windowId, guard);
    return () => {
      const current = this.guards.get(windowId);
      if (current === guard) {
        this.guards.delete(windowId);
      }
    };
  }

  async canClose(windowId: string) {
    const guard = this.guards.get(windowId);
    if (!guard) return true;
    try {
      return await Promise.resolve(guard());
    } catch {
      return false;
    }
  }
}

export const windowCloseGuards = new WindowCloseGuards();
