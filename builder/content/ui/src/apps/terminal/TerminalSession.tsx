import { useEffect, useRef, useCallback } from "react";

/**
 * Single terminal session backed by Electron's node-pty via IPC.
 * When not in Electron, shows a fallback message.
 */
export function TerminalSession(props: {
  sessionId: string;
  active: boolean;
  onTitleChange: (title: string) => void;
}) {
  const containerRef = useRef<HTMLDivElement>(null);
  const xtermRef = useRef<any>(null);
  const fitAddonRef = useRef<any>(null);
  const initializedRef = useRef(false);

  const writeToPty = useCallback((data: string) => {
    if (window.electronAPI) {
      window.electronAPI.terminal.write(props.sessionId, data);
    }
  }, [props.sessionId]);

  useEffect(() => {
    if (!props.active || initializedRef.current) return;
    initializedRef.current = true;

    const loadXterm = async () => {
      try {
        const { Terminal } = await import("xterm");
        const { FitAddon } = await import("xterm-addon-fit");
        const { WebLinksAddon } = await import("xterm-addon-web-links");

        const term = new Terminal({
          cursorBlink: true,
          cursorStyle: "block",
          fontSize: 13,
          fontFamily: '"JetBrains Mono", "Fira Code", "Cascadia Code", "Menlo", monospace',
          theme: {
            background: "#0c1624",
            foreground: "#d4e4f4",
            cursor: "#48f0c8",
            selectionBackground: "rgba(72, 240, 200, 0.25)",
            black: "#1a2332",
            red: "#e06c75",
            green: "#98c379",
            yellow: "#e5c07b",
            blue: "#61afef",
            magenta: "#c678dd",
            cyan: "#56b6c2",
            white: "#abb2bf",
            brightBlack: "#3e4451",
            brightRed: "#e06c75",
            brightGreen: "#98c379",
            brightYellow: "#e5c07b",
            brightBlue: "#61afef",
            brightMagenta: "#c678dd",
            brightCyan: "#56b6c2",
            brightWhite: "#ffffff",
          },
          allowTransparency: true,
          rows: 24,
          cols: 80,
        });

        const fitAddon = new FitAddon();
        term.loadAddon(fitAddon);
        term.loadAddon(new WebLinksAddon());

        xtermRef.current = term;
        fitAddonRef.current = fitAddon;

        if (containerRef.current) {
          term.open(containerRef.current);
          setTimeout(() => fitAddon.fit(), 50);
        }

        // Terminal input → PTY
        term.onData((data: string) => {
          writeToPty(data);
        });

        // PTY output → Terminal
        if (window.electronAPI) {
          const cleanup = window.electronAPI.terminal.onData(({ id, data }) => {
            if (id === props.sessionId) {
              term.write(data);
            }
          });

          window.electronAPI.terminal.onExit(({ id }) => {
            if (id === props.sessionId) {
              term.write("\r\n\x1b[31m[process exited]\x1b[0m\r\n");
            }
          });

          // Create the PTY session
          const dims = fitAddon.proposeDimensions();
          await window.electronAPI.terminal.create(props.sessionId, {
            shell: "bash",
            cols: dims?.cols || 80,
            rows: dims?.rows || 24,
          });

          return cleanup;
        }
      } catch (err) {
        console.error("[terminal] xterm init error:", err);
      }
    };

    const cleanupPromise = loadXterm();

    return () => {
      cleanupPromise.then((cleanup) => {
        if (cleanup && typeof cleanup === "function") cleanup();
        if (window.electronAPI) window.electronAPI.terminal.kill(props.sessionId);
        if (xtermRef.current) {
          xtermRef.current.dispose();
          xtermRef.current = null;
        }
        initializedRef.current = false;
      });
    };
  }, [props.active, props.sessionId, writeToPty]);

  // Fit terminal on resize
  useEffect(() => {
    if (!props.active || !fitAddonRef.current) return;
    const onResize = () => {
      try {
        fitAddonRef.current.fit();
        const dims = fitAddonRef.current.proposeDimensions();
        if (dims && window.electronAPI) {
          window.electronAPI.terminal.resize(props.sessionId, dims.cols, dims.rows);
        }
      } catch {}
    };
    const observer = new ResizeObserver(onResize);
    if (containerRef.current) observer.observe(containerRef.current);
    window.addEventListener("resize", onResize);
    return () => {
      observer.disconnect();
      window.removeEventListener("resize", onResize);
    };
  }, [props.active, props.sessionId]);

  if (!window.electronAPI) {
    return (
      <div className="terminal__fallback">
        <div className="terminal__fallbackIcon">⌨️</div>
        <div className="terminal__fallbackTitle">Terminal Electron</div>
        <div className="terminal__fallbackText">
          Le terminal nécessite l'Electron shell avec node-pty pour exécuter bash.
        </div>
      </div>
    );
  }

  return <div className="terminal__session" ref={containerRef} />;
}
