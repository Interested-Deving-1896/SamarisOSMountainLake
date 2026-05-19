import "./print.css";
import { PrintApp } from "./PrintApp";

export default function PrintIndex(props: { windowId: string }) {
  return <PrintApp windowId={props.windowId} />;
}
