import { ReactNode } from "react";
import { Reveal } from "./Reveal";

interface Props {
  eyebrow?: string;
  title: ReactNode;
  description?: ReactNode;
  align?: "left" | "center";
}

export const SectionHeading = ({ eyebrow, title, description, align = "center" }: Props) => (
  <Reveal className={align === "center" ? "text-center mx-auto max-w-3xl" : "max-w-3xl"}>
    {eyebrow && (
      <div className="inline-flex items-center gap-2 rounded-full glass px-4 py-1.5 text-[0.7rem] font-mono font-medium uppercase tracking-[0.2em] text-primary mb-6">
        <span className="size-1.5 rounded-full bg-primary animate-pulse" />
        {eyebrow}
      </div>
    )}
    <h2 className="font-display heading-lg text-foreground">
      {title}
    </h2>
    {description && (
      <p className="mt-6 text-base md:text-lg text-muted-foreground leading-relaxed">
        {description}
      </p>
    )}
  </Reveal>
);
