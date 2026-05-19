import { UtilitiesApp } from "./utilities/UtilitiesApp";
import "./utilities/utilities.css";

export default function Utilities(props: { windowId: string }) {
  return <UtilitiesApp windowId={props.windowId} />;
}
