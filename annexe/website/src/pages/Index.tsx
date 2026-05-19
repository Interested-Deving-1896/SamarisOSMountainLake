import { Link } from "react-router-dom";
import { motion, useScroll, useTransform, useSpring } from "framer-motion";
import { useRef } from "react";
import {
  Usb, ShieldCheck, BrainCircuit, Wine, Store, Layers, Lock, CloudOff,
  Download as DownloadIcon, Zap, HardDrive, Power, ArrowRight, ChevronDown,
  Check, AlertTriangle, Compass, FileText,
} from "lucide-react";
import { Button } from "@/components/ui/button";
import { FeatureCard } from "@/components/common/FeatureCard";
import { StepCard } from "@/components/common/StepCard";
import { Reveal } from "@/components/common/Reveal";
import { Parallax, FadeOnScroll } from "@/components/common/Cinematic";
import { AuroraBackground } from "@/components/common/AuroraBackground";
import { Counter } from "@/components/common/Counter";
import { SpotlightCard } from "@/components/common/SpotlightCard";
import { DotNavigation } from "@/components/common/DotNavigation";
import heroWallpaper from "@/assets/hero-wallpaper.webp";
import samarisLogo from "@/assets/samaris-logo.webp";
import { site } from "@/config/site";

const features = [
  { icon: Usb, title: "Portable", description: "Boots straight from a USB key. Carry your full system in your pocket." },
  { icon: ShieldCheck, title: "Encryption", description: "LUKS encryption architecture implemented — temporarily disabled in Alpha One during VM-focused testing." },
  { icon: BrainCircuit, title: "Local AI", description: "An experimental local AI assistant, running entirely on-device. Lightweight, CPU-only, offline-first." },
  { icon: Wine, title: "Wine Compatibility", description: "Experiment with some Windows applications through integrated Wine." },
  { icon: Store, title: "App Store", description: "Install apps directly from curated GitHub repositories and compatible packages. No account." },
  { icon: Layers, title: "Custom Desktop", description: "A unified, GPU-accelerated desktop environment designed end to end." },
  { icon: Lock, title: "No hidden telemetry", description: "No analytics platform. No advertising trackers. No user profiling." },
  { icon: CloudOff, title: "Local-first", description: "No account required. No mandatory cloud. Your machine, your rules." },
];

const steps = [
  { icon: DownloadIcon, title: "Download", description: "Grab the latest 1.9 GB ISO from our download page." },
  { icon: HardDrive, title: "Flash", description: "Write it to any 8 GB+ USB drive with dd, Rufus or Balena Etcher." },
  { icon: Power, title: "Boot", description: "Plug in the USB and power on. Samaris loads in seconds." },
  { icon: Zap, title: "First-boot setup", description: "Pick a language, create a local session, and enter the desktop." },
];

