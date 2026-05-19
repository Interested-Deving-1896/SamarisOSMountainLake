import "./videos.css";
import { VideosApp } from "./VideosApp";

export default function VideosIndex(props: { windowId: string }) {
  return <VideosApp windowId={props.windowId} />;
}
