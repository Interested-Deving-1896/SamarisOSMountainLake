import React from "react";
import { SunMedium, MoonStar, Cloud, CloudRain, CloudSnow, CloudFog, CloudDrizzle, CloudLightning } from "lucide-react";

const ICONS: Record<string, React.ComponentType<any>> = {
  "clear-day": SunMedium,
  "clear-night": MoonStar,
  "cloudy": Cloud,
  "partly-cloudy-day": Cloud,
  "partly-cloudy-night": Cloud,
  "rain": CloudRain,
  "snow": CloudSnow,
  "fog": CloudFog,
  "drizzle": CloudDrizzle,
  "thunderstorm": CloudLightning,
};

export const AirBarWeather = React.memo(function AirBarWeather() {
  const [weather, setWeather] = React.useState<{ temp: string; icon: string; city: string } | null>(null);
  const [loading, setLoading] = React.useState(true);

  React.useEffect(() => {
    let cancelled = false;
    fetch("https://wttr.in/?format=j1", { mode: "cors" })
      .then((r) => r.json())
      .then((d) => {
        if (cancelled) return;
        const cc = d?.current_condition?.[0];
        const area = d?.nearest_area?.[0]?.areaName?.[0]?.value || d?.request?.[0]?.query || "";
        if (cc) {
          setWeather({
            temp: `${cc.temp_C || cc.temp_F || ""}°`,
            icon: cc.weatherDesc?.[0]?.value?.toLowerCase().replace(/\s+/g, "-") || "clear-day",
            city: area,
          });
        }
        setLoading(false);
      })
      .catch(() => { setLoading(false); });
    return () => { cancelled = true; };
  }, []);

  if (loading || !weather) return null;

  const Icon = ICONS[weather.icon] || Cloud;

  return (
    <div className="air-weather" title={`${weather.city} — ${weather.temp}`}>
      <span className="air-weather__icon">
        <Icon size={14} strokeWidth={2.2} absoluteStrokeWidth aria-hidden="true" />
      </span>
      <span className="air-weather__temp">{weather.temp}</span>
    </div>
  );
});