const Hero = () => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start start", "end start"] });
  const y = useTransform(scrollYProgress, [0, 1], [0, 150]);
  const scale = useTransform(scrollYProgress, [0, 1], [1, 1.05]);
  const opacity = useTransform(scrollYProgress, [0, 0.8], [1, 0]);
  const springProgress = useSpring(scrollYProgress, { stiffness: 100, damping: 30 });

  return (
    <section ref={ref} className="relative -mt-24 min-h-screen w-full overflow-hidden flex items-center justify-center noise">
      <motion.div
        style={{ y, scale }}
        aria-hidden
        className="absolute inset-0 will-change-transform"
      >
        <motion.img
          src={heroWallpaper}
          alt=""
          loading="eager"
          decoding="async"
          fetchpriority="high"
          className="w-full h-full object-fill"
        />
      </motion.div>
      <div aria-hidden className="absolute inset-0 bg-gradient-to-b from-background/20 via-background/30 to-background/85 dark:from-background/45 dark:via-background/55 dark:to-background/95" />
      <div aria-hidden className="absolute inset-0 bg-black/20 dark:bg-black/30" />
      <Parallax speed={0.3} className="absolute inset-0">
        <AuroraBackground className="opacity-60" />
      </Parallax>

      <motion.div style={{ opacity }} className="container relative z-10 pt-24">
        <div className="max-w-5xl mx-auto text-center mt-[100px]">
          <motion.div
            initial={{ opacity: 0, scale: 0.8 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ duration: 0.8, ease: [0.22, 1, 0.36, 1] }}
            className="mb-4 -mt-[400px]"
          >
            <div className="relative inline-block">
              <div className="absolute inset-0 bg-gradient-primary rounded-full blur-2xl opacity-40 scale-110" />
              <img
                src={samarisLogo}
                alt="Samaris OS Logo"
                className="relative w-64 h-64 md:w-72 md:h-72 mx-auto drop-shadow-[0_0_40px_rgba(255,255,255,0.3)]"
              />
            </div>
          </motion.div>

          <motion.h1
            initial={{ opacity: 0, y: 40, filter: "blur(10px)" }}
            animate={{ opacity: 1, y: 0, filter: "blur(0px)" }}
            transition={{ duration: 1, delay: 0.1, ease: [0.22, 1, 0.36, 1] }}
            className="font-display heading-xl drop-shadow-sm"
          >
            {site.name}
          </motion.h1>

          <motion.p
            initial={{ opacity: 0, y: 30, filter: "blur(8px)" }}
            animate={{ opacity: 1, y: 0, filter: "blur(0px)" }}
            transition={{ duration: 0.8, delay: 0.3, ease: [0.22, 1, 0.36, 1] }}
            className="mt-4 text-lg md:text-2xl text-foreground/90 max-w-2xl mx-auto leading-relaxed font-medium"
          >
            The Native WebOS that lives in your pocket.
          </motion.p>

          <motion.div
            initial={{ opacity: 0, y: -10, scale: 0.95 }}
            animate={{ opacity: 1, y: 0, scale: 1 }}
            transition={{ duration: 0.6, ease: [0.22, 1, 0.36, 1] }}
            className="inline-flex items-center gap-2 glass rounded-full px-4 py-1.5 text-[0.7rem] font-mono font-medium uppercase tracking-[0.2em] text-foreground/80 mt-4 mb-4"
          >
            {site.download.version}
          </motion.div>

          <motion.div
            initial={{ opacity: 0, y: 30 }}
            animate={{ opacity: 1, y: 0 }}
            transition={{ duration: 0.7, delay: 0.5, ease: [0.22, 1, 0.36, 1] }}
            className="mt-10 flex flex-col sm:flex-row items-center justify-center gap-3"
          >
            <Button asChild size="lg" className="bg-gradient-primary text-primary-foreground border-0 hover:opacity-95 h-12 px-7 text-base shadow-glow rounded-full btn-shine group">
              <Link to="/download">
                <DownloadIcon className="mr-2 size-4 group-hover:scale-110 transition-transform" />
                Download Alpha
              </Link>
            </Button>
            <Button asChild size="lg" variant="outline" className="glass border-border h-12 px-7 text-base rounded-full hover:bg-foreground/5 group">
              <Link to="/software">
                Enter Samaris
                <ArrowRight className="ml-2 size-4 group-hover:translate-x-1 transition-transform" />
              </Link>
            </Button>
          </motion.div>
        </div>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ duration: 0.8, delay: 0.9 }}
        style={{ opacity }}
        className="absolute bottom-8 left-1/2 -translate-x-1/2 z-10 flex flex-col items-center gap-2 text-foreground/60"
      >
        <span className="font-mono text-[0.65rem] uppercase tracking-[0.25em]">Scroll</span>
        <motion.div
          animate={{ y: [0, 8, 0] }}
          transition={{ duration: 1.5, repeat: Infinity, ease: "easeInOut" }}
        >
          <ChevronDown className="size-5" />
        </motion.div>
      </motion.div>
    </section>
  );
};

const WhySamaris = () => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start end", "end start"] });
  const y = useTransform(scrollYProgress, [0, 1], [80, -80]);
  const opacity = useTransform(scrollYProgress, [0, 0.3, 0.7, 1], [0.3, 1, 1, 0.3]);

  return (
    <section id="vision" ref={ref} className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
      <FadeOnScroll from={0.4} to={0} className="absolute inset-0">
        <AuroraBackground className="opacity-35" />
      </FadeOnScroll>
      
      <motion.div style={{ y, opacity }} className="container relative">
        <Reveal>
          <div className="max-w-3xl mx-auto text-center">
            <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-8">
              Why we exist
            </div>
            <h2 className="font-display text-4xl md:text-5xl lg:text-6xl font-semibold mb-10 tracking-tight leading-tight">
              A quiet return to <span className="text-gradient">computing you own</span>.
            </h2>
            <div className="space-y-6 text-muted-foreground text-base md:text-lg leading-relaxed max-w-2xl mx-auto">
              <Reveal delay={0.15}>
                <p>
                  Modern computing increasingly depends on accounts, cloud synchronization, background services, and permanent connectivity. Personal computers feel less personal than they used to.
                </p>
              </Reveal>
              <Reveal delay={0.25}>
                <p>
                  Samaris OS explores a different direction — local-first, portable, and designed to stay under your control. It is an experimental operating system built around calm computing, offline workflows, and a custom desktop environment designed from the ground up.
                </p>
              </Reveal>
              <Reveal delay={0.35}>
                <p>
                  Samaris OS is an independent project — not trying to replace Windows or macOS. It is an exploration of what personal computing could still feel like when ownership, simplicity, and local control come first.
                </p>
              </Reveal>
            </div>
            <Reveal delay={0.45}>
              <div className="mt-12 flex items-center justify-center gap-2">
                <div className="size-1.5 rounded-full bg-primary" />
                <span className="text-sm text-muted-foreground">Independently developed and currently self-funded</span>
              </div>
            </Reveal>
          </div>
        </Reveal>
      </motion.div>
    </section>
  );
};

