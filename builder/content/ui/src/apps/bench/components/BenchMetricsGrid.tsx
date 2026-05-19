import React from "react";
import type { BenchHardware, BenchEnvironment } from "../types/bench";

export function BenchMetricsGrid(props: {
  hardware: BenchHardware;
  environment: BenchEnvironment;
  os: { name: string; version: string; build: string; release_channel: string };
}) {
  return (
    <div className="bench__section">
      <div className="bench__sectionTitle">System Information</div>
      <div className="bench__grid">
        <div className="bench__gridItem">
          <div className="bench__gridLabel">Hardware Class</div>
          <div className="bench__gridValue">{props.hardware.class}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">Model</div>
          <div className="bench__gridValue">{props.hardware.model}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">CPU</div>
          <div className="bench__gridValue">{props.hardware.cpu} ({props.hardware.cpu_cores} cores)</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">RAM</div>
          <div className="bench__gridValue">{props.hardware.ram_gb} GB</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">GPU</div>
          <div className="bench__gridValue">{props.hardware.gpu}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">Storage</div>
          <div className="bench__gridValue">{props.hardware.storage_type}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">Architecture</div>
          <div className="bench__gridValue">{props.hardware.arch}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">OS</div>
          <div className="bench__gridValue">{props.os.name} {props.os.version}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">Channel</div>
          <div className="bench__gridValue">{props.os.release_channel}</div>
        </div>
        <div className="bench__gridItem">
          <div className="bench__gridLabel">Thermal State</div>
          <div className="bench__gridValue">{props.hardware.thermal_state}</div>
        </div>
      </div>
    </div>
  );
}
