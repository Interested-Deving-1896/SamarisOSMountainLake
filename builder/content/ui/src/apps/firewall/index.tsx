import "./firewall.css";
import { FirewallApp } from "./FirewallApp";

export default function FirewallIndex(props: { windowId: string }) {
  return <FirewallApp windowId={props.windowId} />;
}
