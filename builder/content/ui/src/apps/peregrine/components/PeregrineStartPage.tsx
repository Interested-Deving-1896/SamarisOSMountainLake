import React from "react";
import { Globe, Sparkles } from "lucide-react";
import type { PeregrineQuickLink } from "../types";

export function PeregrineStartPage(props: {
  quickLinks: PeregrineQuickLink[];
  onOpen: (url: string) => void;
}) {
  const [input, setInput] = React.useState("");

  const handleSubmit = (e: React.FormEvent) => {
    e.preventDefault();
    if (input.trim()) props.onOpen(input.trim());
  };

  return (
    <div className="pr-start">
      <div className="pr-startLogo"><Globe size={28} /></div>
      <h1 className="pr-startTitle">Peregrine</h1>
      <p className="pr-startSub">Search the web, right from your desktop</p>

      <form className="pr-searchBox" onSubmit={handleSubmit}>
        <Globe size={16} className="pr-searchIcon" />
        <input value={input} onChange={(e) => setInput(e.target.value)} placeholder="Search or enter a URL" spellCheck={false} />
      </form>

      <div className="pr-quickLinks">
        {props.quickLinks.map((link) => (
          <button key={link.id} className="pr-quickCard" onClick={() => props.onOpen(link.url)}>
            <div className="pr-quickIcon"><Sparkles size={20} /></div>
            <span className="pr-quickLabel">{link.label}</span>
          </button>
        ))}
      </div>
    </div>
  );
}
