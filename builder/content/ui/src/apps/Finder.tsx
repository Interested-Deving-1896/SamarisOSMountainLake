import React from "react";
import { FinderWindow } from "../windows/finder/FinderWindow";

export default function Finder(props: { windowId: string }) {
  return <FinderWindow windowId={props.windowId} />;
}
