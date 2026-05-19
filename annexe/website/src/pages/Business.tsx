import { Mail, MapPin, Github, Youtube, Check, Copy, Users, Cpu, GraduationCap, Building2, ArrowRight, Handshake, Globe, Sparkles, Target } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Reveal } from "@/components/common/Reveal";
import { PageHero } from "@/components/common/PageHero";
import { SpotlightCard } from "@/components/common/SpotlightCard";
import { AuroraBackground } from "@/components/common/AuroraBackground";
import { SectionHeading } from "@/components/common/SectionHeading";
import { site } from "@/config/site";
import { usePageMeta } from "@/hooks/usePageMeta";

const opportunities = [
  {
    icon: Building2,
    title: "OEM Discussions",
    description: "Pre-install Samaris OS on your hardware. Exploratory conversations about custom builds for laptops, mini-PCs, and embedded systems.",
  },
  {
    icon: GraduationCap,
    title: "Education & Research",
    description: "University programs, coding bootcamps, and research institutions. Early-stage discussions about educational licensing and privacy-focused learning environments.",
  },
  {
    icon: Globe,
    title: "Platform Collaborations",
    description: "Looking to explore privacy-first OS integration? We offer experimental collaboration frameworks and development discussions.",
  },
  {
    icon: Handshake,
    title: "Technology Partners",
    description: "Integration partnerships, co-development opportunities, and joint experiments. Samaris OS is open to collaborations that advance local-first computing.",
  },
];

const benefits = [
  "Custom builds and branding discussions",
  "Exploratory support channels",
  "Source access for modifications",
  "Early feature access",
  "Co-development opportunities",
  "Flexible collaboration terms",
];

