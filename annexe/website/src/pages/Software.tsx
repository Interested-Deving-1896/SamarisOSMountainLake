import { useRef } from "react";
import { motion, useScroll, useTransform } from "framer-motion";
import {
  Folder, StickyNote, BrainCircuit, Mail, Music, Video, Store,
  Globe, Settings, Calculator, CloudSun, ImageIcon,
  FileText, Archive, Activity, HardDrive, Network, Bluetooth, Printer,
  ShieldCheck, Lock, Flame, EyeOff, Cpu, Zap,
  Briefcase, Film, Wifi, Wrench, Terminal, Search, User, Gamepad2, Box,
} from "lucide-react";
import { Reveal } from "@/components/common/Reveal";
import { SpotlightCard } from "@/components/common/SpotlightCard";
import { AuroraBackground } from "@/components/common/AuroraBackground";
import { Counter } from "@/components/common/Counter";
import { DotNavigation } from "@/components/common/DotNavigation";
import { FadeOnScroll } from "@/components/common/Cinematic";
import { usePageMeta } from "@/hooks/usePageMeta";

const appCategories = [
  {
    name: "Core System", icon: Settings,
    apps: [
      { icon: Folder, name: "Finder" },
      { icon: Settings, name: "Settings" },
      { icon: StickyNote, name: "Notes" },
      { icon: Terminal, name: "Terminal" },
      { icon: Store, name: "App Store" },
      { icon: Globe, name: "Peregrine Browser" },
    ],
  },
  {
    name: "Productivity", icon: Briefcase,
    apps: [
      { icon: Search, name: "Spotlight Search" },
      { icon: Mail, name: "Mail" },
      { icon: BrainCircuit, name: "Orbit AI" },
      { icon: FileText, name: "TextEditor" },
    ],
  },
  {
    name: "Media", icon: Film,
    apps: [
      { icon: Music, name: "Music" },
      { icon: ImageIcon, name: "Photos" },
      { icon: Video, name: "Videos" },
      { icon: Calculator, name: "PDF Viewer" },
    ],
  },
  {
    name: "System Tools", icon: Wrench,
    apps: [
      { icon: Activity, name: "System Monitor" },
      { icon: Activity, name: "Task Manager" },
      { icon: HardDrive, name: "Disk Utility" },
      { icon: User, name: "Session Manager" },
      { icon: Lock, name: "Lock Screen" },
      { icon: User, name: "Guest Mode" },
      { icon: Network, name: "Network" },
      { icon: ShieldCheck, name: "Firewall" },
      { icon: Lock, name: "Encryption" },
      { icon: ShieldCheck, name: "Permissions" },
      { icon: Printer, name: "Print" },
    ],
  },
  {
    name: "Compatibility", icon: Gamepad2,
    apps: [
      { icon: Gamepad2, name: "Wine Launcher" },
      { icon: Gamepad2, name: "Wine Session" },
      { icon: Gamepad2, name: "Doom" },
    ],
  },
];

const totalApps = appCategories.reduce((n, c) => n + c.apps.length, 0);

const performance: { icon: typeof Zap; label: string; value: number; suffix?: string; decimals?: number; note: string }[] = [
  { icon: Zap, label: "Boot time", value: 38, suffix: " s", note: "Tested on Mac Mini late 2012" },
  { icon: HardDrive, label: "ISO size", value: 1.9, suffix: " GB", decimals: 1, note: "Single image, full system" },
  { icon: Cpu, label: "Architecture", value: 64, suffix: "-bit", note: "x86_64 · ARM64 in progress" },
];

const archStack = [
  { layer: "BIOS / UEFI", description: "Firmware hands off boot to Samaris media." },
  { layer: "GRUB Bootloader", description: "Loads the Samaris kernel and prepares the runtime environment." },
  { layer: "Linux Kernel (x86_64)", description: "Mainline Linux kernel with Samaris-specific configuration and drivers." },
  { layer: "System services (Node.js)", description: "Powers the App Store, system daemons and the Orbit AI agent loop." },
  { layer: "Chromium rendering layer", description: "GPU-accelerated rendering surface for the entire desktop." },
  { layer: "Samaris desktop (React)", description: "The custom desktop environment — dock, AirBar, Finder, windows, login." },
];

const security = [
  { icon: Lock, title: "LUKS encryption", description: "Architecture implemented — temporarily disabled in Alpha One while hardware validation is ongoing.", status: "Implemented" },
  { icon: EyeOff, title: "No hidden telemetry", description: "No analytics platform. No advertising trackers. No user profiling under normal operation.", status: "Shipping" },
  { icon: Flame, title: "Stateful firewall", description: "Per-app outbound network controls. Architecture defined, implementation in progress.", status: "Roadmap" },
  { icon: ShieldCheck, title: "Granular permissions", description: "Per-app access to camera, microphone, files and location. Currently experimental.", status: "Roadmap" },
];

