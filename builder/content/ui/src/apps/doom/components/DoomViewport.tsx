export function DoomViewport(props: {
  containerRef: React.RefObject<HTMLDivElement>;
  status: "idle" | "loading" | "ready" | "error";
  error: string | null;
}) {
  return (
    <div className="doom__viewportShell">
      <div className="doom__viewport" ref={props.containerRef} />
      {props.status === "loading" ? <div className="doom__overlay">Booting DOOM…</div> : null}
      {props.status === "error" ? <div className="doom__overlay doom__overlay--error">{props.error || "Unable to start DOOM."}</div> : null}
    </div>
  );
}
