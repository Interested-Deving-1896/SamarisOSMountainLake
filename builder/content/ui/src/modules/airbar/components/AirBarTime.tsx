import React from "react";
import { useAirBarClock } from "../useAirBarClock";
import { AirBarPill } from "./AirBarPill";

export const AirBarTime = React.memo(function AirBarTime() {
  const timeText = useAirBarClock();
  return (
    <AirBarPill className="air-pill air-time" title="Time" ariaLabel="Time">
      {timeText}
    </AirBarPill>
  );
});

