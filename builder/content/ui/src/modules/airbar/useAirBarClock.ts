import React from "react";
import { formatTime } from "./utils/formatTime";

export function useAirBarClock() {
  const [timeText, setTimeText] = React.useState(() => formatTime(new Date()));

  React.useEffect(() => {
    const id = window.setInterval(() => {
      setTimeText(formatTime(new Date()));
    }, 1000);
    return () => window.clearInterval(id);
  }, []);

  return timeText;
}

