import { Link } from "react-router-dom";
import { Github, Youtube } from "lucide-react";
import { navLinks, site } from "@/config/site";
import samarisLogo from "@/assets/samaris-logo.webp";

export const Footer = () => (
  <footer className="mt-32 border-t border-border/40">
    <div className="container py-16">
      <div className="max-w-2xl mx-auto text-center">
        <Link to="/" className="inline-flex items-center gap-3 group">
          <img 
            src={samarisLogo}
            alt="Samaris OS logo" 
            loading="lazy"
            decoding="async"
            className="size-10 rounded-full ring-1 ring-border group-hover:ring-primary/50 transition-all" 
          />
          <span className="font-display text-lg font-semibold tracking-tight">{site.name}</span>
        </Link>
        
        <p className="mt-6 text-sm text-muted-foreground max-w-md mx-auto leading-relaxed">
          An experimental operating system — built on Linux with a custom desktop rendered through Chromium.
        </p>

        <nav className="mt-10 flex flex-wrap justify-center gap-x-6 gap-y-3">
          {navLinks.map((l) => (
            <Link 
              key={l.to} 
              to={l.to} 
              className="text-sm text-muted-foreground hover:text-foreground transition-colors"
            >
              {l.label}
            </Link>
          ))}
        </nav>

        <div className="mt-10 flex items-center justify-center gap-4">
          <a 
            href={site.social.github} 
            target="_blank" 
            rel="noreferrer" 
            aria-label="GitHub" 
            className="size-9 rounded-full glass grid place-items-center text-muted-foreground hover:text-foreground hover:bg-foreground/5 transition-all"
          >
            <Github className="size-4" />
          </a>
          <a 
            href={site.social.youtube} 
            target="_blank" 
            rel="noreferrer" 
            aria-label="YouTube" 
            className="size-9 rounded-full glass grid place-items-center text-muted-foreground hover:text-foreground hover:bg-foreground/5 transition-all"
          >
            <Youtube className="size-4" />
          </a>
          <a 
            href={`mailto:${site.email}`}
            aria-label="Email" 
            className="size-9 rounded-full glass grid place-items-center text-muted-foreground hover:text-foreground hover:bg-foreground/5 transition-all"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="18" height="18" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="1.5" strokeLinecap="round" strokeLinejoin="round" className="size-4">
              <rect width="20" height="16" x="2" y="4" rx="2"/>
              <path d="m22 7-8.97 5.7a1.94 1.94 0 0 1-2.06 0L2 7"/>
            </svg>
          </a>
        </div>

        <div className="mt-12 pt-8 border-t border-border/40">
          <p className="text-xs text-muted-foreground/70">
            © {new Date().getFullYear()} {site.name}. All rights reserved.
          </p>
          <p className="text-xs text-muted-foreground/50 mt-2">
            Released under the Samaris Public License (SPL)
          </p>
        </div>
      </div>
    </div>
  </footer>
);