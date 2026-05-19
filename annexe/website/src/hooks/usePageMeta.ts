import { useEffect } from "react";

interface PageMeta {
  title?: string;
  description?: string;
  keywords?: string;
  ogTitle?: string;
  ogDescription?: string;
  ogImage?: string;
  canonicalPath?: string;
}

const BASE_URL = "https://samaris.tech";
const BASE_TITLE = "Samaris OS";
const BASE_DESCRIPTION =
  "Samaris OS is an experimental operating system built for calm, local-first computing — portable, offline-friendly, and designed to stay under your control, on a USB stick.";
const BASE_KEYWORDS =
  "Samaris OS, portable operating system, local-first OS, privacy-first OS, Linux USB, sovereign computing, offline operating system";

export function usePageMeta(meta: PageMeta = {}) {
  const {
    title,
    description = BASE_DESCRIPTION,
    keywords,
    ogTitle,
    ogDescription = BASE_DESCRIPTION,
    ogImage,
    canonicalPath = "",
  } = meta;

  const fullTitle = title ? `${title} — ${BASE_TITLE}` : `${BASE_TITLE} — A sovereign portable operating system`;
  const canonicalUrl = canonicalPath ? `${BASE_URL}${canonicalPath}` : `${BASE_URL}/`;
  const keywordsWithBase = keywords
    ? `${keywords}, ${BASE_KEYWORDS}`
    : BASE_KEYWORDS;
  const resolvedOgImage = ogImage || "/samaris-logo.png";
  const resolvedOgImageUrl = resolvedOgImage.startsWith("http")
    ? resolvedOgImage
    : `${BASE_URL}${resolvedOgImage}`;

  useEffect(() => {
    const updateMeta = (name: string, value: string, property?: boolean) => {
      if (property) {
        document.querySelector<HTMLMetaElement>(`meta[property="${name}"]`)?.setAttribute("content", value);
      } else {
        document.querySelector<HTMLMetaElement>(`meta[name="${name}"]`)?.setAttribute("content", value);
      }
    };

    const setTitle = (t: string) => {
      document.title = t;
    };

    const setCanonical = (href: string) => {
      let link = document.querySelector<HTMLLinkElement>('link[rel="canonical"]');
      if (!link) {
        link = document.createElement("link");
        link.rel = "canonical";
        document.head.appendChild(link);
      }
      link.href = href;
    };

    setTitle(fullTitle);
    updateMeta("description", description, false);
    updateMeta("keywords", keywordsWithBase, false);
    updateMeta("og:title", ogTitle || fullTitle, true);
    updateMeta("og:description", ogDescription, true);
    updateMeta("og:image", resolvedOgImageUrl, true);
    updateMeta("og:url", canonicalUrl, true);
    updateMeta("twitter:title", ogTitle || fullTitle, false);
    updateMeta("twitter:description", ogDescription, false);
    updateMeta("twitter:image", resolvedOgImageUrl, false);
    setCanonical(canonicalUrl);

    return () => {
      setTitle(BASE_TITLE);
      updateMeta("description", BASE_DESCRIPTION, false);
      updateMeta("keywords", BASE_KEYWORDS, false);
    };
  }, [fullTitle, description, keywordsWithBase, ogTitle, ogDescription, resolvedOgImageUrl, canonicalUrl]);
}
