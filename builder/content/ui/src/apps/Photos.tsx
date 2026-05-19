import { PhotosApp } from "./photos/PhotosApp";
import "./photos/photos.css";

export default function Photos(props: { windowId: string }) {
  return <PhotosApp windowId={props.windowId} />;
}
