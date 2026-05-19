import React from "react";
import { firewallKernel } from "../../services/kernel/firewall";
import { PromptModal } from "../../components/PromptModal";

export function FirewallApp(_props: { windowId: string }) {
  const [state, setState] = React.useState<Awaited<ReturnType<typeof firewallKernel.list>> | null>(null);
  const [adding, setAdding] = React.useState(false);

  const refresh = React.useCallback(async () => {
    setState(await firewallKernel.list());
  }, []);

  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  return (
    <div className="fw">
      <div className="fw__hero">
        <div>
          <div className="fw__title">Firewall</div>
          <div className="fw__subtitle">Control inbound and outbound access rules for Samaris OS.</div>
        </div>
        <label className="fw__switch">
          <span>{state?.enabled ? "Enabled" : "Disabled"}</span>
          <input
            type="checkbox"
            checked={Boolean(state?.enabled)}
            onChange={(event) => void firewallKernel.setEnabled(event.target.checked).then(setState)}
          />
        </label>
      </div>
      <div className="fw__meta">System firewall status: {state?.systemEnabled == null ? "Unknown" : state.systemEnabled ? "Active" : "Inactive"}</div>
      <div className="fw__policies">
        <button type="button" className="fw__policy" onClick={() => void firewallKernel.setPolicy("inbound", state?.inboundPolicy === "allow" ? "deny" : "allow").then(setState)}>
          Inbound: {state?.inboundPolicy || "allow"}
        </button>
        <button type="button" className="fw__policy" onClick={() => void firewallKernel.setPolicy("outbound", state?.outboundPolicy === "allow" ? "deny" : "allow").then(setState)}>
          Outbound: {state?.outboundPolicy || "allow"}
        </button>
        <button type="button" className="fw__policy fw__policy--primary" onClick={() => setAdding(true)}>
          Add rule
        </button>
      </div>
      <div className="fw__rules">
        {state?.rules.map((rule) => (
          <div key={rule.id} className="fw__rule">
            <div>
              <div className="fw__ruleTitle">{rule.label}</div>
              <div className="fw__ruleMeta">{rule.direction} • port {rule.port} • {rule.action}</div>
            </div>
            <button type="button" className="fw__remove" onClick={() => void firewallKernel.removeRule(rule.id).then(setState)}>
              Remove
            </button>
          </div>
        ))}
      </div>
      {adding ? (
        <PromptModal
          title="Add firewall rule"
          subtitle="Use label:port:direction:action, for example Web:8080:inbound:allow"
          placeholder="Web:8080:inbound:allow"
          confirmLabel="Add"
          onCancel={() => setAdding(false)}
          onConfirm={(value) => {
            setAdding(false);
            const [label, port, direction, action] = value.split(":");
            void firewallKernel
              .addRule({ label, port: Number(port), direction: (direction as "inbound" | "outbound") || "inbound", action: (action as "allow" | "deny") || "allow" })
              .then(setState);
          }}
        />
      ) : null}
    </div>
  );
}
