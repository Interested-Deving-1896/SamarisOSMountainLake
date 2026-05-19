import React from "react";
import { AirBarPill } from "./AirBarPill";

export const AirBarDateTime = React.memo(function AirBarDateTime() {
  const [now, setNow] = React.useState(new Date());
  React.useEffect(() => { const t = setInterval(() => setNow(new Date()), 10000); return () => clearInterval(t); }, []);

  const time = now.toLocaleTimeString([], { hour: "2-digit", minute: "2-digit" });
  const date = now.toLocaleDateString([], { weekday: "short", month: "short", day: "numeric" });

  return (
    <div className="air-datetime">
      <span className="air-datetime__time">{time}</span>
      <span className="air-datetime__date">{date}</span>
    </div>
  );
});
