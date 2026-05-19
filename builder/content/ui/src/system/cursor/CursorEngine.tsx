import React from "react";
import { cursorStore } from "./cursorStore";
import { themeStore } from "../theme/themeStore";

export function CursorEngine() {
  const cursor = React.useSyncExternalStore(
    (listener) => cursorStore.subscribe(listener),
    () => cursorStore.getState()
  );

  React.useEffect(() => {
    document.documentElement.dataset.cursor = cursor;
    return () => {
      delete document.documentElement.dataset.cursor;
    };
  }, [cursor]);

  React.useEffect(() => {
    if (!window.electronAPI?.cursor) return;
    const unsubscribe = themeStore.subscribe(() => {
      window.electronAPI!.cursor.setCursor(cursorStore.getState(), themeStore.getState());
    });
    window.electronAPI.cursor.setCursor(cursor, themeStore.getState());
    return unsubscribe;
  }, [cursor]);

  return null;
}
