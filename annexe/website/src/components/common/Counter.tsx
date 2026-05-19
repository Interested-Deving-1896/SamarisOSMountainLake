import { useEffect, useRef, useState } from "react";
import { useInView, useMotionValue, useSpring, useTransform } from "framer-motion";

interface Props {
  to: number;
  duration?: number;
  decimals?: number;
  suffix?: string;
  prefix?: string;
}

export const Counter = ({ to, duration = 1.6, decimals = 0, suffix = "", prefix = "" }: Props) => {
  const ref = useRef<HTMLSpanElement | null>(null);
  const inView = useInView(ref, { once: true, margin: "-80px" });
  const mv = useMotionValue(0);
  const spring = useSpring(mv, { duration: duration * 1000, bounce: 0 });
  const text = useTransform(spring, (v) =>
    `${prefix}${v.toLocaleString(undefined, { minimumFractionDigits: decimals, maximumFractionDigits: decimals })}${suffix}`,
  );
  const [val, setVal] = useState(`${prefix}0${suffix}`);

  useEffect(() => {
    if (inView) mv.set(to);
  }, [inView, to, mv]);

  useEffect(() => text.on("change", setVal), [text]);

  return <span ref={ref}>{val}</span>;
};
