import { ButtonHTMLAttributes, forwardRef, useRef, MouseEvent } from "react";
import { cn } from "@/lib/utils";

interface Props extends ButtonHTMLAttributes<HTMLButtonElement> {
  strength?: number;
}

export const MagneticButton = forwardRef<HTMLButtonElement, Props>(
  ({ className, children, strength = 18, onMouseMove, onMouseLeave, ...rest }, ref) => {
    const localRef = useRef<HTMLButtonElement | null>(null);

    const handleMove = (e: MouseEvent<HTMLButtonElement>) => {
      const el = (ref as React.RefObject<HTMLButtonElement>)?.current ?? localRef.current;
      if (!el) return;
      const r = el.getBoundingClientRect();
      const x = e.clientX - (r.left + r.width / 2);
      const y = e.clientY - (r.top + r.height / 2);
      el.style.transform = `translate(${(x / r.width) * strength}px, ${(y / r.height) * strength}px)`;
      onMouseMove?.(e);
    };
    const handleLeave = (e: MouseEvent<HTMLButtonElement>) => {
      const el = (ref as React.RefObject<HTMLButtonElement>)?.current ?? localRef.current;
      if (el) el.style.transform = "translate(0,0)";
      onMouseLeave?.(e);
    };

    return (
      <button
        ref={(node) => {
          localRef.current = node;
          if (typeof ref === "function") ref(node);
          else if (ref) (ref as React.MutableRefObject<HTMLButtonElement | null>).current = node;
        }}
        onMouseMove={handleMove}
        onMouseLeave={handleLeave}
        className={cn("transition-transform duration-300 ease-out will-change-transform", className)}
        {...rest}
      >
        {children}
      </button>
    );
  }
);
MagneticButton.displayName = "MagneticButton";
