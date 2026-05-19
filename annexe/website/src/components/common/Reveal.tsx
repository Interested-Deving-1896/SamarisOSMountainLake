import { motion, useReducedMotion, Variants } from "framer-motion";
import { ReactNode } from "react";

interface RevealProps {
  children: ReactNode;
  delay?: number;
  className?: string;
  y?: number;
  x?: number;
  scale?: number;
  duration?: number;
  once?: boolean;
  clip?: boolean;
}

const clipVariants: Variants = {
  hidden: { clipPath: "inset(0 100% 0 0)" },
  visible: { 
    clipPath: "inset(0 0% 0 0)",
    transition: { duration: 0.8, ease: [0.22, 1, 0.36, 1] }
  }
};

export const Reveal = ({ 
  children, 
  delay = 0, 
  className, 
  y = 24, 
  x = 0,
  scale = 1,
  duration = 0.6,
  once = true,
  clip = false
}: RevealProps) => {
  const reduce = useReducedMotion();
  
  if (clip) {
    return (
      <motion.div
        initial="hidden"
        whileInView="visible"
        viewport={{ once, margin: "-80px" }}
        variants={clipVariants}
        className={className}
      >
        {children}
      </motion.div>
    );
  }

  return (
    <motion.div
      initial={reduce ? false : { opacity: 0, y, x, scale: scale === 1 ? undefined : scale }}
      whileInView={{ opacity: 1, y: 0, x: 0, scale: 1 }}
      viewport={{ once, margin: "-80px" }}
      transition={{ duration, delay, ease: [0.22, 1, 0.36, 1] }}
      className={className}
    >
      {children}
    </motion.div>
  );
};

export const RevealLeft = ({ children, delay = 0, className }: Omit<RevealProps, "x">) => (
  <Reveal children={children} delay={delay} className={className} x={-24} />
);

export const RevealRight = ({ children, delay = 0, className }: Omit<RevealProps, "x">) => (
  <Reveal children={children} delay={delay} className={className} x={24} />
);

export const RevealScale = ({ children, delay = 0, className }: Omit<RevealProps, "scale">) => (
  <Reveal children={children} delay={delay} className={className} scale={0.92} />
);

export const RevealClip = ({ children, delay = 0, className }: Omit<RevealProps, "clip">) => (
  <Reveal children={children} delay={delay} className={className} clip once />
);