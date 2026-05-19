import React from "react";

export function useWindowFocus(params: {
  id: string;
  focused?: boolean;
  onFocus?: (id: string) => void;
}) {
  function handleFocus() {
    params.onFocus?.(params.id);
  }

  return {
    focused: Boolean(params.focused),
    bindFocusProps: {
      onPointerDown: handleFocus,
      onFocus: handleFocus
    }
  };
}