const Business = () => {
  usePageMeta({
    title: "Business",
    description: "Business and partnership inquiries for Samaris OS — OEM discussions, educational licensing, technology collaborations, and strategic partnerships.",
    keywords: "Samaris OS partnership, OEM discussion, education license, technology collaboration",
    canonicalPath: "/business",
  });
  const [copied, setCopied] = useState(false);

  const copyEmail = async () => {
    await navigator.clipboard.writeText(site.email);
    setCopied(true);
    setTimeout(() => setCopied(false), 2000);
  };

  return (
    <div>
      <main>
        <PageHero
        eyebrow="Business Inquiries"
        title={<>Let's build something <span className="text-gradient">together</span>.</>}
        description="Samaris OS is seeking early-stage collaboration, research partnerships, and future hardware discussions. Every inquiry is read personally by the founder."
      />

      <div className="container -mt-12 mb-24">
        {/* Main CTA */}
        <Reveal className="mb-16">
          <div className="relative">
            <div aria-hidden className="absolute -inset-2 bg-gradient-aurora opacity-30 blur-3xl rounded-[3rem]" style={{ background: "var(--gradient-aurora)" }} />
            <div className="relative glass-strong rounded-3xl p-10 md:p-14 shadow-elegant noise overflow-hidden">
              <AuroraBackground className="opacity-25" />
              <div className="relative text-center max-w-2xl mx-auto">
                <div className="size-16 rounded-2xl bg-gradient-primary grid place-items-center shadow-glow mx-auto mb-6">
                  <Mail className="size-8 text-primary-foreground" />
                </div>
                <h2 className="font-display text-3xl md:text-4xl font-semibold mb-4 tracking-tight">
                  Start a conversation
                </h2>
                <p className="text-muted-foreground text-lg mb-8 leading-relaxed">
                  Whether you're a hardware maker, educational institution, or potential collaborator — 
                  we want to hear from you. Every message is read and replied to personally.
                </p>
                <div className="flex flex-col sm:flex-row items-center justify-center gap-4">
                  <a
                    href={`mailto:${site.email}`}
                    className="inline-flex items-center gap-3 bg-gradient-primary text-primary-foreground hover:opacity-95 h-14 px-8 text-base rounded-full btn-shine shadow-glow font-medium"
                  >
                    <Mail className="size-5" />
                    {site.email}
                  </a>
                  <Button size="lg" variant="outline" onClick={copyEmail} className="glass border-border h-14 px-8 text-base rounded-full hover:bg-foreground/5">
                    {copied ? <Check className="mr-2 size-5 text-emerald" /> : <Copy className="mr-2 size-5" />}
                    {copied ? "Copied!" : "Copy email"}
                  </Button>
                </div>
              </div>
            </div>
          </div>
        </Reveal>

        {/* Opportunities Grid */}
        <SectionHeading
          eyebrow="Opportunities"
          title={<>What we're <span className="text-gradient">looking for</span>.</>}
          description="Samaris OS is more than a project — it's a vision for sovereign computing. We're building partnerships that make this vision a reality."
        />
        
        <div className="mt-12 grid gap-6 md:grid-cols-2">
          {opportunities.map((item, i) => (
            <Reveal key={item.title} delay={i * 0.08}>
              <SpotlightCard className="h-full p-8">
                <div className="flex items-start gap-5">
                  <div className="size-12 rounded-xl bg-gradient-primary grid place-items-center shadow-glow shrink-0">
                    <item.icon className="size-5 text-primary-foreground" />
                  </div>
                  <div>
                    <h3 className="font-display text-xl font-semibold text-foreground mb-3">{item.title}</h3>
                    <p className="text-muted-foreground leading-relaxed">{item.description}</p>
                  </div>
                </div>
              </SpotlightCard>
            </Reveal>
          ))}
        </div>

        {/* Strategic Discussions */}
        <Reveal>
          <div className="mt-16 relative">
            <div aria-hidden className="absolute -inset-2 bg-gradient-aurora opacity-20 blur-3xl rounded-[2.5rem]" style={{ background: "var(--gradient-aurora)" }} />
            <div className="relative glass-strong rounded-3xl p-12 md:p-16 overflow-hidden">
              <AuroraBackground className="opacity-20" />
              <div className="relative text-center max-w-xl mx-auto">
                <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">Vision</div>
                <h3 className="font-display text-2xl md:text-3xl font-semibold mb-6 tracking-tight">
                  Strategic discussions
                </h3>
                <p className="text-muted-foreground text-base leading-relaxed mb-2">
                  Samaris OS is an independent long-term operating system project exploring local-first and sovereign computing.
                </p>
                <p className="text-muted-foreground text-base leading-relaxed mb-8">
                  We are open to thoughtful discussions around research partnerships, infrastructure support, hardware collaborations, long-term platform development, and aligned long-term support and strategic investment.
                </p>
                <div className="inline-flex items-center gap-2 text-sm text-muted-foreground">
                  <div className="size-1.5 rounded-full bg-primary" />
                  <span>Early-stage discussions handled directly by the founder</span>
                </div>
              </div>
            </div>
          </div>
        </Reveal>

        {/* Benefits */}
        <Reveal delay={0.1}>
          <div className="mt-16 glass-strong rounded-3xl p-10 md:p-12">
            <div className="grid md:grid-cols-2 gap-12 items-center">
              <div>
                <div className="font-mono text-[0.7rem] uppercase tracking-[0.22em] text-primary mb-4">Why collaborate</div>
                <h2 className="font-display text-2xl md:text-3xl font-semibold mb-6">
                  More than a project — a <span className="text-gradient">community</span>.
                </h2>
                <p className="text-muted-foreground leading-relaxed">
                  We believe in building something meaningful together. 
                  Your input shapes the future of Samaris OS. Every collaboration is a step toward better local-first computing.
                </p>
                <div className="flex items-center gap-3 mt-6">
                  <a href={site.social.github} target="_blank" rel="noreferrer" className="size-10 rounded-xl glass grid place-items-center text-muted-foreground hover:text-primary transition-all hover:-translate-y-0.5">
                    <Github className="size-5" />
                  </a>
                  <a href={site.social.youtube} target="_blank" rel="noreferrer" className="size-10 rounded-xl glass grid place-items-center text-muted-foreground hover:text-primary transition-all hover:-translate-y-0.5">
                    <Youtube className="size-5" />
                  </a>
                </div>
              </div>
              <div className="grid grid-cols-2 gap-4">
                {benefits.map((benefit) => (
                  <div key={benefit} className="flex items-center gap-3">
                    <div className="size-2 rounded-full bg-primary shrink-0" />
                    <span className="text-sm text-foreground">{benefit}</span>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </Reveal>

        {/* Contact Info */}
        <Reveal delay={0.15}>
          <div className="mt-12 grid gap-6 md:grid-cols-2">
            <SpotlightCard className="p-8">
              <div className="flex items-center gap-3 mb-4">
                <div className="size-10 rounded-xl bg-gradient-primary grid place-items-center shadow-glow">
                  <Globe className="size-5 text-primary-foreground" />
                </div>
                <h2 className="font-display font-semibold text-lg">Global Reach</h2>
              </div>
              <p className="text-muted-foreground leading-relaxed">
                Remote-first operation with a global vision. 
                Independently operated — no middlemen, no gatekeepers.
              </p>
            </SpotlightCard>

            <SpotlightCard className="p-8">
              <div className="flex items-center gap-3 mb-4">
                <div className="size-10 rounded-xl bg-gradient-primary grid place-items-center shadow-glow">
                  <Sparkles className="size-5 text-primary-foreground" />
                </div>
                <h2 className="font-display font-semibold text-lg">Independent project</h2>
              </div>
              <p className="text-muted-foreground leading-relaxed">
                Independently developed and currently self-funded. Decisions are made directly by the team. 
                Your inquiry goes straight to the people who build Samaris OS.
              </p>
            </SpotlightCard>
          </div>
        </Reveal>

        {/* Bottom CTA */}
        <Reveal delay={0.2}>
          <div className="mt-16 text-center">
            <p className="text-muted-foreground mb-6">
              Not sure if your idea is a good fit? Reach out anyway — we read every message.
            </p>
            <a
              href={`mailto:${site.email}?subject=Partnership Inquiry`}
              className="inline-flex items-center gap-2 text-primary hover:underline font-medium"
            >
              Send us a message
              <ArrowRight className="size-4" />
            </a>
          </div>
        </Reveal>
      </div>
      </main>
    </div>
  );
};

export default Business;