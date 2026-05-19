import { LucideIcon } from "lucide-react";
import { SpotlightCard } from "./SpotlightCard";

interface Props {
  icon: LucideIcon;
  title: string;
  description: string;
}

export const FeatureCard = ({ icon: Icon, title, description }: Props) => (
  <SpotlightCard className="h-full p-7">
    <div className="relative size-12 rounded-xl bg-gradient-primary grid place-items-center mb-5 shadow-glow">
      <Icon className="size-6 text-primary-foreground" strokeWidth={2.2} />
      <div aria-hidden className="absolute inset-0 rounded-xl bg-gradient-aurora opacity-0 blur-md transition-opacity duration-500 group-hover:opacity-60" style={{ background: "var(--gradient-aurora)" }} />
    </div>
    <h3 className="font-display text-lg font-semibold text-foreground mb-2 tracking-tight">{title}</h3>
    <p className="text-sm text-muted-foreground leading-relaxed">{description}</p>
  </SpotlightCard>
);
