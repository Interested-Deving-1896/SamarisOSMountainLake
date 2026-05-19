import { ReactNode, useRef } from "react";
import { motion, useScroll, useTransform } from "framer-motion";
import { AuroraBackground } from "./AuroraBackground";

interface Props {
  eyebrow: string;
  title: ReactNode;
  description?: ReactNode;
  children?: ReactNode;
}

/** Cinematic page header with parallax aurora + grid + film grain. */
export const PageHero = ({ eyebrow, title, description, children }: Props) => {
  const ref = useRef<HTMLDivElement | null>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start start", "end start"] });
  const y = useTransform(scrollYProgress, [0, 1], [0, 120]);
  const opacity = useTransform(scrollYProgress, [0, 1], [1, 0.2]);

  return (
    <section
      ref={ref}
      className="relative -mt-24 pt-40 pb-24 md:pt-48 md:pb-32 overflow-hidden noise"
    >
      <AuroraBackground className="opacity-70" />
      <div aria-hidden className="absolute inset-0 bg-grid opacity-60" />
      <div aria-hidden className="absolute inset-x-0 bottom-0 h-32 bg-gradient-to-b from-transparent to-background" />

      <motion.div style={{ y, opacity }} className="container relative z-10 text-center">
        <motion.div
          initial={{ opacity: 0, y: -8 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.6 }}
          className="inline-flex items-center gap-2 glass rounded-full px-4 py-1.5 text-[0.7rem] font-mono font-medium uppercase tracking-[0.22em] text-primary mb-7"
        >
          <span className="size-1.5 rounded-full bg-primary animate-pulse" />
          {eyebrow}
        </motion.div>

        <motion.h1
          initial={{ opacity: 0, y: 24 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.85, ease: [0.21, 0.47, 0.32, 0.98] }}
          className="font-display heading-xl mx-auto max-w-5xl"
        >
          {title}
        </motion.h1>

        {description && (
          <motion.p
            initial={{ opacity: 0, y: 18 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.15 }}
            className="mt-7 text-base md:text-xl text-muted-foreground max-w-2xl mx-auto leading-relaxed"
          >
            {description}
          </motion.p>
        )}

        {children && (
          <motion.div
            initial={{ opacity: 0, y: 16 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.3 }}
            className="mt-10"
          >
            {children}
          </motion.div>
        )}
      </motion.div>
    </section>
  );
};
