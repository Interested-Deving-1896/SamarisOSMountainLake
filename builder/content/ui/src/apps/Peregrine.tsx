import { PeregrineApp } from "./peregrine/PeregrineApp";
import "./peregrine/peregrine.css";

export default function Peregrine(props: { windowId: string }) {
  return <PeregrineApp windowId={props.windowId} />;
}
