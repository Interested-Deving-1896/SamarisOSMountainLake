import { motion, useScroll, useTransform, MotionValue, Variants } from "framer-motion";
import { ReactNode, useRef } from "react";

interface ParallaxProps {
  children: ReactNode;
  className?: string;
  speed?: number;
  direction?: "up" | "down" | "left" | "right";
  range?: [number, number];
}

export const Parallax = ({ 
  children, 
  className, 
  speed = 1, 
  direction = "up",
  range = [0, 1]
}: ParallaxProps) => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start end", "end start"]
  });

  const getTransform = () => {
    const [start, end] = range;
    const distance = 200 * speed;
    switch (direction) {
      case "up": return [start * distance, -end * distance];
      case "down": return [-start * distance, end * distance];
      case "left": return [start * distance, -end * distance];
      case "right": return [-start * distance, end * distance];
    }
  };

  const y = useTransform(scrollYProgress, [0, 1], getTransform());
  const x = direction === "left" || direction === "right" 
    ? useTransform(scrollYProgress, [0, 1], getTransform()) 
    : undefined;

  return (
    <motion.div ref={ref} style={{ y, x }} className={className}>
      {children}
    </motion.div>
  );
};

interface FadeOnScrollProps {
  children: ReactNode;
  className?: string;
  from?: number;
  to?: number;
}

export const FadeOnScroll = ({ 
  children, 
  className, 
  from = 1, 
  to = 0 
}: FadeOnScrollProps) => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start start", "end start"]
  });

  const opacity = useTransform(scrollYProgress, [0, 1], [from, to]);

  return (
    <motion.div ref={ref} style={{ opacity }} className={className}>
      {children}
    </motion.div>
  );
};

interface ScaleOnScrollProps {
  children: ReactNode;
  className?: string;
  from?: number;
  to?: number;
}

export const ScaleOnScroll = ({ 
  children, 
  className, 
  from = 1, 
  to = 1.2 
}: ScaleOnScrollProps) => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({
    target: ref,
    offset: ["start end", "end start"]
  });

  const scale = useTransform(scrollYProgress, [0, 1], [from, to]);

  return (
    <motion.div ref={ref} style={{ scale }} className={className}>
      {children}
    </motion.div>
  );
};

interface StaggerProps {
  children: ReactNode;
  className?: string;
  delay?: number;
  stagger?: number;
}

export const Stagger = ({ children, className, delay = 0, stagger = 0.05 }: StaggerProps) => {
  const variants: Variants = {
    hidden: { opacity: 0, y: 20 },
    visible: (i: number) => ({
      opacity: 1,
      y: 0,
      transition: {
        delay: delay + i * stagger,
        duration: 0.5,
        ease: [0.22, 1, 0.36, 1]
      }
    })
  };

  const childArray = Array.isArray(children) ? children : [children];
  
  return (
    <div className={className}>
      {childArray.map((child, i) => (
        <motion.div
          key={i}
          custom={i}
          initial="hidden"
          whileInView="visible"
          viewport={{ once: true, margin: "-50px" }}
          variants={variants}
        >
          {child}
        </motion.div>
      ))}
    </div>
  );
};

interface TextRevealProps {
  text: string;
  className?: string;
  tag?: "h1" | "h2" | "h3" | "p" | "span";
  delay?: number;
}

export const TextReveal = ({ 
  text, 
  className, 
  tag: Tag = "p", 
  delay = 0 
}: TextRevealProps) => {
  const words = text.split(" ");
  
  const containerVariants: Variants = {
    hidden: { opacity: 0 },
    visible: {
      opacity: 1,
      transition: { staggerChildren: 0.03, delayChildren: delay }
    }
  };

  const wordVariants: Variants = {
    hidden: { 
      opacity: 0, 
      y: 20,
      filter: "blur(4px)"
    },
    visible: {
      opacity: 1,
      y: 0,
      filter: "blur(0px)",
      transition: {
        duration: 0.4,
        ease: [0.22, 1, 0.36, 1]
      }
    }
  };

  return (
    <motion.div 
      variants={containerVariants}
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true }}
      className={className}
      style={{ display: "inline" }}
    >
      {words.map((word, i) => (
        <motion.span 
          key={i} 
          variants={wordVariants}
          style={{ display: "inline-block", marginRight: "0.25em" }}
        >
          {word}
        </motion.span>
      ))}
    </motion.div>
  );
};

interface ClipRevealProps {
  children: ReactNode;
  className?: string;
  delay?: number;
}

export const ClipReveal = ({ children, className, delay = 0 }: ClipRevealProps) => {
  const variants: Variants = {
    hidden: { clipPath: "inset(0 100% 0 0)" },
    visible: {
      clipPath: "inset(0 0% 0 0)",
      transition: {
        duration: 0.8,
        delay,
        ease: [0.22, 1, 0.36, 1]
      }
    }
  };

  return (
    <motion.div
      initial="hidden"
      whileInView="visible"
      viewport={{ once: true, margin: "-100px" }}
      variants={variants}
      className={className}
    >
      {children}
    </motion.div>
  );
};

export const useScrollProgress = () => {
  const { scrollYProgress } = useScroll();
  return scrollYProgress;
};

export const useParallax = (speed: number = 1) => {
  const { scrollYProgress } = useScroll();
  return useTransform(scrollYProgress, [0, 1], [0, -100 * speed]);
};

interface CinematicSectionProps {
  children: ReactNode;
  className?: string;
}

export const CinematicSection = ({ children, className }: CinematicSectionProps) => (
  <motion.div
    initial={{ opacity: 0 }}
    whileInView={{ opacity: 1 }}
    viewport={{ once: true, margin: "-100px" }}
    transition={{ duration: 0.8, ease: "easeOut" }}
    className={className}
  >
    {children}
  </motion.div>
);