const devices = [
  { device: "x86_64 VM (VirtualBox, QEMU, UTM)", support: "Supported", note: "Primary validation environment for Alpha One" },
  { device: "Physical x86_64 PCs", support: "Experimental", note: "Real hardware testing ongoing — early results vary" },
  { device: "Intel Macs (pre-Apple Silicon)", support: "Tested", note: "Booted on Mac Mini late 2012 in 38s" },
  { device: "Apple Silicon (M1 / M2 / M3)", support: "Not supported", note: "ARM64 build in development" },
  { device: "Raspberry Pi 4 / 5", support: "Not supported", note: "ARM64 support not yet available" },
];

const Hero = () => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start start", "end start"] });
  const y = useTransform(scrollYProgress, [0, 1], [0, 120]);
  const opacity = useTransform(scrollYProgress, [0, 1], [1, 0]);

  return (
    <section ref={ref} className="relative -mt-24 min-h-screen w-full overflow-hidden flex items-center justify-center noise">
      <div aria-hidden className="absolute inset-0 bg-gradient-to-b from-background/30 via-background/40 to-background/85" />
      <FadeOnScroll from={0.5} to={0} className="absolute inset-0">
        <AuroraBackground className="opacity-50" />
      </FadeOnScroll>

      <motion.div style={{ y, opacity }} className="container relative z-10 pt-40 pb-20">
        <Reveal>
          <div className="max-w-4xl mx-auto text-center">
            <div className="font-mono text-[0.7rem] uppercase tracking-[0.25em] text-primary mb-8">
              Samaris OS · Software
            </div>
            <h1 className="font-display heading-xl">
              One ISO. <span className="text-gradient">Most modern PCs.</span>
            </h1>
            <p className="mt-8 text-lg md:text-xl text-foreground/80 max-w-2xl mx-auto leading-relaxed">
              An independent operating system, designed end to end. Here is what's inside, how it's built, and what it currently runs on.
            </p>
          </div>
        </Reveal>
      </motion.div>

      <motion.div
        initial={{ opacity: 0, y: 20 }}
        animate={{ opacity: 1, y: 0 }}
        transition={{ delay: 0.8 }}
        className="absolute bottom-8 left-1/2 -translate-x-1/2 z-10 flex flex-col items-center gap-2 text-foreground/60"
      >
        <span className="font-mono text-[0.65rem] uppercase tracking-[0.25em]">Scroll</span>
        <motion.div animate={{ y: [0, 8, 0] }} transition={{ duration: 1.5, repeat: Infinity, ease: "easeInOut" }}>
          <svg xmlns="http://www.w3.org/2000/svg" width="20" height="20" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
            <path d="M12 5v14M19 12l-7 7-7-7"/>
          </svg>
        </motion.div>
      </motion.div>
    </section>
  );
};

const PerformanceStrip = () => (
  <section id="performance" className="relative min-h-screen w-full flex items-center justify-center py-20 overflow-hidden">
    <FadeOnScroll from={0.4} to={0} className="absolute inset-0">
      <AuroraBackground className="opacity-40" />
    </FadeOnScroll>
    <div className="container relative">
      <Reveal>
        <div className="text-center mb-16">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">Performance</div>
          <h2 className="font-display text-4xl md:text-5xl font-semibold">
            Tiny footprint. <span className="text-gradient">Real ambition.</span>
          </h2>
        </div>
      </Reveal>
      <div className="grid gap-6 sm:grid-cols-3 max-w-3xl mx-auto">
        {performance.map((p, i) => (
          <Reveal key={p.label} delay={i * 0.1}>
            <SpotlightCard className="text-center p-10 h-full">
              <p.icon className="size-10 mx-auto mb-6 text-primary" />
              <div className="font-display text-5xl md:text-6xl font-bold text-gradient leading-none">
                <Counter to={p.value} suffix={p.suffix} decimals={p.decimals ?? 0} />
              </div>
              <div className="mt-4 text-lg font-medium text-foreground">{p.label}</div>
              <div className="text-sm text-muted-foreground mt-3">{p.note}</div>
            </SpotlightCard>
          </Reveal>
        ))}
      </div>
    </div>
  </section>
);

