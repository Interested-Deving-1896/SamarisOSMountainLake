// All placeholders live here — swap in real values post‑ALPHA launch.
export const site = {
  name: "Samaris OS",
  company: "Samaris OS",
  tagline: "An experimental sovereign computing platform.",
  email: "contact.samaris.os@gmail.com",
  license: "SPL",

  download: {
    url: "#download-placeholder",
    version: "Mountain Lake Alpha One",
    sizeGB: 1.9,
    sha256: "0000000000000000000000000000000000000000000000000000000000000000",
    releaseNotesUrl: "#release-notes-placeholder",
  },

  social: {
    github: "https://github.com/btkhaled/SamarisOS/",
    youtube: "https://www.youtube.com/@btbkhaled",
  },
} as const;

export const navLinks = [
  { to: "/", label: "Home" },
  { to: "/interface", label: "Interface" },
  { to: "/software", label: "Software" },
  { to: "/download", label: "Download" },
  { to: "/license", label: "License" },
  { to: "/business", label: "Business" },
  { to: "/faq", label: "FAQ" },
] as const;