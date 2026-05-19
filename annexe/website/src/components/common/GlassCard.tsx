import { ReactNode } from "react";
import { cn } from "@/lib/utils";

interface Props {
  children: ReactNode;
  className?: string;
  interactive?: boolean;
}

export const GlassCard = ({ children, className, interactive }: Props) => (
  <div
    className={cn(
      "glass rounded-2xl p-6 shadow-elegant",
      interactive && "transition-all duration-300 hover:-translate-y-1 hover:border-primary/40 hover:shadow-glow",
      className,
    )}
  >
    {children}
  </div>
);
