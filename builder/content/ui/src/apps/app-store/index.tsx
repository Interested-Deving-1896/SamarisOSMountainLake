import "./app-store.css";
import { AppStoreApp } from "./AppStoreApp";

export default function AppStoreIndex(props: { windowId: string }) {
  return <AppStoreApp windowId={props.windowId} />;
}
