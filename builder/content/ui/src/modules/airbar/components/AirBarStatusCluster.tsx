import React from "react";
import { Volume1, Volume2, VolumeX, Wifi, WifiOff, WifiZero, WifiHigh, Bluetooth, BluetoothConnected, BluetoothOff } from "lucide-react";
import { audioStore } from "../../../system/audio/audioStore";
import { connectivityStore } from "../../../system/connectivity/connectivityStore";
import { AirBarButton } from "./AirBarButton";
import { AirBarBattery } from "./AirBarBattery";
import { AirBarSidebarButton } from "./AirBarSidebarButton";
import { useAirBar } from "../useAirBar";

function SoundIcon(props: { level: number; muted: boolean }) {
  if (props.muted || props.level === 0) return <VolumeX size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  if (props.level < 34) return <Volume1 size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  return <Volume2 size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
}

function WifiIcon(props: { wifiEnabled: boolean; currentNetworkId: string; networks: Array<{ id: string; strength: number }> }) {
  if (!props.wifiEnabled) return <WifiOff size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  if (!props.currentNetworkId) return <Wifi size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  const s = props.networks.find((n) => n.id === props.currentNetworkId)?.strength ?? 0;
  if (s > 66) return <WifiHigh size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  if (s > 33) return <Wifi size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  return <WifiZero size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
}

function BluetoothIcon(props: { btEnabled: boolean; hasConnected: boolean }) {
  if (!props.btEnabled) return <BluetoothOff size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  if (props.hasConnected) return <BluetoothConnected size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
  return <Bluetooth size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />;
}

export const AirBarStatusCluster = React.memo(function AirBarStatusCluster(props: {
  showBatteryPercentage?: boolean;
  onOpenSidebar?: () => void;
}) {
  const air = useAirBar();
  const audio = React.useSyncExternalStore((l) => audioStore.subscribe(l), () => audioStore.getState());
  const c = React.useSyncExternalStore((l) => connectivityStore.subscribe(l), () => connectivityStore.getState());

  const soundActive = air.activePanel === "sound";
  const wifiActive = air.activePanel === "wifi";
  const btActive = air.activePanel === "bluetooth";
  const soundRef = React.useRef<HTMLButtonElement | null>(null);
  const wifiRef = React.useRef<HTMLButtonElement | null>(null);
  const btRef = React.useRef<HTMLButtonElement | null>(null);

  React.useEffect(() => { air.registerAnchor("sound", soundRef.current); air.registerAnchor("wifi", wifiRef.current); air.registerAnchor("bluetooth", btRef.current); }, []);

  return (
    <>
      <AirBarButton ref={soundRef} className="air-status sound" ariaLabel="Sound" active={soundActive} ariaExpanded={soundActive} onClick={() => air.setActivePanel(soundActive ? "none" : "sound")}>
        <span className="icon"><SoundIcon muted={audio.muted} level={audio.volume} /></span>
      </AirBarButton>
      <AirBarButton ref={wifiRef} className="air-status wifi" ariaLabel="Wi-Fi" active={wifiActive} ariaExpanded={wifiActive} onClick={() => air.setActivePanel(wifiActive ? "none" : "wifi")}>
        <span className="icon"><WifiIcon wifiEnabled={c.wifiEnabled} currentNetworkId={c.currentNetworkId} networks={c.networks} /></span>
      </AirBarButton>
      <AirBarButton ref={btRef} className="air-status bt" ariaLabel="Bluetooth" active={btActive} ariaExpanded={btActive} onClick={() => air.setActivePanel(btActive ? "none" : "bluetooth")}>
        <span className="icon"><BluetoothIcon btEnabled={c.bluetoothEnabled} hasConnected={c.devices.some((d) => d.connected)} /></span>
      </AirBarButton>
      <AirBarBattery showPercentage={props.showBatteryPercentage} />
      <AirBarSidebarButton onOpenSidebar={props.onOpenSidebar} />
    </>
  );
});
