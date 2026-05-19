import React from "react";
import { MoonStar, SunMedium } from "lucide-react";
import { themeStore } from "../../../system/theme/themeStore";
import { AirBarButton } from "./AirBarButton";

export const AirBarThemeToggle = React.memo(function AirBarThemeToggle(_props: { onToggleTheme?: (theme: "day" | "night") => void }) {
  const [mode, setMode] = React.useState(() => themeStore.getState());

  React.useEffect(() => themeStore.subscribe(() => setMode(themeStore.getState())), []);

  const isNight = mode === "dark";
  const ref = React.useRef<HTMLButtonElement | null>(null);

  const handleClick = React.useCallback(() => {
    const next = isNight ? "light" : "dark";
    themeStore.setMode(next);
    _props.onToggleTheme?.(next === "dark" ? "night" : "day");
  }, [isNight, _props]);

  const Icon = isNight ? SunMedium : MoonStar;

  return (
    <AirBarButton
      ref={ref}
      className="air-status"
      ariaLabel={isNight ? "Switch to day theme" : "Switch to night theme"}
      onClick={handleClick}
    >
      <span className="icon">
        <Icon size={16} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />
      </span>
    </AirBarButton>
  );
});