const Features = () => (
  <section id="features" className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
    <div className="container">
      <Reveal>
        <div className="text-center mb-16">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">
            What makes it different
          </div>
          <h2 className="font-display text-4xl md:text-5xl font-semibold tracking-tight">
            Everything you need. <span className="text-gradient">Nothing you don't.</span>
          </h2>
          <p className="mt-6 text-muted-foreground text-base md:text-lg leading-relaxed max-w-2xl mx-auto">
            Eight foundations that define every Samaris OS install. No accounts, no hidden telemetry, no surprises.
          </p>
        </div>
      </Reveal>
      <div className="grid gap-5 sm:grid-cols-2 lg:grid-cols-4 max-w-6xl mx-auto">
        {features.map((f, i) => (
          <motion.div
            key={f.title}
            initial={{ opacity: 0, y: 40, scale: 0.95 }}
            whileInView={{ opacity: 1, y: 0, scale: 1 }}
            viewport={{ once: true, margin: "-50px" }}
            transition={{ 
              duration: 0.5, 
              delay: i * 0.08, 
              ease: [0.22, 1, 0.36, 1] 
            }}
          >
            <FeatureCard {...f} />
          </motion.div>
        ))}
      </div>
    </div>
  </section>
);

const ByTheNumbers = () => {
  const ref = useRef<HTMLDivElement>(null);
  const containerRef = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start end", "end start"] });
  const rotate = useTransform(scrollYProgress, [0, 1], [-18, 18]);
  const y = useTransform(scrollYProgress, [0, 1], [60, -60]);
  const scale = useTransform(scrollYProgress, [0, 0.5, 1], [0.8, 1, 0.8]);

  const stats: { value: number; suffix?: string; decimals?: number; label: string; sub: string }[] = [
    { value: 1.9, suffix: " GB", decimals: 1, label: "ISO size", sub: "single image, full system" },
    { value: 38,  suffix: " s",            label: "Boot time", sub: "Mac Mini late 2012 — Tested!" },
    { value: 0,                            label: "Telemetry", sub: "no analytics, no profiling" },
  ];

  return (
    <section id="specs" ref={ref} className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
      <div className="container grid lg:grid-cols-2 gap-12 lg:gap-20 items-center">
        <motion.div 
          ref={containerRef}
          style={{ y, scale }}
          className="relative aspect-square max-w-md mx-auto w-full order-2 lg:order-1"
        >
          <div className="absolute inset-6 rounded-[2rem] bg-gradient-aurora opacity-40 blur-3xl" style={{ background: "var(--gradient-aurora)" }} />
          <motion.div
            style={{ rotate }}
            className="relative w-full h-full glass-strong rounded-[2rem] shadow-elegant grid place-items-center overflow-hidden noise"
          >
            <div aria-hidden className="absolute inset-0 bg-grid opacity-50" />
            <motion.div
              initial={{ scale: 0.8, opacity: 0 }}
              whileInView={{ scale: 1, opacity: 1 }}
              transition={{ duration: 0.6, delay: 0.2 }}
              className="relative text-center p-8"
            >
              <Usb className="size-24 mx-auto text-primary drop-shadow-[0_0_30px_hsl(var(--primary)/0.6)]" strokeWidth={1.4} />
              <div className="mt-6 font-mono text-[0.7rem] uppercase tracking-[0.25em] text-muted-foreground">Boot device</div>
            </motion.div>
          </motion.div>
        </motion.div>

        <div className="order-1 lg:order-2">
          <Reveal>
            <div className="font-mono text-[0.7rem] uppercase tracking-[0.2em] text-primary mb-6">By the numbers</div>
            <h2 className="font-display text-4xl md:text-5xl font-semibold">
              Tiny footprint. <span className="text-gradient">Real ambition.</span>
            </h2>
            <p className="mt-6 text-muted-foreground text-lg leading-relaxed max-w-xl">
              Samaris fits on any USB key, designed for modern x86_64 systems and VM-based testing. It pairs a Linux
              core with a custom desktop, integrated Wine compatibility, and a lightweight
              experimental local AI assistant.
            </p>
          </Reveal>

          <div className="mt-12 grid grid-cols-2 gap-4">
            {stats.map((s, i) => (
              <motion.div
                key={s.label}
                initial={{ opacity: 0, y: 30 }}
                whileInView={{ opacity: 1, y: 0 }}
                viewport={{ once: true }}
                transition={{ duration: 0.5, delay: i * 0.1 }}
              >
                <div className="gradient-border p-5 hover:scale-[1.02] transition-transform duration-300">
                  <div className="font-display text-3xl md:text-4xl font-bold text-gradient leading-none">
                    <Counter to={s.value} suffix={s.suffix} decimals={s.decimals ?? 0} />
                  </div>
                  <div className="mt-2 text-sm font-medium text-foreground">{s.label}</div>
                  <div className="text-xs text-muted-foreground mt-1">{s.sub}</div>
                </div>
              </motion.div>
            ))}
          </div>
        </div>
      </div>
    </section>
  );
};

