import React from "react";
import { networkKernel, type NetworkInterface } from "../../../services/kernel/network";
import type { NetworkInterfaceModel, NetworkMode } from "../types";

function toModel(entry: NetworkInterface): NetworkInterfaceModel {
  return {
    interfaceId: entry.id,
    name: entry.name,
    label: entry.label,
    type: entry.type,
    mac: entry.mac,
    address: entry.address,
    netmask: entry.netmask,
    mode: entry.mode as NetworkMode,
    gateway: entry.gateway,
    dnsPrimary: entry.dnsPrimary,
    dnsSecondary: entry.dnsSecondary,
    connected: entry.connected,
  };
}

export function useNetworkInterfaces() {
  const [interfaces, setInterfaces] = React.useState<NetworkInterfaceModel[]>([]);
  const [loading, setLoading] = React.useState(true);
  const [saving, setSaving] = React.useState(false);
  const [note, setNote] = React.useState<string | null>(null);
  const [selectedId, setSelectedId] = React.useState<string | null>(null);

  const refresh = React.useCallback(async () => {
    setLoading(true);
    try {
      const next = await networkKernel.list();
      const mapped = next.map(toModel);
      setInterfaces(mapped);
      setSelectedId((current) => current || mapped[0]?.interfaceId || null);
    } finally {
      setLoading(false);
    }
  }, []);

  React.useEffect(() => {
    void refresh();
  }, [refresh]);

  const applyConfig = React.useCallback(
    async (draft: Partial<NetworkInterfaceModel> & { interfaceId: string }) => {
      setSaving(true);
      try {
        const response = await networkKernel.setConfig(draft);
        setInterfaces(response.interfaces.map(toModel));
        setNote(response.note);
      } finally {
        setSaving(false);
      }
    },
    []
  );

  return {
    interfaces,
    loading,
    saving,
    note,
    selectedId,
    setSelectedId,
    refresh,
    applyConfig
  };
}
