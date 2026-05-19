import React from "react";
import type { AirBarContextValue, AirBarPanel, AirBarTheme } from "./types";
import { resolveTheme } from "./utils/resolveTheme";
import { themeStore } from "../../system/theme/themeStore";

const AirBarContext = React.createContext<AirBarContextValue | null>(null);

export function AirBarProvider(props: { theme: AirBarTheme; children: React.ReactNode }) {
  const [activePanel, setActivePanel] = React.useState<AirBarPanel>("none");
  const [layoutVersion, setLayoutVersion] = React.useState(0);
  const anchorsRef = React.useRef<Partial<Record<Exclude<AirBarPanel, "none">, HTMLElement | null>>>({});
  // Keep in sync with global theme when AirBar uses auto theme.
  const systemTheme = React.useSyncExternalStore(
    (listener) => themeStore.subscribe(listener),
    () => themeStore.getState()
  );
  const resolvedTheme = resolveTheme(props.theme === "auto" ? (systemTheme === "dark" ? "night" : "day") : props.theme);

  const registerAnchor = React.useCallback((panel: Exclude<AirBarPanel, "none">, element: HTMLElement | null) => {
    anchorsRef.current[panel] = element;
  }, []);

  const getPanelStyle = React.useCallback<AirBarContextValue["getPanelStyle"]>(
    (panel, options) => {
      const width = options?.width ?? 340;
      const gap = options?.gap ?? 10;
      const align = options?.align ?? "center";
      const anchor = anchorsRef.current[panel];
      const viewportWidth = typeof window === "undefined" ? width + 28 : window.innerWidth;
      const safeMaxWidth = Math.max(280, options?.maxWidth ?? viewportWidth - 28);
      const panelWidth = Math.min(width, safeMaxWidth);
      const fallbackLeft = 14;
      const fallbackTop = 66;
      const rect = anchor?.getBoundingClientRect();

      let left = fallbackLeft;
      let top = fallbackTop;
      let arrowLeft = 28;

      if (rect) {
        top = Math.round(rect.bottom + gap);
        if (align === "start") {
          left = rect.left;
        } else if (align === "end") {
          left = rect.right - panelWidth;
        } else {
          left = rect.left + rect.width * 0.5 - panelWidth * 0.5;
        }
      }

      left = Math.max(14, Math.min(left, viewportWidth - panelWidth - 14));

      if (rect) {
        arrowLeft = Math.max(24, Math.min(rect.left + rect.width * 0.5 - left, panelWidth - 24));
      }

      return {
        ["--air-panel-left" as string]: `${Math.round(left)}px`,
        ["--air-panel-top" as string]: `${Math.round(top)}px`,
        ["--air-panel-width" as string]: `${Math.round(panelWidth)}px`,
        ["--air-panel-arrow-left" as string]: `${Math.round(arrowLeft)}px`
      } as React.CSSProperties;
    },
    []
  );

  React.useEffect(() => {
    if (activePanel === "none") return;
    const sync = () => setLayoutVersion((value) => value + 1);
    window.addEventListener("resize", sync);
    window.addEventListener("scroll", sync, true);
    return () => {
      window.removeEventListener("resize", sync);
      window.removeEventListener("scroll", sync, true);
    };
  }, [activePanel]);

  const value = React.useMemo<AirBarContextValue>(() => {
    return {
      theme: props.theme,
      resolvedTheme,
      activePanel,
      setActivePanel,
      closePanels: () => setActivePanel("none"),
      registerAnchor,
      getPanelStyle
    };
  }, [activePanel, props.theme, resolvedTheme, registerAnchor, getPanelStyle, layoutVersion]);

  return <AirBarContext.Provider value={value}>{props.children}</AirBarContext.Provider>;
}

export function useAirBarContext() {
  const ctx = React.useContext(AirBarContext);
  if (!ctx) throw new Error("useAirBar must be used within <AirBarProvider />");
  return ctx;
}
