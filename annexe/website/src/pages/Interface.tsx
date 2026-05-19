import { useState } from "react";
import { motion, AnimatePresence } from "framer-motion";
import { X, ChevronLeft, ChevronRight } from "lucide-react";
import { PageHero } from "@/components/common/PageHero";
import { Reveal } from "@/components/common/Reveal";
import { Button } from "@/components/ui/button";
import { cn } from "@/lib/utils";
import { usePageMeta } from "@/hooks/usePageMeta";
import heroWallpaper from "@/assets/hero-wallpaper.webp";

interface GalleryItem {
  id: number;
  title: string;
  caption: string;
  image: string;
}

const galleryItems: GalleryItem[] = [
  {
    id: 1,
    title: "Desktop Overview",
    caption: "The main desktop environment with dock, wallpaper, and clean window chrome.",
    image: heroWallpaper,
  },
  {
    id: 2,
    title: "Finder",
    caption: "Navigate your files with the native Finder — clean, minimal, and functional.",
    image: heroWallpaper,
  },
  {
    id: 3,
    title: "Settings",
    caption: "System preferences and configuration panels unified in one place.",
    image: heroWallpaper,
  },
  {
    id: 4,
    title: "App Store",
    caption: "Install applications directly from curated GitHub repositories.",
    image: heroWallpaper,
  },
  {
    id: 5,
    title: "Orbit AI",
    caption: "A local-first AI assistant that runs entirely on-device. Private and offline.",
    image: heroWallpaper,
  },
  {
    id: 6,
    title: "System Monitor",
    caption: "Track CPU, memory, and system performance in real-time.",
    image: heroWallpaper,
  },
  {
    id: 7,
    title: "Music Player",
    caption: "Listen to your music library with a built-in audio player.",
    image: heroWallpaper,
  },
  {
    id: 8,
    title: "Terminal",
    caption: "A modern terminal emulator for power users and developers.",
    image: heroWallpaper,
  },
  {
    id: 9,
    title: "Login Screen",
    caption: "The elegant login screen greeting you when you power on Samaris OS.",
    image: heroWallpaper,
  },
];

const Lightbox = ({
  item,
  onClose,
  onPrev,
  onNext,
  hasPrev,
  hasNext,
}: {
  item: GalleryItem;
  onClose: () => void;
  onPrev: () => void;
  onNext: () => void;
  hasPrev: boolean;
  hasNext: boolean;
}) => (
  <motion.div
    initial={{ opacity: 0 }}
    animate={{ opacity: 1 }}
    exit={{ opacity: 0 }}
    className="fixed inset-0 z-50 flex items-center justify-center bg-background/95 backdrop-blur-xl"
    onClick={onClose}
  >
    <button
      onClick={(e) => {
        e.stopPropagation();
        onClose();
      }}
      className="absolute top-4 right-4 p-3 sm:p-2 rounded-full bg-foreground/10 hover:bg-foreground/20 transition-colors z-10"
    >
      <X className="size-5 sm:size-5" />
    </button>

    {hasPrev && (
      <button
        onClick={(e) => {
          e.stopPropagation();
          onPrev();
        }}
        className="absolute left-2 sm:left-4 top-1/2 -translate-y-1/2 p-4 sm:p-3 rounded-full bg-foreground/10 hover:bg-foreground/20 transition-colors"
      >
        <ChevronLeft className="size-6 sm:size-6" />
      </button>
    )}

    {hasNext && (
      <button
        onClick={(e) => {
          e.stopPropagation();
          onNext();
        }}
        className="absolute right-2 sm:right-4 top-1/2 -translate-y-1/2 p-4 sm:p-3 rounded-full bg-foreground/10 hover:bg-foreground/20 transition-colors"
      >
        <ChevronRight className="size-6 sm:size-6" />
      </button>
    )}

    <motion.div
      initial={{ scale: 0.9, opacity: 0 }}
      animate={{ scale: 1, opacity: 1 }}
      exit={{ scale: 0.9, opacity: 0 }}
      className="max-w-5xl max-h-[80vh] w-full mx-4"
      onClick={(e) => e.stopPropagation()}
    >
      <div className="relative rounded-2xl overflow-hidden glass-strong shadow-elegant">
        <img
          src={item.image}
          alt={item.title}
          className="w-full h-auto max-h-[70vh] object-cover"
        />
        <div className="absolute bottom-0 inset-x-0 p-6 bg-gradient-to-t from-background/90 to-transparent">
          <h3 className="font-display text-xl font-semibold">{item.title}</h3>
          <p className="text-muted-foreground mt-1">{item.caption}</p>
        </div>
      </div>
    </motion.div>
  </motion.div>
);

