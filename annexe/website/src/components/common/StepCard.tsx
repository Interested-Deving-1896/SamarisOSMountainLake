import { LucideIcon } from "lucide-react";
import { SpotlightCard } from "./SpotlightCard";

interface Props {
  step: number;
  icon: LucideIcon;
  title: string;
  description: string;
}

export const StepCard = ({ step, icon: Icon, title, description }: Props) => (
  <SpotlightCard className="h-full p-7">
    <span className="absolute -top-4 -right-2 font-display text-[7rem] leading-none font-black text-primary/[0.06] select-none pointer-events-none">
      {step}
    </span>
    <div className="size-11 rounded-lg glass-strong grid place-items-center mb-4 text-primary">
      <Icon className="size-5" />
    </div>
    <div className="font-mono text-[0.65rem] uppercase tracking-[0.2em] text-primary/80 mb-1">
      Step {step.toString().padStart(2, "0")}
    </div>
    <h3 className="font-display text-lg font-semibold text-foreground mb-2 tracking-tight">{title}</h3>
    <p className="text-sm text-muted-foreground leading-relaxed">{description}</p>
  </SpotlightCard>
);
