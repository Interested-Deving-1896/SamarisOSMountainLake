class PermissionManager {
  private permissions: Map<string, Set<string>> = new Map();

  seed(appId: string, actions: string[]) {
    const existing = this.permissions.get(appId) || new Set();
    for (const a of actions) existing.add(a);
    this.permissions.set(appId, existing);
  }

  can(appId: string, action: string) {
    const appPerms = this.permissions.get(appId);
    if (!appPerms) return false;
    if (appPerms.has(action)) return true;
    const parts = action.split(".");
    for (let i = parts.length; i > 0; i--) {
      if (appPerms.has(parts.slice(0, i).join(".") + ".*")) return true;
    }
    return appPerms.has("*");
  }
}

export const permissionManager = new PermissionManager();
