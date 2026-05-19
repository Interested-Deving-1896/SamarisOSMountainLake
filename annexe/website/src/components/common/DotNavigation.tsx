import { useCallback, useEffect, useRef, useState } from "react";
import { motion } from "framer-motion";

interface DotNavigationProps {
  sections: { id: string; label: string }[];
}

export const DotNavigation = ({ sections }: DotNavigationProps) => {
  const [activeSection, setActiveSection] = useState(0);
  const rafRef = useRef<number>(0);

  const onScroll = useCallback(() => {
    if (rafRef.current) return;
    rafRef.current = requestAnimationFrame(() => {
      rafRef.current = 0;
      const scrolled = window.scrollY;
      const sectionOffsets = sections.map(({ id }) => {
        const el = document.getElementById(id);
        return el?.offsetTop || 0;
      });
      const midpoint = scrolled + window.innerHeight * 0.4;
      let active = 0;
      for (let i = 0; i < sectionOffsets.length; i++) {
        if (midpoint >= sectionOffsets[i]) active = i;
      }
      setActiveSection(active);
    });
  }, [sections]);

  useEffect(() => {
    onScroll();
    window.addEventListener("scroll", onScroll, { passive: true });
    return () => {
      window.removeEventListener("scroll", onScroll);
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
    };
  }, [onScroll]);

  const [isVisible, setIsVisible] = useState(false);
  const [ready, setReady] = useState(false);

  useEffect(() => {
    setReady(true);
    const onVisible = () => setIsVisible(window.scrollY > window.innerHeight * 0.3);
    onVisible();
    window.addEventListener("scroll", onVisible, { passive: true });
    return () => window.removeEventListener("scroll", onVisible);
  }, []);

  if (!ready || !isVisible) return null;

  const scrollTo = (index: number) => {
    const el = document.getElementById(sections[index].id);
    el?.scrollIntoView({ behavior: "smooth" });
  };

  return (
    <motion.div
      initial={{ opacity: 0, x: 20 }}
      animate={{ opacity: 1, x: 0 }}
      className="fixed right-4 lg:right-6 top-1/2 -translate-y-1/2 z-40 flex flex-col items-center gap-4"
    >
      {sections.map((section, index) => (
        <button
          key={section.id}
          onClick={() => scrollTo(index)}
          className="group relative flex items-center cursor-pointer"
          aria-label={section.label}
        >
          <span className="absolute right-6 opacity-0 group-hover:opacity-100 transition-opacity text-xs text-muted-foreground whitespace-nowrap">
            {section.label}
          </span>
          <div
            className={`w-2 h-2 rounded-full transition-all duration-300 ${
              index === activeSection
                ? "bg-primary shadow-[0_0_10px_hsl(200,95%,48%)]"
                : "bg-foreground/30 group-hover:bg-foreground/50"
            }`}
          />
        </button>
      ))}
    </motion.div>
  );
};