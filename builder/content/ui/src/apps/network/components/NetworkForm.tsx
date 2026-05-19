import React from "react";
import type { NetworkInterfaceModel, NetworkMode } from "../types";

export function NetworkForm(props: {
  network: NetworkInterfaceModel | null;
  saving: boolean;
  note: string | null;
  onApply: (next: Partial<NetworkInterfaceModel> & { interfaceId: string }) => void;
}) {
  const [draft, setDraft] = React.useState<NetworkInterfaceModel | null>(props.network);

  React.useEffect(() => {
    setDraft(props.network);
  }, [props.network]);

  if (!draft) {
    return <div className="network__empty">No interface selected.</div>;
  }

  return (
    <section className="network__form">
      <div className="network__formHeader">
        <div>
          <div className="network__formTitle">{draft.label}</div>
          <div className="network__formMeta">
            {draft.connected ? "Live interface detected" : "Saved configuration draft"}
          </div>
        </div>
        <button
          type="button"
          className="network__apply"
          disabled={props.saving}
          onClick={() => props.onApply(draft)}
        >
          {props.saving ? "Applying…" : "Apply"}
        </button>
      </div>

      <div className="network__grid">
        <label className="network__field">
          <span>Mode</span>
          <select
            value={draft.mode}
            onChange={(event) =>
              setDraft((current) => (current ? { ...current, mode: event.target.value as NetworkMode } : current))
            }
          >
            <option value="dhcp">DHCP</option>
            <option value="manual">Manual</option>
          </select>
        </label>

        <label className="network__field">
          <span>Interface</span>
          <input value={draft.name} disabled />
        </label>

        <label className="network__field">
          <span>IPv4 address</span>
          <input
            value={draft.address}
            disabled={draft.mode === "dhcp"}
            onChange={(event) => setDraft((current) => (current ? { ...current, address: event.target.value } : current))}
          />
        </label>

        <label className="network__field">
          <span>Subnet mask</span>
          <input
            value={draft.netmask}
            disabled={draft.mode === "dhcp"}
            onChange={(event) => setDraft((current) => (current ? { ...current, netmask: event.target.value } : current))}
          />
        </label>

        <label className="network__field">
          <span>Gateway</span>
          <input
            value={draft.gateway}
            disabled={draft.mode === "dhcp"}
            onChange={(event) => setDraft((current) => (current ? { ...current, gateway: event.target.value } : current))}
          />
        </label>

        <label className="network__field">
          <span>DNS primary</span>
          <input
            value={draft.dnsPrimary}
            onChange={(event) => setDraft((current) => (current ? { ...current, dnsPrimary: event.target.value } : current))}
          />
        </label>

        <label className="network__field">
          <span>DNS secondary</span>
          <input
            value={draft.dnsSecondary}
            onChange={(event) => setDraft((current) => (current ? { ...current, dnsSecondary: event.target.value } : current))}
          />
        </label>

        <label className="network__field">
          <span>MAC address</span>
          <input value={draft.mac} disabled />
        </label>
      </div>

      {props.note ? <div className="network__note">{props.note}</div> : null}
    </section>
  );
}
