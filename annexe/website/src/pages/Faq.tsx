import { useMemo, useState, useEffect } from "react";
import { Search, MessageCircleQuestion } from "lucide-react";
import { Accordion, AccordionContent, AccordionItem, AccordionTrigger } from "@/components/ui/accordion";
import { Input } from "@/components/ui/input";
import { Reveal } from "@/components/common/Reveal";
import { PageHero } from "@/components/common/PageHero";
import { SpotlightCard } from "@/components/common/SpotlightCard";
import { site } from "@/config/site";
import { usePageMeta } from "@/hooks/usePageMeta";

const faqs = [
  {
    cat: "Basics",
    q: "Is this a real operating system?",
    a: "Yes. Samaris OS is a real, bootable operating system. It boots from a USB key and provides a functional desktop environment. However, it's currently in Alpha stage — experimental, VM-tested primarily, and not production-ready.",
  },
  {
    cat: "Basics",
    q: "Why does Samaris OS use Chromium?",
    a: "Chromium acts as the GPU-accelerated rendering layer for the desktop experience. Linux provides the system foundation, Node.js handles native orchestration (persistence, filesystem access, application integration), and React builds the desktop shell. It's not 'just a browser' — the browser is the rendering engine powering a complete OS UI.",
  },
  {
    cat: "Basics",
    q: "Is Samaris OS Linux?",
    a: "Yes and no. Samaris OS uses the Linux kernel as its foundation, but it's not a traditional Linux distribution. It ships with a single, opinionated custom desktop environment, a curated App Store, an integrated local AI assistant, and a privacy-first default configuration. You don't choose a desktop — there is one, designed end to end.",
  },
  {
    cat: "Basics",
    q: "Why release an alpha publicly?",
    a: "Samaris OS has been in development long enough to be shareable. We believe in transparency — showing the project as it is, early and honest. An alpha release invites early feedback, real-world testing, and collaborative improvement. The goal is not to oversell, but to be honest about where things stand.",
  },
  {
    cat: "Basics",
    q: "What is Samaris OS?",
    a: "Samaris OS is an experimental operating system that runs from a USB key. It is built on the Linux kernel with a custom desktop environment rendered through Chromium, an integrated Wine compatibility layer, a lightweight experimental local AI assistant called Orbit AI, and encryption architecture. It is an independent project focused on privacy-first, local-first computing.",
  },
  {
    cat: "Basics",
    q: "Is it really free?",
    a: `Yes. The Samaris OS ISO is free to download and use under the ${site.license} license. The operating system itself is never charged for. Commercial licensing exists only for organisations that want to embed Samaris OS into their own hardware products.`,
  },
  {
    cat: "Basics",
    q: "Is Samaris OS production-ready?",
    a: "No. Samaris OS is currently a public alpha. Core features (USB boot, custom desktop, encryption architecture, Wine layer, Orbit AI) work well on supported hardware. Other areas — ARM64, advanced security, in-place updates — are still in active development. Treat it as an experimental sovereign OS, not a Windows or macOS replacement.",
  },
  {
    cat: "Hardware",
    q: "Which architectures are supported?",
    a: "Samaris OS currently supports x86_64 (Intel and AMD) — modern PCs and Intel-based Macs. Currently focused on VM-tested x86_64 systems. ARM64 support, including Apple Silicon and Raspberry Pi, is in active development and not yet released as a stable image.",
  },
  {
    cat: "Hardware",
    q: "Does it work on Apple Silicon Macs or Raspberry Pi?",
    a: "Not yet. The current build targets x86_64 only. ARM64 support — Apple Silicon (M1/M2/M3) and Raspberry Pi 4/5 — is on the roadmap.",
  },
  {
    cat: "Hardware",
    q: "Will it run on my exact laptop?",
    a: "Most modern x86_64 laptops boot fine. Hardware validation is ongoing — Wi-Fi chipsets, GPUs and laptop sensors may need workarounds depending on the model. We recommend testing from USB before installing.",
  },
  {
    cat: "Hardware",
    q: "Why is USB-first design important?",
    a: "USB-first means carrying your full computing environment in your pocket. Your OS, your data, your settings — on a device you control. No cloud dependency, no mandatory accounts, no telemetry. It's about computing sovereignty and portability.",
  },
  {
    cat: "Apps",
    q: "Can I run Windows applications?",
    a: "Many of them. Samaris OS ships with an integrated Wine compatibility layer, so a wide range of Windows applications can be launched directly without a virtual machine. Compatibility varies by application — treat it as best-effort, not a complete Windows replacement.",
  },
  {
    cat: "Apps",
    q: "How does the App Store work?",
    a: "The App Store installs applications directly from curated GitHub repositories, web apps and other compatible packages, through a simplified desktop interface. No account is required and there is no centralised app marketplace.",
  },
  {
    cat: "AI",
    q: "What is Orbit AI?",
    a: "Orbit AI is an experimental lightweight local AI interface. It integrates compact CPU-only language models directly into the desktop environment. It is not, and is not trying to be, a cloud-scale assistant or a replacement for cloud AI services.",
  },
  {
    cat: "Privacy",
    q: "Is my data safe?",
    a: "LUKS full-disk encryption architecture exists and is implemented, but is temporarily disabled in Alpha One during VM-focused testing. There is no analytics platform, no advertising trackers and no user profiling under normal operation. More advanced controls — a stateful firewall and granular per-app permissions — are on the roadmap.",
  },
  {
    cat: "Privacy",
    q: "Does Samaris OS have zero telemetry?",
    a: "Samaris OS has no hidden telemetry and no analytics under normal operation. However, there are exceptions: GitHub-based app downloads, website logs, and future opt-in services. The system is designed around local-first privacy, but complete isolation isn't guaranteed.",
  },
  {
    cat: "Privacy",
    q: "Do I need an internet connection?",
    a: "No. Samaris OS is fully usable offline. Orbit AI runs locally on your device — your prompts and your data never leave the machine. Internet is only required to install new apps or browse the web.",
  },
  {
    cat: "Updates",
    q: "How do I update?",
    a: "Updates currently ship as new ISO releases. You re-flash the same USB key — persistence and migration workflows are still evolving. In-place incremental updates are planned for a future release.",
  },
  {
    cat: "Licensing",
    q: "What is the license?",
    a: `Samaris OS is source-available, released under the ${site.license} (Samaris Public License). Built on open-source foundations with proprietary branding, UX assets, platform integrations, and official distributions. Free for individuals and small teams. Commercial redistribution and OEM bundling require a separate license — contact ${site.email}.`,
  },
  {
    cat: "Project",
    q: "Who is behind this?",
    a: "Samaris OS is an independent project. It is a founder-led project, not a venture-backed startup or a corporate platform. Every decision is made directly by the team building it.",
  },
  {
    cat: "Project",
    q: "Is this open source?",
    a: "Samaris OS is source-available, not fully open source. It's built on open-source foundations (Linux, Node.js, React, etc.) under their respective licenses. However, the Samaris platform itself has proprietary components: branding, UI assets, platform integrations, and official distributions. The source is available for reference and modification, but official builds and branding are controlled.",
  },
];

