import { Outlet, useLocation } from "react-router-dom";
import { AnimatePresence, motion } from "framer-motion";
import { useEffect } from "react";
import { Navbar } from "./Navbar";
import { Footer } from "./Footer";
import { site } from "@/config/site";

export const Layout = () => {
  const location = useLocation();

  useEffect(() => {
    window.scrollTo({ top: 0, behavior: "instant" as ScrollBehavior });
  }, [location.pathname]);

  useEffect(() => {
    const schema = {
      "@context": "https://schema.org",
      "@type": "Organization",
      "name": site.name,
      "url": "https://samaris.tech",
      "logo": "https://samaris.tech/samaris-logo.png",
      "description": "An experimental operating system built for calm, local-first computing.",
      "sameAs": [
        site.social.github,
        site.social.youtube,
      ],
    };

    let existing = document.querySelector<HTMLScriptElement>('script[data-seo-schema="organization"]');
    if (!existing) {
      existing = document.createElement("script");
      existing.type = "application/ld+json";
      existing.setAttribute("data-seo-schema", "organization");
      existing.textContent = JSON.stringify(schema);
      document.head.appendChild(existing);
    }
  }, []);

  return (
    <div className="min-h-screen flex flex-col">
      <Navbar />
      <main className="flex-1 pt-24">
        <AnimatePresence mode="wait">
          <motion.div
            key={location.pathname}
            initial={{ opacity: 0, y: 8 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -8 }}
            transition={{ duration: 0.35, ease: "easeOut" }}
          >
            <Outlet />
          </motion.div>
        </AnimatePresence>
      </main>
      <Footer />
    </div>
  );
};
