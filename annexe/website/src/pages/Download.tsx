import { useRef } from "react";
import { motion, useScroll, useTransform } from "framer-motion";
import { Download as DownloadIcon, ShieldCheck, FileText, Cpu, MemoryStick, Usb, Check, Copy, Sparkles } from "lucide-react";
import { useState } from "react";
import { Button } from "@/components/ui/button";
import { Tabs, TabsContent, TabsList, TabsTrigger } from "@/components/ui/tabs";
import { CodeBlock } from "@/components/common/CodeBlock";
import { Reveal } from "@/components/common/Reveal";
import { SpotlightCard } from "@/components/common/SpotlightCard";
import { AuroraBackground } from "@/components/common/AuroraBackground";
import { FadeOnScroll } from "@/components/common/Cinematic";
import { site } from "@/config/site";
import { usePageMeta } from "@/hooks/usePageMeta";

const reqs = [
  { icon: Usb, label: "USB drive", value: "8 GB or larger" },
  { icon: MemoryStick, label: "RAM", value: "2 GB minimum" },
  { icon: Cpu, label: "CPU", value: "x86_64 (ARM64 coming soon)" },
];

const Hero = () => {
  const ref = useRef<HTMLDivElement>(null);
  const { scrollYProgress } = useScroll({ target: ref, offset: ["start start", "end start"] });
  const y = useTransform(scrollYProgress, [0, 1], [0, 120]);
  const opacity = useTransform(scrollYProgress, [0, 1], [1, 0]);

  const fileName = `Samaris-OS-Mountain-Lake-Alpha-One.iso`;

  return (
    <section ref={ref} className="relative -mt-24 min-h-screen w-full overflow-hidden flex items-center justify-center noise">
      <div aria-hidden className="absolute inset-0 bg-gradient-to-b from-background/30 via-background/40 to-background/85" />
      <FadeOnScroll from={0.5} to={0} className="absolute inset-0">
        <AuroraBackground className="opacity-50" />
      </FadeOnScroll>

      <motion.div style={{ y, opacity }} className="container relative z-10 pt-40 pb-20">
        <Reveal>
          <div className="max-w-4xl mx-auto text-center mb-12">
            <div className="font-mono text-[0.7rem] uppercase tracking-[0.25em] text-primary mb-6">
              {site.download.version}
            </div>
            <h1 className="font-display heading-xl">
              Get <span className="text-gradient">Samaris OS</span>.
            </h1>
            <p className="mt-6 text-lg text-foreground/80 max-w-xl mx-auto">
              One ISO. x86_64 experimental. Free, no account, no telemetry.
            </p>
          </div>
        </Reveal>

        <Reveal delay={0.2}>
          <div className="relative max-w-3xl mx-auto">
            <div aria-hidden className="absolute -inset-4 bg-gradient-aurora opacity-40 blur-3xl rounded-[3rem]" style={{ background: "var(--gradient-aurora)" }} />
            <div className="relative glass-strong rounded-[2rem] p-10 md:p-12 shadow-elegant noise overflow-hidden">
              <AuroraBackground className="opacity-30" />
              <div className="relative flex flex-col md:flex-row items-center gap-8">
                <div className="relative">
                  <div className="size-24 rounded-2xl bg-gradient-primary grid place-items-center shadow-glow">
                    <DownloadIcon className="size-10 text-primary-foreground" />
                  </div>
                  <div className="absolute -top-1 -right-1 size-6 rounded-full bg-emerald grid place-items-center ring-4 ring-background">
                    <Sparkles className="size-3 text-emerald-foreground" />
                  </div>
                </div>
                <div className="flex-1 text-center md:text-left">
                  <div className="font-mono text-[0.7rem] uppercase tracking-[0.22em] text-primary">Latest release</div>
                  <div className="mt-2 font-display text-2xl font-semibold break-all">{fileName}</div>
                  <div className="text-muted-foreground mt-2">~ {site.download.sizeGB} GB · x86_64 · {site.license}</div>
                </div>
                <Button asChild size="lg" className="bg-gradient-primary text-primary-foreground border-0 hover:opacity-95 h-14 px-10 text-base rounded-full btn-shine shadow-glow">
                  <a href={site.download.url} download>
                    <DownloadIcon className="mr-2 size-5" />
                    Download ISO
                  </a>
                </Button>
              </div>
              <p className="relative mt-8 text-sm text-muted-foreground text-center">
                By downloading you accept the <span className="text-foreground font-medium">{site.license}</span> license · {" "}
                <a href={site.download.releaseNotesUrl} className="text-primary hover:underline">Release notes →</a>
              </p>
            </div>
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

const InstructionsAndVerify = () => {
  const [copied, setCopied] = useState(false);
  const fileName = `Samaris-OS-Mountain-Lake-Alpha-One.iso`;

  const copyHash = async () => {
    await navigator.clipboard.writeText(site.download.sha256);
    setCopied(true);
    setTimeout(() => setCopied(false), 1800);
  };

  return (
    <section className="relative min-h-screen w-full flex items-center justify-center py-16 overflow-hidden">
      <FadeOnScroll from={0.4} to={0} className="absolute inset-0">
        <AuroraBackground className="opacity-40" />
      </FadeOnScroll>
      <div className="container relative">
        <Reveal>
          <div className="text-center mb-12">
            <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-4">Flash to USB</div>
            <h2 className="font-display text-3xl md:text-4xl font-semibold">
              Write the ISO in <span className="text-gradient">two minutes</span>.
            </h2>
          </div>
        </Reveal>

        <Reveal className="max-w-3xl mx-auto mb-16">
          <Tabs defaultValue="unix">
            <TabsList className="glass w-full grid grid-cols-2 h-12 p-1 rounded-full mb-6">
              <TabsTrigger value="unix" className="rounded-full data-[state=active]:bg-gradient-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-glow">Linux / macOS</TabsTrigger>
              <TabsTrigger value="win" className="rounded-full data-[state=active]:bg-gradient-primary data-[state=active]:text-primary-foreground data-[state=active]:shadow-glow">Windows</TabsTrigger>
            </TabsList>

            <TabsContent value="unix" className="space-y-4">
              <p className="text-sm text-muted-foreground">
                Identify your USB device with <code className="text-primary font-mono">lsblk</code> (Linux) or{" "}
                <code className="text-primary font-mono">diskutil list</code> (macOS), then run:
              </p>
              <CodeBlock code={`sudo dd if=${fileName} of=/dev/sdX bs=4M status=progress oflag=sync`} />
              <p className="text-xs text-muted-foreground">
                Replace <code className="text-foreground font-mono">/dev/sdX</code> with your USB device.
                On macOS use <code className="text-foreground font-mono">/dev/rdiskN</code> for ~10× speed.
              </p>
            </TabsContent>

            <TabsContent value="win" className="grid gap-4 sm:grid-cols-2">
              <SpotlightCard className="p-6">
                <a href="https://rufus.ie" target="_blank" rel="noreferrer" className="block">
                  <div className="font-display font-semibold text-lg mb-2">Rufus</div>
                  <p className="text-sm text-muted-foreground leading-relaxed">Lightweight, fast, Windows-native. Select the ISO and your USB drive — done.</p>
                  <div className="mt-3 text-primary text-sm font-medium">rufus.ie →</div>
                </a>
              </SpotlightCard>
              <SpotlightCard className="p-6">
                <a href="https://etcher.balena.io" target="_blank" rel="noreferrer" className="block">
                  <div className="font-display font-semibold text-lg mb-2">Balena Etcher</div>
                  <p className="text-sm text-muted-foreground leading-relaxed">Cross-platform, drag-and-drop simple. Verifies the write automatically.</p>
                  <div className="mt-3 text-primary text-sm font-medium">etcher.balena.io →</div>
                </a>
              </SpotlightCard>
            </TabsContent>
          </Tabs>
        </Reveal>

        <div className="grid gap-8 lg:grid-cols-2 max-w-4xl mx-auto">
          <Reveal>
            <SpotlightCard className="p-6">
              <div className="font-mono text-[0.65rem] uppercase tracking-[0.22em] text-primary mb-4">Requirements</div>
              <h2 className="font-display text-lg font-semibold mb-4">The bare minimum.</h2>
              <ul className="space-y-3">
                {reqs.map((r) => (
                  <li key={r.label} className="flex items-center gap-3">
                    <div className="size-9 rounded-lg bg-gradient-primary grid place-items-center text-primary-foreground shadow-glow shrink-0">
                      <r.icon className="size-4" />
                    </div>
                    <div>
                      <div className="text-sm font-semibold text-foreground">{r.label}</div>
                      <div className="text-xs text-muted-foreground">{r.value}</div>
                    </div>
                  </li>
                ))}
              </ul>
            </SpotlightCard>
          </Reveal>

          <Reveal delay={0.05}>
            <SpotlightCard className="p-6">
              <div className="flex items-center gap-2 mb-4">
                <ShieldCheck className="size-5 text-emerald" />
                <span className="font-mono text-[0.7rem] uppercase tracking-[0.22em] text-emerald">Verify</span>
              </div>
              <h2 className="font-display text-lg font-semibold mb-3">Trust, but verify.</h2>
              <p className="text-sm text-muted-foreground mb-4 leading-relaxed">
                Run <code className="text-primary font-mono">sha256sum samaris-os.iso</code> and compare:
              </p>
              <div className="glass-strong rounded-lg p-2.5 flex items-start gap-2">
                <code className="text-xs font-mono break-all flex-1 text-foreground/90">
                  {site.download.sha256}
                </code>
                <Button size="sm" variant="ghost" className="h-6 px-2 shrink-0" onClick={copyHash} aria-label="Copy checksum">
                  {copied ? <Check className="size-3 text-emerald" /> : <Copy className="size-3" />}
                </Button>
              </div>
              <a href={site.download.releaseNotesUrl} className="mt-4 inline-flex items-center gap-1.5 text-sm text-primary hover:underline font-medium">
                <FileText className="size-4" />
                Release notes →
              </a>
            </SpotlightCard>
          </Reveal>
        </div>
      </div>
    </section>
  );
};

const Download = () => {
  usePageMeta({
    title: "Download",
    description: "Download the Samaris OS Mountain Lake Alpha One ISO (1.9 GB, x86_64) — free, no account, no telemetry. Includes flash instructions for Linux, macOS, and Windows.",
    keywords: "download Samaris OS, free portable OS, USB boot ISO, x86_64 operating system download",
    canonicalPath: "/download",
  });
  return (
    <div>
      <main>
        <Hero />
        <InstructionsAndVerify />
      </main>
    </div>
  );
};

export default Download;