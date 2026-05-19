import React from "react";
import { Cpu, HardDrive, Monitor, Globe, Lightbulb, ExternalLink, User, Heart } from "lucide-react";
import "./about.css";

const nav = navigator as Navigator & { deviceMemory?: number };
const specs = [
  { icon: <Cpu size={16} />, label: "Platform", value: `${navigator.platform || "Unknown"} · ${navigator.hardwareConcurrency || "?"} cores` },
  { icon: <HardDrive size={16} />, label: "Memory", value: nav.deviceMemory ? `${nav.deviceMemory} GB` : "Unknown" },
  { icon: <Monitor size={16} />, label: "Display", value: `${window.screen?.width || 0}×${window.screen?.height || 0} @${(window.devicePixelRatio || 1).toFixed(1)}x` },
  { icon: <Globe size={16} />, label: "User Agent", value: navigator.userAgent.slice(0, 80).replace(/chrome.*?safari/i, "Chromium") + "…" },
];

const highlights = [
  { icon: "🖥️", text: "Bootable x86_64 ISO — GRUB + Linux + Plymouth" },
  { icon: "⚡", text: "Node.js native bridge — 23 system services" },
  { icon: "🎨", text: "React desktop — Glass UI, windows, dock, themes" },
  { icon: "📦", text: "28+ apps — Finder, Mail, Browser, Terminal, Orbit AI" },
  { icon: "🔒", text: "Local-first — No telemetry, no mandatory account" },
  { icon: "🍷", text: "Wine compatibility — Windows .exe support" },
];

export function AboutApp() {
  return (
    <div className="about">
      <div className="about__scroll">
        {/* Hero */}
        <div className="about__hero">
          <div className="about__logoWrap">
            <img src="brand/samaris-logo.png" alt="Samaris OS" className="about__logo" />
          </div>
          <h1 className="about__title">Samaris OS</h1>
          <div className="about__version">Mountain Lake Alpha One</div>
          <div className="about__tagline">The Native WebOS.</div>
          <div className="about__badge">
            <Lightbulb size={13} /> Public Alpha — Experimental
          </div>
        </div>

        {/* What is Samaris OS */}
        <section className="about__section">
          <h2 className="about__sectionTitle">What is Samaris OS?</h2>
          <p className="about__text">
            Samaris OS is a <strong>bootable x86_64 operating system</strong> where the desktop experience is built with
            web technologies — but runs as a first-class citizen within a real bootable OS, not inside a browser tab.
          </p>
          <p className="about__text">
            Linux underneath. Web technologies on top. Native OS behavior in between.
          </p>
        </section>

        {/* Architecture */}
        <section className="about__section">
          <h2 className="about__sectionTitle">Architecture</h2>
          <div className="about__stack">
            <div className="about__layer" style={{ zIndex: 6 }}><span>Apps</span><small>Finder · Mail · Browser · Terminal · Orbit AI · Wine</small></div>
            <div className="about__layer" style={{ zIndex: 5, background: "rgba(37,99,235,0.08)" }}><span>Desktop Shell</span><small>React · Glass UI · Window Manager · Dock · Themes</small></div>
            <div className="about__layer" style={{ zIndex: 4, background: "rgba(20,184,166,0.08)" }}><span>Electron Runtime</span><small>BrowserView · node-pty · IPC Bridge</small></div>
            <div className="about__layer" style={{ zIndex: 3, background: "rgba(139,92,246,0.08)" }}><span>Node.js Kernel</span><small>23 services · WebSocket RPC · File System · Network</small></div>
            <div className="about__layer" style={{ zIndex: 2, background: "rgba(245,158,11,0.08)" }}><span>Linux Foundation</span><small>Debian trixie · systemd · X.Org · Openbox · NetworkManager</small></div>
            <div className="about__layer" style={{ zIndex: 1, background: "rgba(239,68,68,0.08)" }}><span>Hardware</span><small>x86_64 · GRUB · Plymouth · Kernel 6.x</small></div>
          </div>
        </section>

        {/* Highlights */}
        <section className="about__section">
          <h2 className="about__sectionTitle">Key Highlights</h2>
          <div className="about__highlights">
            {highlights.map((h, i) => (
              <div key={i} className="about__highlight">
                <span className="about__hIcon">{h.icon}</span>
                <span className="about__hText">{h.text}</span>
              </div>
            ))}
          </div>
        </section>

        {/* System Specs */}
        <section className="about__section">
          <h2 className="about__sectionTitle">This Machine</h2>
          <div className="about__specs">
            {specs.map((s, i) => (
              <div key={i} className="about__spec">
                <span className="about__specIcon">{s.icon}</span>
                <div className="about__specInfo">
                  <span className="about__specLabel">{s.label}</span>
                  <span className="about__specValue">{s.value}</span>
                </div>
              </div>
            ))}
          </div>
        </section>

        {/* Credits */}
        <section className="about__section">
          <h2 className="about__sectionTitle">Credits</h2>
          <div className="about__credit">
            <User size={16} />
            <div>
              <div className="about__creditName">Khaled Ben Taieb</div>
              <div className="about__creditRole">Creator & Developer · Tunisia</div>
            </div>
            <Heart size={16} className="about__heart" />
          </div>
          <p className="about__text about__text--small" style={{ marginTop: 12 }}>
            Built with love, zero budget, and a lot of late nights. Samaris OS is an independent
            project exploring the future of personal computing — a portable, local-first, beautiful
            Native WebOS.
          </p>
        </section>

        {/* Links */}
        <section className="about__section">
          <h2 className="about__sectionTitle">Links</h2>
          <div className="about__links">
            <a className="about__link" href="https://samaris.tech" target="_blank" rel="noreferrer" onClick={(e) => { e.preventDefault(); window.electronAPI?.shell.openExternal("https://samaris.tech"); }}>
              <Globe size={14} /> Website <ExternalLink size={12} />
            </a>
            <a className="about__link" href="mailto:contact.samaris.os@gmail.com" onClick={(e) => { e.preventDefault(); window.electronAPI?.shell.openExternal("mailto:contact.samaris.os@gmail.com"); }}>
              <span role="img" aria-label="email">✉️</span> Contact <ExternalLink size={12} />
            </a>
          </div>
        </section>

        {/* Footer */}
        <div className="about__footer">
          Samaris OS Mountain Lake Alpha One
        </div>
      </div>
    </div>
  );
}
