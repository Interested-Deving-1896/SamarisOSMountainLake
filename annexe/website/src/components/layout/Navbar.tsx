import { Link, NavLink, useLocation } from "react-router-dom";
import { useEffect, useState } from "react";
import { motion, useScroll, useSpring } from "framer-motion";
import { Menu } from "lucide-react";
import { Button } from "@/components/ui/button";
import { Sheet, SheetContent, SheetTrigger } from "@/components/ui/sheet";
import { navLinks, site } from "@/config/site";
import { cn } from "@/lib/utils";
import { ThemeToggle } from "@/components/theme/ThemeToggle";
import samarisLogo from "@/assets/samaris-logo.webp";

export const Navbar = () => {
  const [scrolled, setScrolled] = useState(false);
  const [open, setOpen] = useState(false);
  const location = useLocation();
  const { scrollYProgress } = useScroll();
  const progress = useSpring(scrollYProgress, { stiffness: 120, damping: 25, mass: 0.2 });

  useEffect(() => {
    const onScroll = () => setScrolled(window.scrollY > 12);
    onScroll();
    window.addEventListener("scroll", onScroll);
    return () => window.removeEventListener("scroll", onScroll);
  }, []);

  useEffect(() => { setOpen(false); }, [location.pathname]);

  return (
    <header className={cn("fixed top-0 inset-x-0 z-50 transition-all duration-500", scrolled ? "py-3" : "py-5")}>
      <div className="container">
        <nav
          className={cn(
            "relative rounded-2xl px-5 md:px-6 h-16 md:h-18 flex items-center justify-between transition-all duration-500",
            scrolled ? "glass-strong shadow-elegant" : "glass",
          )}
          aria-label="Primary"
        >
          <Link to="/" className="flex items-center gap-2.5 group pl-2">
            <img
              src={samarisLogo}
              alt="Samaris OS logo"
              loading="eager"
              fetchPriority="high"
              className="size-8 rounded-full ring-glow transition-transform duration-500 group-hover:scale-110 group-hover:rotate-6"
            />
            <span className="font-display font-semibold tracking-tight text-foreground">
              {site.name}
            </span>
          </Link>

          <ul className="hidden md:flex items-center gap-0.5 absolute left-1/2 -translate-x-1/2 overflow-x-auto max-w-[50%] [-ms-overflow-style:none] [scrollbar-width:none]">
            {navLinks.map((link) => (
              <li key={link.to} className="shrink-0">
                <NavLink
                  to={link.to}
                  end={link.to === "/"}
                  className={({ isActive }) =>
                    cn(
                      "relative px-3 py-2 rounded-full text-sm font-medium transition-colors whitespace-nowrap",
                      isActive ? "text-foreground" : "text-muted-foreground hover:text-foreground",
                    )
                  }
                >
                  {({ isActive }) => (
                    <>
                      {isActive && (
                        <motion.span
                          layoutId="nav-pill"
                          className="absolute inset-0 rounded-full bg-foreground/[0.06] dark:bg-foreground/[0.08]"
                          transition={{ type: "spring", stiffness: 380, damping: 30 }}
                        />
                      )}
                      <span className="relative">{link.label}</span>
                    </>
                  )}
                </NavLink>
              </li>
            ))}
          </ul>

          <div className="flex items-center gap-1.5">
            <ThemeToggle className="relative" />
            <Button
              asChild
              size="sm"
              className="hidden md:inline-flex bg-gradient-primary text-primary-foreground hover:opacity-95 border-0 btn-shine rounded-full h-9 px-4 shadow-glow"
            >
              <Link to="/download">Download Alpha</Link>
            </Button>

            <Sheet open={open} onOpenChange={setOpen}>
              <SheetTrigger asChild>
                <Button size="icon" variant="ghost" className="md:hidden rounded-full" aria-label="Open menu">
                  <Menu className="size-5" />
                </Button>
              </SheetTrigger>
              <SheetContent side="right" className="glass-strong border-l border-border w-[80%] max-w-sm">
                <div className="mt-10 flex flex-col gap-1">
                  {navLinks.map((link) => (
                    <NavLink
                      key={link.to}
                      to={link.to}
                      end={link.to === "/"}
                      className={({ isActive }) =>
                        cn(
                          "px-4 py-3 rounded-xl text-base font-medium transition-colors",
                          isActive ? "bg-primary/10 text-primary" : "text-foreground/80 hover:bg-muted hover:text-foreground",
                        )
                      }
                    >
                      {link.label}
                    </NavLink>
                  ))}
                  <Button asChild className="mt-4 bg-gradient-primary text-primary-foreground border-0 rounded-full">
                    <Link to="/download">Download Alpha</Link>
                  </Button>
                </div>
              </SheetContent>
            </Sheet>
          </div>

          {/* scroll progress bar */}
          <motion.div
            style={{ scaleX: progress, transformOrigin: "0% 50%" }}
            className="absolute -bottom-px left-3 right-3 h-px bg-gradient-primary rounded-full"
          />
        </nav>
      </div>
    </header>
  );
};