const GalleryCard = ({
  item,
  index,
  onClick,
}: {
  item: GalleryItem;
  index: number;
  onClick: () => void;
}) => (
  <motion.div
    initial={{ opacity: 0, y: 30 }}
    whileInView={{ opacity: 1, y: 0 }}
    viewport={{ once: true, margin: "-40px" }}
    transition={{ duration: 0.5, delay: index * 0.05, ease: [0.22, 1, 0.36, 1] }}
    className="group"
  >
    <button
      onClick={onClick}
      className="relative w-full text-left focus:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-4 rounded-2xl"
    >
      <div className="relative aspect-[4/3] rounded-2xl overflow-hidden glass-strong shadow-elegant transition-all duration-500 group-hover:shadow-glow group-hover:scale-[1.02]">
        <img
          src={item.image}
          alt={item.title}
          loading="lazy"
          decoding="async"
          className="w-full h-full object-cover transition-transform duration-700 group-hover:scale-110"
        />
        <div className="absolute inset-0 bg-gradient-to-t from-background/80 via-transparent to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300" />
        <div className="absolute inset-0 flex items-center justify-center opacity-0 group-hover:opacity-100 transition-opacity duration-300">
          <div className="size-14 rounded-full bg-foreground/20 backdrop-blur-md grid place-items-center">
            <svg
              xmlns="http://www.w3.org/2000/svg"
              width="24"
              height="24"
              viewBox="0 0 24 24"
              fill="none"
              stroke="currentColor"
              strokeWidth="1.5"
              strokeLinecap="round"
              strokeLinejoin="round"
              className="size-6"
            >
              <path d="M15 3h6v6M9 21H3v-6M21 3l-7 7M3 21l7-7" />
            </svg>
          </div>
        </div>
      </div>
      <div className="mt-4 px-1">
        <h3 className="font-display text-lg font-semibold group-hover:text-primary transition-colors">
          {item.title}
        </h3>
        <p className="text-sm text-muted-foreground mt-1 line-clamp-2">
          {item.caption}
        </p>
      </div>
    </button>
  </motion.div>
);

const Interface = () => {
  usePageMeta({
    title: "Interface",
    description: "Explore the Samaris OS desktop environment — the dock, AirBar, Finder, settings, and Orbit AI assistant through a visual screenshot gallery.",
    keywords: "Samaris OS interface, desktop environment screenshots, privacy OS UI, local-first OS interface",
    canonicalPath: "/interface",
  });
  const [selectedIndex, setSelectedIndex] = useState<number | null>(null);

  const openLightbox = (index: number) => setSelectedIndex(index);
  const closeLightbox = () => setSelectedIndex(null);

  const goToPrev = () => {
    if (selectedIndex !== null && selectedIndex > 0) {
      setSelectedIndex(selectedIndex - 1);
    }
  };

  const goToNext = () => {
    if (selectedIndex !== null && selectedIndex < galleryItems.length - 1) {
      setSelectedIndex(selectedIndex + 1);
    }
  };

  return (
    <div>
      <PageHero
        eyebrow="Samaris OS · Interface"
        title="Interface"
        description="A calm, sovereign desktop environment designed for local-first computing."
      />

      <section className="py-24">
        <div className="container">
          <Reveal>
            <div className="text-center mb-16">
              <div className="font-mono text-[0.65rem] uppercase tracking-[0.25em] text-primary mb-6">
                Gallery
              </div>
              <h2 className="font-display text-3xl md:text-4xl font-semibold tracking-tight">
                Experience the <span className="text-gradient">desktop environment</span>.
              </h2>
              <p className="mt-4 text-muted-foreground text-base md:text-lg max-w-xl mx-auto">
                A visual tour of the Samaris OS interface. Click any screenshot to preview in full.
              </p>
            </div>
          </Reveal>

          <div className="grid gap-6 sm:grid-cols-2 lg:grid-cols-3">
            {galleryItems.map((item, index) => (
              <GalleryCard
                key={item.id}
                item={item}
                index={index}
                onClick={() => openLightbox(index)}
              />
            ))}
          </div>
        </div>
      </section>

      <AnimatePresence>
        {selectedIndex !== null && (
          <Lightbox
            item={galleryItems[selectedIndex]}
            onClose={closeLightbox}
            onPrev={goToPrev}
            onNext={goToNext}
            hasPrev={selectedIndex > 0}
            hasNext={selectedIndex < galleryItems.length - 1}
          />
        )}
      </AnimatePresence>
    </div>
  );
};

export default Interface;