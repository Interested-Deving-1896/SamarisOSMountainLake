#!/usr/bin/env node
/**
 * PTY Helper — spawned by terminal-manager.js as a child process.
 * Creates a node-pty session and communicates via JSON messages on stdin/stdout.
 */
const pty = require("node-pty");

function findShell() {
  const fs = require("fs");
  const candidates = ["/bin/bash", "/bin/zsh", "/usr/bin/bash", "/usr/bin/zsh", "bash", "zsh", "sh"];
  for (const c of candidates) {
    if (c.startsWith("/")) { try { fs.accessSync(c, fs.constants.X_OK); return c; } catch {} }
  }
  return "bash";
}

const SHELL = findShell();
const COLORS = { xterm: true, "xterm-256color": true, truecolor: true };

function send(msg) { process.stdout.write(JSON.stringify(msg) + "\n"); }
function log(msg) { process.stderr.write("[pty-helper] " + msg + "\n"); }

const sessionId = process.argv[2] || `term-${Date.now()}`;
const shell = process.env.SHELL || SHELL;
const cwd = process.env.HOME || "/tmp";

const term = pty.spawn(shell, [], {
  name: "xterm-256color",
  cols: parseInt(process.env.COLS || "80", 10),
  rows: parseInt(process.env.ROWS || "24", 10),
  cwd,
  env: { ...process.env, TERM: "xterm-256color", COLORTERM: "truecolor" },
});

send({ type: "started", sessionId, pid: term.pid, shell });

term.onData((data) => {
  send({ type: "data", sessionId, data });
});

term.onExit(({ exitCode, signal }) => {
  send({ type: "exit", sessionId, exitCode, signal });
  process.exit(0);
});

process.stdin.on("data", (chunk) => {
  const lines = chunk.toString().split("\n").filter(Boolean);
  for (const line of lines) {
    try {
      const msg = JSON.parse(line);
      if (msg.type === "resize") term.resize(msg.cols, msg.rows);
      if (msg.type === "write") term.write(msg.data);
      if (msg.type === "kill") { term.kill(); process.exit(0); }
    } catch (e) { log("parse error: " + e.message); }
  }
});