const Manifesto = () => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start end", "end start"] });
  const y = useTransform(scrollYProgress, [0, 1], [50, -50]);
  const scale = useTransform(scrollYProgress, [0, 0.5, 1], [0.9, 1, 0.9]);

  return (
    <section id="manifesto" ref={ref} className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
      <motion.div style={{ y, scale }} className="absolute inset-0">
        <AuroraBackground className="opacity-40" />
      </motion.div>
      <div className="container relative">
        <Reveal>
          <p className="font-mono text-[0.7rem] uppercase tracking-[0.25em] text-primary text-center mb-10">Manifesto</p>
        </Reveal>
        <motion.h2
          initial={{ opacity: 0, y: 60, filter: "blur(20px)" }}
          whileInView={{ opacity: 1, y: 0, filter: "blur(0px)" }}
          viewport={{ once: true }}
          transition={{ duration: 1, ease: [0.22, 1, 0.36, 1] }}
          className="font-display text-center mx-auto max-w-5xl text-[clamp(2.25rem,5.5vw,5rem)] leading-[1.05] tracking-[-0.04em] font-bold"
        >
          <span className="block">Your data. Your machine.</span>
          <span className="text-stroke block">Your rules.</span>
          <span className="text-gradient block">No exceptions.</span>
        </motion.h2>
      </div>
    </section>
  );
};

const HowItWorks = () => (
  <section id="how-it-works" className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
    <div className="container">
      <Reveal>
        <div className="text-center mb-16">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">How it works</div>
          <h2 className="font-display text-4xl md:text-5xl font-semibold tracking-tight">
            From ISO to desktop in <span className="text-gradient">under 5 minutes</span>.
          </h2>
        </div>
      </Reveal>
      <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-4 max-w-5xl mx-auto">
        {steps.map((s, i) => (
          <motion.div
            key={s.title}
            initial={{ opacity: 0, y: 50, scale: 0.9 }}
            whileInView={{ opacity: 1, y: 0, scale: 1 }}
            viewport={{ once: true, margin: "-50px" }}
            transition={{ 
              duration: 0.6, 
              delay: i * 0.12, 
              ease: [0.22, 1, 0.36, 1] 
            }}
          >
            <StepCard step={i + 1} {...s} />
          </motion.div>
        ))}
      </div>
    </div>
  </section>
);

