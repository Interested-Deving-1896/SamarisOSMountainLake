import "./permissions-manager.css";
import { PermissionsManagerApp } from "./PermissionsManagerApp";

export default function PermissionsManagerIndex(props: { windowId: string }) {
  return <PermissionsManagerApp windowId={props.windowId} />;
}
