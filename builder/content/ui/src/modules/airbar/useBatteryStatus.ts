import React from "react";
import { batteryStore } from "../../system/battery/batteryStore";

export function useBatteryStatus() {
  return React.useSyncExternalStore(
    (listener) => batteryStore.subscribe(listener),
    () => batteryStore.getState()
  );
}

