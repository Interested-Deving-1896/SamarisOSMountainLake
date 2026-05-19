import React from "react";
import { permissionsKernel, type AppPermissionEntry } from "../../services/kernel/permissions";

export function PermissionsManagerApp(_props: { windowId: string }) {
  const [entries, setEntries] = React.useState<AppPermissionEntry[]>([]);
  const [loading, setLoading] = React.useState(true);

  const refresh = React.useCallback(async () => {
    setLoading(true);
    try {
      setEntries(await permissionsKernel.listAll());
    } finally {
      setLoading(false);
    }
  }, []);

  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  return (
    <div className="perm">
      <div className="perm__hero">
        <div className="perm__title">Permissions Manager</div>
        <div className="perm__subtitle">Review the permissions granted to every Samaris system app.</div>
      </div>
      <div className="perm__list">
        {loading ? <div className="perm__empty">Loading permissions…</div> : null}
        {!loading &&
          entries.map((entry) => (
            <section key={entry.appId} className="perm__card">
              <div className="perm__cardHead">
                <div className="perm__cardTitle">{entry.appId}</div>
              </div>
              <div className="perm__rows">
                {entry.permissions.map((permission) => (
                  <label key={permission.action} className="perm__row">
                    <span>{permission.action}</span>
                    <input
                      type="checkbox"
                      checked={permission.allowed}
                      onChange={(event) => {
                        void permissionsKernel.set(entry.appId, permission.action, event.target.checked).then(refresh);
                      }}
                    />
                  </label>
                ))}
              </div>
            </section>
          ))}
      </div>
    </div>
  );
}
