import { Package } from "lucide-react";

export function ArchiveSidebar(props: { name: string; destination: string }) {
  return (
    <aside className="archive__sidebar">
      <div className="archive__brand">
        <div className="archive__brandGlyph">
          <Package size={18} strokeWidth={2.2} />
        </div>
        <div>
          <div className="archive__brandTitle">Archive</div>
          <div className="archive__brandMeta">{props.name}</div>
        </div>
      </div>
      <div className="archive__section">
        <div className="archive__sectionTitle">Extract to</div>
        <div className="archive__path">{props.destination}</div>
      </div>
    </aside>
  );
}