const CurrentStatus = () => {
  const works = [
    "USB boot on real hardware — tested on Mac Mini late 2012",
    "Custom desktop environment (dock, AirBar, Finder, login)",
    "Wine compatibility layer for many Windows apps",
    "LUKS encryption architecture (temporarily disabled in Alpha One)",
    "Lightweight local AI (Orbit AI, CPU-only, experimental)",
    "App Store backed by curated GitHub repositories",
  ];
  const experimental = [
    "ARM64 build (in progress, not yet released)",
    "Apple Silicon support (not ready)",
    "Real hardware compatibility (Wi-Fi/GPU still validating)",
    "VM performance varies by hypervisor)",
    "Security hardening (firewall, permissions) — evolving",
    "In-place updates (planned, currently re-flash)",
  ];

  return (
    <section id="status" className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
      <div className="container">
        <Reveal>
          <div className="text-center mb-16">
            <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">Current status</div>
            <h2 className="font-display text-4xl md:text-5xl font-semibold">
              Public Alpha One. <span className="text-gradient">Honest by default.</span>
            </h2>
            <p className="mt-6 text-muted-foreground text-base md:text-lg leading-relaxed max-w-2xl mx-auto">
              Samaris OS is a real, usable operating system in active development. Here is what works today, and what is still evolving.
            </p>
          </div>
        </Reveal>
        <div className="grid gap-6 lg:grid-cols-2 max-w-4xl mx-auto">
          <Reveal>
            <SpotlightCard className="h-full p-7">
              <div className="flex items-center gap-3 mb-5">
                <div className="size-10 rounded-xl bg-emerald/15 grid place-items-center text-emerald ring-1 ring-emerald/25">
                  <Check className="size-5" />
                </div>
                <h3 className="font-display text-xl font-semibold">What works today</h3>
              </div>
              <ul className="space-y-3">
                {works.map((w, i) => (
                  <motion.li 
                    key={w} 
                    initial={{ opacity: 0, x: -20 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    viewport={{ once: true }}
                    transition={{ delay: i * 0.05 }}
                    className="flex items-start gap-3 text-sm text-muted-foreground"
                  >
                    <span className="mt-2 size-1.5 rounded-full bg-emerald shrink-0" />
                    <span>{w}</span>
                  </motion.li>
                ))}
              </ul>
            </SpotlightCard>
          </Reveal>
          <Reveal delay={0.1}>
            <SpotlightCard className="h-full p-7">
              <div className="flex items-center gap-3 mb-5">
                <div className="size-10 rounded-xl bg-secondary/15 grid place-items-center text-secondary ring-1 ring-secondary/25">
                  <Compass className="size-5" />
                </div>
                <h3 className="font-display text-xl font-semibold">Experimental & in progress</h3>
              </div>
              <ul className="space-y-3">
                {experimental.map((e, i) => (
                  <motion.li 
                    key={e} 
                    initial={{ opacity: 0, x: 20 }}
                    whileInView={{ opacity: 1, x: 0 }}
                    viewport={{ once: true }}
                    transition={{ delay: i * 0.05 }}
                    className="flex items-start gap-3 text-sm text-muted-foreground"
                  >
                    <span className="mt-2 size-1.5 rounded-full bg-secondary shrink-0" />
                    <span>{e}</span>
                  </motion.li>
                ))}
              </ul>
            </SpotlightCard>
          </Reveal>
        </div>
      </div>
    </section>
  );
};

const KnownLimitations = () => {
  const items = [
    { title: "Hardware support is partial", desc: "Wi-Fi chipsets, GPUs and laptop sensors may need manual configuration. We focus on common, modern desktops and laptops first." },
    { title: "Apple Silicon not supported yet", desc: "M1/M2/M3 Macs are not ready. Intel Macs work; an ARM64 build is in development." },
    { title: "ARM64 still experimental", desc: "Including Raspberry Pi 4/5. Builds exist but are not yet published as a stable image." },
    { title: "Wine compatibility varies", desc: "Many Windows apps run well, some don't. Treat the Wine layer as best-effort, not a Windows replacement." },
    { title: "VM performance is inconsistent", desc: "GPU acceleration in virtualised environments depends on the hypervisor. Bare-metal USB boot remains the recommended way to try Samaris." },
    { title: "Security model is evolving", desc: "Sandboxing, granular app permissions and a stateful firewall are roadmap items, not finished features." },
    { title: "Local AI is lightweight", desc: "Orbit AI uses compact CPU-only models. It's useful and private — not a replacement for cloud-scale assistants." },
    { title: "No in-place upgrades yet", desc: "Updates currently ship as new ISO images. Re-flash to upgrade; persistence workflows are still evolving." },
  ];

  return (
    <section id="limitations" className="relative min-h-screen w-full flex items-center justify-center py-24 overflow-hidden">
      <div className="container">
        <Reveal>
          <div className="text-center mb-16">
            <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">Known limitations</div>
            <h2 className="font-display text-4xl md:text-5xl font-semibold">
              What Samaris OS <span className="text-gradient">isn't — yet</span>.
            </h2>
            <p className="mt-6 text-muted-foreground text-base md:text-lg leading-relaxed max-w-2xl mx-auto">
              Transparency matters. These are the rough edges you should know about before flashing your first USB.
            </p>
          </div>
        </Reveal>
        <div className="grid gap-4 sm:grid-cols-2 lg:grid-cols-3 max-w-5xl mx-auto">
          {items.map((it, i) => (
            <motion.div
              key={it.title}
              initial={{ opacity: 0, y: 30, scale: 0.95 }}
              whileInView={{ opacity: 1, y: 0, scale: 1 }}
              viewport={{ once: true, margin: "-30px" }}
              transition={{ 
                duration: 0.4, 
                delay: i * 0.05,
                ease: [0.22, 1, 0.36, 1] 
              }}
              whileHover={{ scale: 1.02, y: -4 }}
            >
              <SpotlightCard className="h-full p-6">
                <div className="flex items-start gap-3">
                  <div className="size-9 rounded-lg bg-secondary/10 grid place-items-center text-secondary ring-1 ring-secondary/20 shrink-0">
                    <AlertTriangle className="size-4" />
                  </div>
                  <div>
                    <h3 className="font-display font-semibold text-foreground">{it.title}</h3>
                    <p className="mt-1.5 text-sm text-muted-foreground leading-relaxed">{it.desc}</p>
                  </div>
                </div>
              </SpotlightCard>
            </motion.div>
          ))}
        </div>
      </div>
    </section>
  );
};

const Teaser = () => (
  <section id="contact" className="relative min-h-[60vh] w-full flex items-center justify-center py-24 overflow-hidden">
    <div className="container">
      <Reveal>
        <div className="relative rounded-[2rem] overflow-hidden glass-strong shadow-elegant noise">
          <div className="absolute inset-0">
            <AuroraBackground className="opacity-50" />
          </div>
          <div className="relative p-10 md:p-16 text-center">
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5 }}
              className="font-mono text-[0.7rem] uppercase tracking-[0.25em] text-primary/90 font-semibold mb-5"
            >
              {site.company}
            </motion.div>
            <motion.h3
              initial={{ opacity: 0, y: 30 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.6, delay: 0.1 }}
              className="font-display heading-lg max-w-3xl mx-auto"
            >
              Independent technology. <span className="text-gradient">Privacy by default.</span>
            </motion.h3>
            <motion.p
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: 0.2 }}
              className="mt-6 text-muted-foreground max-w-2xl mx-auto text-lg leading-relaxed"
            >
              Samaris OS is an independent project, designed and developed with a clear vision,
              and published under the {site.company} entity.
            </motion.p>
            <motion.div
              initial={{ opacity: 0, y: 20 }}
              whileInView={{ opacity: 1, y: 0 }}
              viewport={{ once: true }}
              transition={{ duration: 0.5, delay: 0.3 }}
              className="mt-10 flex flex-col sm:flex-row items-center justify-center gap-3"
            >
              <Button asChild size="lg" className="bg-gradient-primary text-primary-foreground border-0 hover:opacity-95 rounded-full h-12 px-7 btn-shine shadow-glow group">
                <Link to="/license">
                  View License <FileText className="ml-2 size-4 group-hover:scale-110 transition-transform" />
                </Link>
              </Button>
              <Button asChild size="lg" variant="outline" className="glass border-border h-12 px-7 text-base rounded-full hover:bg-foreground/5 group">
                <Link to="/business">
                  Get in touch <ArrowRight className="ml-2 size-4 group-hover:translate-x-1 transition-transform" />
                </Link>
              </Button>
            </motion.div>
          </div>
        </div>
      </Reveal>
    </div>
  </section>
);

const Index = () => {
  const sections = [
    { id: "vision", label: "Vision" },
    { id: "features", label: "Features" },
    { id: "specs", label: "Specs" },
    { id: "manifesto", label: "Manifesto" },
    { id: "how-it-works", label: "How it works" },
    { id: "status", label: "Status" },
    { id: "limitations", label: "Limitations" },
    { id: "contact", label: "Contact" },
  ];

  return (
    <main>
      <Hero />
      <WhySamaris />
      <Features />
      <ByTheNumbers />
      <Manifesto />
      <HowItWorks />
      <CurrentStatus />
      <KnownLimitations />
      <Teaser />
      <DotNavigation sections={sections} />
    </main>
  );
};

export default Index;