import "./network.css";
import { NetworkApp } from "./NetworkApp";

export default function NetworkIndex(props: { windowId: string }) {
  return <NetworkApp windowId={props.windowId} />;
}
