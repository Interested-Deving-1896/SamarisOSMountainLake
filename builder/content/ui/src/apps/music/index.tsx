import "./music.css";
import { MusicApp } from "./MusicApp";

export default function MusicIndex(props: { windowId: string }) {
  return <MusicApp windowId={props.windowId} />;
}
