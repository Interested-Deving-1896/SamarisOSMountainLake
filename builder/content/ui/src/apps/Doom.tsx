import { DoomApp } from "./doom/DoomApp";
import "./doom/doom.css";

export default function Doom(props: { windowId: string }) {
  return <DoomApp windowId={props.windowId} />;
}
