import { TerminalApp } from "./terminal/TerminalApp";
import "./terminal/terminal.css";

export default function Terminal(props: { windowId: string }) {
  return <TerminalApp />;
}
