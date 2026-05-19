import { PageHero } from "@/components/common/PageHero";
import { SpotlightCard } from "@/components/common/SpotlightCard";
import { SectionHeading } from "@/components/common/SectionHeading";
import { Reveal } from "@/components/common/Reveal";
import { Button } from "@/components/ui/button";
import { site } from "@/config/site";
import { FileText, Copy, Check, Shield, AlertTriangle, Globe, Lock, Mail, Heart } from "lucide-react";
import { useState } from "react";
import { usePageMeta } from "@/hooks/usePageMeta";

const License = () => {
  usePageMeta({
    title: "License",
    description: "The Samaris Public License (SPL) governing Samaris OS — open-source foundations, proprietary platform layer, and what you can and cannot do with the software.",
    keywords: "Samaris OS license, SPL license, Samaris Public License, open source operating system",
    canonicalPath: "/license",
  });
  const [copied, setCopied] = useState(false);

  return (
    <div>
      <main>
        <PageHero
          eyebrow="License"
          title={<>Samaris Public License (SPL)</>}
          description="The legal framework governing Samaris OS — transparent, fair, and human-readable."
        />

        <div className="container -mt-12">
          <div className="mt-10 space-y-8">
            <Reveal>
              <div className="flex flex-col sm:flex-row items-center gap-4 glass-strong rounded-2xl p-6 mb-8">
                <div className="size-12 rounded-xl bg-gradient-primary grid place-items-center shadow-glow shrink-0">
                  <FileText className="size-6 text-primary-foreground" />
                </div>
                <div className="text-center sm:text-left">
                  <h2 className="font-display text-xl font-semibold text-foreground">Samaris Public License (SPL)</h2>
                  <p className="text-sm text-muted-foreground mt-1">
                    Effective Alpha One • Samaris Public License (SPL)
                  </p>
                </div>
              </div>
            </Reveal>

            <Reveal delay={0.02}>
              <SpotlightCard className="p-8 border-l-4 border-l-secondary">
                <div className="flex items-start gap-4">
                  <div className="size-10 rounded-lg bg-secondary/15 grid place-items-center text-secondary shrink-0">
                    <AlertTriangle className="size-5" />
                  </div>
                  <div>
                    <h2 className="font-display text-xl font-semibold text-foreground mb-3">Experimental Alpha Notice</h2>
                    <p className="text-muted-foreground leading-relaxed">
                      Samaris OS Mountain Lake Alpha One is experimental pre-release software. Bugs, instability, incomplete features, hardware incompatibilities, and data loss are possible. It is not intended for production or safety-critical environments.
                    </p>
                  </div>
                </div>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.04}>
              <SpotlightCard className="p-8 bg-gradient-to-br from-primary/5 to-transparent">
                <div className="flex items-start gap-4 mb-6">
                  <div className="size-10 rounded-lg bg-primary/15 grid place-items-center text-primary shrink-0">
                    <Heart className="size-5" />
                  </div>
                  <div>
                    <h2 className="font-display text-xl font-semibold text-foreground mb-1">In plain English</h2>
                    <p className="text-sm text-muted-foreground">A human-readable summary</p>
                  </div>
                </div>
                <div className="space-y-3 text-muted-foreground leading-relaxed">
                  <p>Samaris OS is free to download and explore as an experimental alpha operating system.</p>
                  <p>The project respects open-source software and is built on top of it.</p>
                  <p>The Samaris platform layer, branding, official builds, and proprietary assets remain protected.</p>
                  <p className="font-medium text-foreground">Commercial use and redistribution of official builds require permission.</p>
                </div>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.06}>
              <SpotlightCard className="p-8">
                <div className="flex items-start gap-4 mb-6">
                  <div className="size-10 rounded-lg bg-emerald/15 grid place-items-center text-emerald shrink-0">
                    <Globe className="size-5" />
                  </div>
                  <div>
                    <h2 className="font-display text-xl font-semibold text-foreground mb-1">Built on open-source foundations</h2>
                    <p className="text-sm text-muted-foreground">The hybrid architecture</p>
                  </div>
                </div>
                <p className="text-muted-foreground leading-relaxed mb-4">
                  Samaris OS is built on open-source software including Linux, Debian/Ubuntu packages, Chromium, Node.js, React, and many upstream projects. Those components remain governed by their original licenses.
                </p>
                <p className="text-muted-foreground leading-relaxed">
                  The Samaris platform layer — including branding, UI assets, proprietary applications, official builds, and product identity — remains protected under the <span className="text-foreground font-medium">Samaris Public License (SPL)</span>.
                </p>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.08}>
              <SpotlightCard className="p-8">
                <h2 className="font-display text-xl font-semibold text-foreground mb-6">Licensing model</h2>
                <div className="space-y-4 text-muted-foreground">
                  <div className="flex items-start gap-3">
                    <div className="size-2 rounded-full bg-emerald mt-2 shrink-0" />
                    <span><strong className="text-foreground">Open-source components</strong> remain under their original licenses (GPL, MIT, LGPL, BSD, Apache, MPL).</span>
                  </div>
                  <div className="flex items-start gap-3">
                    <div className="size-2 rounded-full bg-primary mt-2 shrink-0" />
                    <span><strong className="text-foreground">Samaris-specific assets</strong> — branding, UI, official builds, and proprietary platform components — are protected under the <span className="text-foreground font-medium">Samaris Public License (SPL)</span>.</span>
                  </div>
                </div>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.1}>
              <SpotlightCard className="p-8 overflow-hidden">
                <h2 className="font-display text-xl font-semibold text-foreground mb-6">What this means in practice</h2>
                <div className="grid md:grid-cols-2 gap-8">
                  <div className="space-y-4">
                    <div className="flex items-center gap-3 mb-4">
                      <div className="size-8 rounded-lg bg-emerald/15 grid place-items-center text-emerald">
                        <Check className="size-4" />
                      </div>
                      <h3 className="font-display font-semibold text-foreground">You can:</h3>
                    </div>
                    <div className="space-y-3 text-muted-foreground">
                      <div className="flex items-start gap-3"><span className="text-emerald">✓</span><span>Download and test Samaris OS</span></div>
                      <div className="flex items-start gap-3"><span className="text-emerald">✓</span><span>Publish reviews, screenshots, and videos</span></div>
                      <div className="flex items-start gap-3"><span className="text-emerald">✓</span><span>Explore the architecture</span></div>
                      <div className="flex items-start gap-3"><span className="text-emerald">✓</span><span>Use open-source components under their original licenses</span></div>
                      <div className="flex items-start gap-3"><span className="text-emerald">✓</span><span>Modify open-source components for personal use</span></div>
                    </div>
                  </div>
                  <div className="space-y-4">
                    <div className="flex items-center gap-3 mb-4">
                      <div className="size-8 rounded-lg bg-destructive/15 grid place-items-center text-destructive">
                        <Shield className="size-4" />
                      </div>
                      <h3 className="font-display font-semibold text-foreground">You cannot:</h3>
                    </div>
                    <div className="space-y-3 text-muted-foreground">
                      <div className="flex items-start gap-3"><span className="text-destructive">✗</span><span>Redistribute official branded ISOs</span></div>
                      <div className="flex items-start gap-3"><span className="text-destructive">✗</span><span>Repackage Samaris OS under another identity</span></div>
                      <div className="flex items-start gap-3"><span className="text-destructive">✗</span><span>Use Samaris branding commercially without permission</span></div>
                      <div className="flex items-start gap-3"><span className="text-destructive">✗</span><span>Present unofficial builds as official releases</span></div>
                      <div className="flex items-start gap-3"><span className="text-destructive">✗</span><span>Remove copyright or attribution notices</span></div>
                    </div>
                  </div>
                </div>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.12}>
              <SpotlightCard className="p-8">
                <div className="flex items-start gap-4 mb-6">
                  <div className="size-10 rounded-lg bg-emerald/15 grid place-items-center text-emerald shrink-0">
                    <Lock className="size-5" />
                  </div>
                  <div>
                    <h2 className="font-display text-xl font-semibold text-foreground mb-1">Security & privacy philosophy</h2>
                    <p className="text-sm text-muted-foreground">Local-first by design</p>
                  </div>
                </div>
                <p className="text-muted-foreground leading-relaxed mb-4">
                  Samaris OS is designed around a local-first philosophy. No hidden telemetry, advertising identifiers, or behavioral analytics are enabled by default.
                </p>
                <p className="text-muted-foreground leading-relaxed">
                  Advanced hardening, sandboxing, and permission systems are still evolving as part of the Alpha roadmap.
                </p>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.14}>
              <SpotlightCard className="p-8">
                <div className="flex items-start gap-4 mb-6">
                  <div className="size-10 rounded-lg bg-secondary/15 grid place-items-center text-secondary shrink-0">
                    <Shield className="size-5" />
                  </div>
                  <div>
                    <h2 className="font-display text-xl font-semibold text-foreground mb-1">Official builds</h2>
                    <p className="text-sm text-muted-foreground">Verify before you flash</p>
                  </div>
                </div>
                <p className="text-muted-foreground leading-relaxed">
                  Official Samaris OS releases are distributed only through official channels. Always verify checksums and release integrity before flashing an image.
                </p>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.16}>
              <SpotlightCard className="p-8 overflow-hidden">
                <div className="flex items-center gap-3 mb-6">
                  <div className="size-10 rounded-lg bg-secondary/15 grid place-items-center text-secondary">
                    <FileText className="size-5" />
                  </div>
                  <h2 className="font-display text-xl font-semibold text-foreground">Open-source components</h2>
                </div>
                <p className="text-muted-foreground mb-6">
                  The following open-source components are used in Samaris OS. Each remains governed by its original license.
                </p>
                <div className="overflow-x-auto">
                  <table className="w-full text-left text-sm">
                    <thead>
                      <tr className="border-b border-border/60">
                        <th className="px-6 py-4 font-mono text-[0.7rem] uppercase tracking-[0.2em] text-muted-foreground">Component</th>
                        <th className="px-6 py-4 font-mono text-[0.7rem] uppercase tracking-[0.2em] text-muted-foreground">License</th>
                      </tr>
                    </thead>
                    <tbody>
                      {[
                        ["Linux kernel", "GPL-2.0"],
                        ["GRUB", "GPL-family"],
                        ["systemd", "LGPL-2.1+"],
                        ["Plymouth", "GPL-2.0+"],
                        ["Chromium", "Various (BSD)"],
                        ["Node.js", "MIT"],
                        ["React", "MIT"],
                        ["Vite", "MIT"],
                        ["Tailwind CSS", "MIT"],
                        ["Debian / Ubuntu packages", "Various"],
                      ].map(([component, license]) => (
                        <tr key={component} className="border-t border-border/40">
                          <td className="px-6 py-4 text-foreground">{component}</td>
                          <td className="px-6 py-4 text-muted-foreground">{license}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.18}>
              <SpotlightCard className="p-8">
                <h2 className="font-display text-xl font-semibold text-foreground mb-4">No warranty</h2>
                <p className="text-muted-foreground leading-relaxed">
                  Samaris OS is provided <em>as is</em> and <em>as available</em>, without warranty of any kind, express or implied, including but not limited to warranties of merchantability, fitness for a particular purpose, and non-infringement.
                </p>
                <p className="text-muted-foreground mt-3">Use at your own risk.</p>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.2}>
              <SpotlightCard className="p-8">
                <div className="flex items-start gap-4 mb-6">
                  <div className="size-10 rounded-lg bg-primary/15 grid place-items-center text-primary shrink-0">
                    <Mail className="size-5" />
                  </div>
                  <div>
                    <h2 className="font-display text-xl font-semibold text-foreground mb-1">Commercial & research discussions</h2>
                    <p className="text-sm text-muted-foreground">Let's talk</p>
                  </div>
                </div>
                <p className="text-muted-foreground leading-relaxed mb-4">
                  For research collaborations, educational use, OEM discussions, or future commercial licensing, reach out directly.
                </p>
                <div className="flex items-center gap-3">
                  <a
                    href={`mailto:${site.email}`}
                    className="inline-flex items-center gap-2 bg-gradient-primary text-primary-foreground hover:opacity-95 h-10 px-5 text-sm rounded-full btn-shine shadow-glow font-medium"
                  >
                    <Mail className="size-4" />
                    {site.email}
                  </a>
                  <Button
                    size="sm"
                    variant="ghost"
                    className="h-10 px-4"
                    onClick={() => {
                      navigator.clipboard.writeText(site.email);
                      setCopied(true);
                      setTimeout(() => setCopied(false), 2000);
                    }}
                  >
                    {copied ? <Check className="size-4 text-emerald" /> : <Copy className="size-4" />}
                    {copied ? "Copied" : "Copy"}
                  </Button>
                </div>
              </SpotlightCard>
            </Reveal>

            <Reveal delay={0.22}>
              <div className="text-center py-8 border-t border-border/40">
                <p className="font-display text-lg text-muted-foreground italic">
                  A proprietary platform layer built on open-source foundations.
                </p>
              </div>
            </Reveal>
          </div>
        </div>
      </main>
    </div>
  );
};

export default License;
