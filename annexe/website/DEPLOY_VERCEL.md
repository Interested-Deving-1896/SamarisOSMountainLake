# Deploy on Vercel

This site is a Vite + React SPA, fully ready for Vercel.

## One-click deploy

1. Push the repo to GitHub.
2. On vercel.com → **Add New → Project** → import the repo.
3. Vercel auto-detects Vite. Defaults are correct:
   - **Framework**: Vite
   - **Build command**: `vite build`
   - **Output directory**: `dist`
   - **Install command**: `npm install`
4. Click **Deploy**. Done.

## What's already wired

- `vercel.json` — SPA rewrites (deep links work on refresh), long-cache immutable headers for `/assets/*` and static media, security headers (HSTS, X-Frame-Options, X-XSS-Protection, Referrer-Policy, Permissions-Policy), Content-Type for sitemap and robots.txt.
- `@vercel/analytics` — page views & custom events, mounted in `src/App.tsx`.
- `@vercel/speed-insights` — Core Web Vitals (LCP, CLS, INP) reporting.

Both Analytics and Speed Insights activate automatically once deployed on Vercel; no API key needed. Enable them in the Vercel dashboard under the project's **Analytics** and **Speed Insights** tabs.

## Custom domain

Vercel project → **Settings → Domains** → add `samaris.tech` and follow the DNS instructions.

Point your DNS to Vercel:
- **A record**: `76.76.21.21`
- **CNAME**: `cname.vercel-dns.com`

## Environment variables

None required for the current build. If you add any later (e.g. a contact form backend), declare them in Vercel → **Settings → Environment Variables** with the `VITE_` prefix to expose them to the client.

## SEO verification post-deploy

After deploying, test these URLs:
- `https://samaris.tech/sitemap.xml` — should return XML with correct Content-Type
- `https://samaris.tech/robots.txt` — should return plain text
- Google Search Console → URL Inspection → `https://samaris.tech/` → verify all SEO signals