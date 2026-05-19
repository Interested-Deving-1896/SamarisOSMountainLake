import "./encryption.css";
import { EncryptionApp } from "./EncryptionApp";

export default function EncryptionIndex(props: { windowId: string }) {
  return <EncryptionApp windowId={props.windowId} />;
}
