import { NotesApp } from "./notes/NotesApp";
import "./notes/notes.css";

export default function Notes(props: { windowId: string }) {
  return <NotesApp windowId={props.windowId} />;
}
