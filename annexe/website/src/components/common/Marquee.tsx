import { ReactNode } from "react";

interface Props {
  children: ReactNode[];
  speed?: number; // seconds per loop
}

/** Infinite horizontal marquee — duplicates children for a seamless loop. */
export const Marquee = ({ children, speed = 40 }: Props) => (
  <div className="marquee-pause overflow-hidden [mask-image:linear-gradient(90deg,transparent,black_10%,black_90%,transparent)]">
    <div className="marquee" style={{ animationDuration: `${speed}s` }}>
      {[...children, ...children].map((c, i) => (
        <div key={i} className="shrink-0">{c}</div>
      ))}
    </div>
  </div>
);
