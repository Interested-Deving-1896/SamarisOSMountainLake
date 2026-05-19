import React from "react";
import type { SamarisIconTheme, SamarisIconVariant } from "./types";

type SamarisIconThemeContextValue = {
  theme?: SamarisIconTheme;
  variant?: SamarisIconVariant;
};

const SamarisIconThemeContext = React.createContext<SamarisIconThemeContextValue>({});

export function SamarisIconProvider(props: {
  theme?: SamarisIconTheme;
  variant?: SamarisIconVariant;
  children: React.ReactNode;
}) {
  return (
    <SamarisIconThemeContext.Provider value={{ theme: props.theme, variant: props.variant }}>
      {props.children}
    </SamarisIconThemeContext.Provider>
  );
}

export function useSamarisIconProvider() {
  return React.useContext(SamarisIconThemeContext);
}

