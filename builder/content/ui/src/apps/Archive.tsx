import { ArchiveApp } from "./archive/ArchiveApp";
import "./archive/archive.css";

export default function Archive(props: { windowId: string }) {
  return <ArchiveApp windowId={props.windowId} />;
}