const AppsOverview = () => (
  <section id="apps" className="relative min-h-screen w-full flex items-center justify-center py-12 overflow-hidden">
    <FadeOnScroll from={0.4} to={0} className="absolute inset-0">
      <AuroraBackground className="opacity-40" />
    </FadeOnScroll>
    <div className="container relative">
      <Reveal>
        <div className="text-center mb-12">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-4">What's inside</div>
          <h2 className="font-display text-4xl md:text-5xl font-semibold">
            {totalApps} essential apps, <span className="text-gradient">organised by purpose</span>.
          </h2>
          <p className="mt-4 text-muted-foreground text-base max-w-xl mx-auto">
            A focused, opinionated selection — every app pulls its weight.
          </p>
        </div>
      </Reveal>
      <div className="space-y-10">
        {appCategories.map((cat, ci) => (
          <Reveal key={cat.name} delay={ci * 0.04}>
            <div className="relative">
              <div className="flex items-center gap-3 mb-6">
                <div className="size-10 rounded-lg bg-gradient-primary grid place-items-center shadow-glow">
                  <cat.icon className="size-4 text-primary-foreground" />
                </div>
                <h2 className="font-display text-lg font-semibold tracking-tight">{cat.name}</h2>
                <div className="flex-1 h-px bg-gradient-to-r from-border via-border to-transparent" />
                <span className="font-mono text-[0.6rem] uppercase tracking-[0.2em] text-muted-foreground">{cat.apps.length}</span>
              </div>
              <div className="grid grid-cols-3 sm:grid-cols-4 md:grid-cols-5 lg:grid-cols-6 xl:grid-cols-7 gap-3">
                {cat.apps.map((a) => (
                  <div key={a.name} className="group">
                    <div className="relative rounded-xl p-4 transition-all duration-300 hover:bg-foreground/[0.04] border border-transparent hover:border-border/50 flex flex-col items-center gap-2 min-h-[88px] justify-center">
                      <div className="size-10 rounded-xl bg-gradient-primary shadow-glow flex items-center justify-center">
                        <a.icon className="w-5 h-5 text-primary-foreground" />
                      </div>
                      <span className="text-[10px] font-medium text-foreground/80 text-center leading-tight line-clamp-2 group-hover:text-foreground transition-colors">{a.name}</span>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </Reveal>
        ))}
      </div>
    </div>
  </section>
);

const Architecture = () => (
  <section id="architecture" className="relative min-h-screen w-full flex items-center justify-center py-10 overflow-hidden">
    <FadeOnScroll from={0.3} to={0} className="absolute inset-0">
      <AuroraBackground className="opacity-35" />
    </FadeOnScroll>
    <div className="container relative">
      <Reveal>
        <div className="text-center mb-10">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-4">Technical architecture</div>
          <h2 className="font-display text-3xl md:text-4xl font-semibold">
            From firmware to <span className="text-gradient">pixels on screen</span>.
          </h2>
          <p className="mt-3 text-muted-foreground text-sm max-w-md mx-auto">
            Six layers, one cohesive system.
          </p>
        </div>
      </Reveal>
      <div className="max-w-2xl mx-auto relative">
        <div aria-hidden className="absolute left-6 top-0 bottom-0 w-px bg-gradient-to-b from-primary/60 via-secondary/40 to-transparent" />
        <div className="space-y-3">
          {archStack.map((layer, i) => (
            <Reveal key={layer.layer} delay={i * 0.06}>
              <div className="relative pl-16">
                <div className="absolute left-0 top-1 size-10 rounded-xl bg-gradient-primary grid place-items-center font-mono font-bold text-primary-foreground shadow-glow ring-2 ring-background text-sm">
                  {String(i + 1).padStart(2, "0")}
                </div>
                <SpotlightCard className="p-4">
                  <h2 className="font-display font-semibold text-base text-foreground">{layer.layer}</h2>
                  <p className="text-muted-foreground mt-1 text-sm leading-relaxed">{layer.description}</p>
                </SpotlightCard>
              </div>
            </Reveal>
          ))}
        </div>
      </div>
    </div>
  </section>
);

const SecurityPrivacy = () => (
  <section id="security" className="relative min-h-screen w-full flex items-center justify-center py-20 overflow-hidden">
    <div className="container">
      <Reveal>
        <div className="text-center mb-16">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">Security & privacy</div>
          <h2 className="font-display text-4xl md:text-5xl font-semibold">
            Privacy is <span className="text-gradient">the default</span>.
          </h2>
          <p className="mt-6 text-muted-foreground text-lg max-w-xl mx-auto">
            What ships today, and what's still on the roadmap.
          </p>
        </div>
      </Reveal>
      <div className="grid gap-6 sm:grid-cols-2 max-w-4xl mx-auto">
        {security.map((s, i) => (
          <Reveal key={s.title} delay={i * 0.1}>
            <SpotlightCard className="h-full p-8">
              <div className="flex items-center gap-6 mb-6">
                <div className={`size-14 rounded-xl grid place-items-center ring-1 shrink-0 ${
                  s.status === "Shipping" ? "bg-emerald/15 text-emerald ring-emerald/20" : "bg-secondary/15 text-secondary ring-secondary/20"
                }`}>
                  <s.icon className="size-7" />
                </div>
                <div className="flex-1">
                  <h2 className="font-display text-xl font-semibold mb-2">{s.title}</h2>
                  <span className={`font-mono text-[0.6rem] uppercase tracking-[0.2em] px-3 py-1.5 rounded-full ring-1 ${
                    s.status === "Shipping" ? "text-emerald ring-emerald/30 bg-emerald/10" : "text-secondary ring-secondary/30 bg-secondary/10"
                  }`}>
                    {s.status}
                  </span>
                </div>
              </div>
              <p className="text-muted-foreground leading-relaxed ml-20">{s.description}</p>
            </SpotlightCard>
          </Reveal>
        ))}
      </div>
    </div>
  </section>
);

const HardwareCompatibility = () => (
  <section id="compatibility" className="relative min-h-screen w-full flex items-center justify-center py-10 overflow-hidden">
    <FadeOnScroll from={0.3} to={0} className="absolute inset-0">
      <AuroraBackground className="opacity-30" />
    </FadeOnScroll>
    <div className="container">
      <Reveal>
        <div className="text-center mb-12">
          <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-4">Hardware compatibility</div>
          <h2 className="font-display text-4xl md:text-5xl font-semibold">
            Designed for <span className="text-gradient">modern x86_64 systems</span>.
          </h2>
          <p className="mt-4 text-muted-foreground text-base max-w-xl mx-auto">
            Alpha One focuses on VM validation.
          </p>
        </div>
      </Reveal>
      <Reveal className="max-w-5xl mx-auto">
        <div className="relative rounded-3xl overflow-hidden glass-strong shadow-elegant noise">
          <AuroraBackground className="opacity-25" />
          <table className="w-full text-left text-base relative">
            <thead className="text-foreground">
              <tr className="border-b border-border/60">
                <th className="px-10 py-8 font-mono text-[0.7rem] uppercase tracking-[0.2em] text-muted-foreground">Device</th>
                <th className="px-10 py-8 font-mono text-[0.7rem] uppercase tracking-[0.2em] text-muted-foreground">Support</th>
                <th className="px-10 py-8 font-mono text-[0.7rem] uppercase tracking-[0.2em] text-muted-foreground hidden lg:table-cell">Notes</th>
              </tr>
            </thead>
            <tbody>
              {devices.map((d) => (
                <tr key={d.device} className="border-t border-border/40 hover:bg-foreground/[0.02] transition-colors">
                  <td className="px-10 py-8 font-medium text-foreground">{d.device}</td>
                  <td className="px-10 py-8">
                    <span className={`inline-flex items-center gap-2 rounded-full px-4 py-2 text-sm font-semibold ${
                      d.support === "Supported"
                        ? "bg-emerald/15 text-emerald ring-1 ring-emerald/30"
                        : d.support === "Experimental"
                        ? "bg-secondary/15 text-secondary ring-1 ring-secondary/30"
                        : "bg-muted text-muted-foreground ring-1 ring-border"
                    }`}>
                      <span className="size-2 rounded-full bg-current animate-pulse" />
                      {d.support}
                    </span>
                  </td>
                  <td className="px-10 py-8 text-muted-foreground hidden lg:table-cell">{d.note}</td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </Reveal>
    </div>
  </section>
);

const Software = () => {
  usePageMeta({
    title: "Software",
    description: "Inside Samaris OS: the architecture, apps, performance stats, and hardware compatibility of the Mountain Lake Alpha One release.",
    keywords: "Samaris OS apps, operating system features, Wine compatibility, Orbit AI, local AI assistant, x86_64 architecture",
    canonicalPath: "/software",
  });
  const sections = [
    { id: "performance", label: "Performance" },
    { id: "apps", label: "Apps" },
    { id: "architecture", label: "Architecture" },
    { id: "security", label: "Security" },
    { id: "compatibility", label: "Compatibility" },
  ];

  return (
    <div>
      <main>
        <Hero />
        <PerformanceStrip />
        <AppsOverview />
        <Architecture />
        <SecurityPrivacy />
        <HardwareCompatibility />
        <DotNavigation sections={sections} />
      </main>
    </div>
  );
};

export default Software;