const categories = ["All", ...Array.from(new Set(faqs.map((f) => f.cat)))];

const Faq = () => {
  usePageMeta({
    title: "FAQ",
    description: "Frequently asked questions about Samaris OS — answers about the operating system, hardware support, Wine compatibility, Orbit AI, privacy, licensing, and more.",
    keywords: "Samaris OS FAQ, frequently asked questions, portable OS questions, local-first OS FAQ",
    canonicalPath: "/faq",
  });
  const [query, setQuery] = useState("");
  const [cat, setCat] = useState<string>("All");

  useEffect(() => {
    const faqSchema = {
      "@context": "https://schema.org",
      "@type": "FAQPage",
      "mainEntity": faqs.map((f) => ({
        "@type": "Question",
        "name": f.q,
        "acceptedAnswer": {
          "@type": "Answer",
          "text": f.a.replace(/<[^>]*>/g, ""),
        },
      })),
    };

    let existing = document.querySelector<HTMLScriptElement>('script[data-seo-schema="faq"]');
    if (!existing) {
      existing = document.createElement("script");
      existing.type = "application/ld+json";
      existing.setAttribute("data-seo-schema", "faq");
      existing.textContent = JSON.stringify(faqSchema);
      document.head.appendChild(existing);
    }
  }, []);

  const filtered = useMemo(() => {
    const q = query.trim().toLowerCase();
    return faqs.filter((f) => {
      const matchesCat = cat === "All" || f.cat === cat;
      const matchesQ = !q || f.q.toLowerCase().includes(q) || f.a.toLowerCase().includes(q);
      return matchesCat && matchesQ;
    });
  }, [query, cat]);

  return (
    <div>
      <main>
        <PageHero
        eyebrow="Frequently asked"
        title={<>Honest answers, <span className="text-gradient">no marketing fluff</span>.</>}
        description="Everything worth knowing before flashing your first Samaris USB — including what doesn't work yet."
      />

      <div className="container -mt-8">
        <Reveal className="max-w-2xl mx-auto">
          <div className="relative">
            <Search className="size-4 absolute left-5 top-1/2 -translate-y-1/2 text-muted-foreground" />
            <Input
              value={query}
              onChange={(e) => setQuery(e.target.value)}
              placeholder="Search questions..."
              className="glass-strong border-border h-14 pl-12 rounded-full text-base shadow-elegant"
              aria-label="Search FAQs"
            />
          </div>
        </Reveal>

        <Reveal className="mt-6 flex flex-wrap items-center justify-center gap-2 max-w-3xl mx-auto">
          {categories.map((c) => (
            <button
              key={c}
              onClick={() => setCat(c)}
              className={`px-4 py-1.5 rounded-full text-xs font-mono uppercase tracking-[0.18em] transition-all ${
                cat === c
                  ? "bg-gradient-primary text-primary-foreground shadow-glow"
                  : "glass text-muted-foreground hover:text-foreground"
              }`}
            >
              {c}
            </button>
          ))}
        </Reveal>

        <Reveal className="mt-12 max-w-3xl mx-auto mb-24">
          <SpotlightCard className="px-6 md:px-8 py-2">
            <Accordion type="single" collapsible className="w-full">
              {filtered.map((f, i) => (
                <AccordionItem key={f.q} value={`item-${i}`} className="border-border/40 last:border-0">
                  <AccordionTrigger className="text-left text-base md:text-lg font-display font-medium hover:text-primary hover:no-underline py-6 group">
                    <span className="flex flex-col sm:flex-row items-start sm:items-center gap-2 sm:gap-4 flex-1">
                      <span className="font-mono text-[0.6rem] uppercase tracking-[0.15em] text-primary/70 shrink-0">
                        {f.cat}
                      </span>
                      <span className="flex-1">{f.q}</span>
                    </span>
                  </AccordionTrigger>
                  <AccordionContent className="text-muted-foreground leading-relaxed text-sm md:text-base pb-6 pl-4 sm:pl-24 pr-4">
                    {f.a}
                  </AccordionContent>
                </AccordionItem>
              ))}
            </Accordion>
            {filtered.length === 0 && (
              <div className="py-16 text-center">
                <MessageCircleQuestion className="size-10 mx-auto text-muted-foreground mb-3" />
                <p className="text-sm text-muted-foreground">
                  No questions match "{query}".
                </p>
              </div>
            )}
          </SpotlightCard>
        </Reveal>
      </div>
      </main>
    </div>
  );
};

export default Faq;
