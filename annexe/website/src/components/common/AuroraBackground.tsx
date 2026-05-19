export const AuroraBackground = ({ className = "" }: { className?: string }) => (
  <div aria-hidden className={`pointer-events-none absolute inset-0 overflow-hidden ${className}`}>
    <div
      className="aurora"
      style={{ width: 620, height: 620, top: "-10%", left: "-8%" }}
    />
    <div
      className="aurora"
      style={{ width: 520, height: 520, bottom: "-15%", right: "-5%", opacity: 0.45 }}
    />
    <div className="absolute inset-0 bg-grid opacity-40" />
  </div>
);
