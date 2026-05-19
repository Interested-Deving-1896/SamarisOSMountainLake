# PDF Viewer

High-performance PDF viewer with **continuous scroll**, **lazy page rendering**, **thumbnail sidebar**, and **full-text search**.

<br>

## Features

| Feature | Detail |
|---------|--------|
| **Continuous scroll** | All pages in a vertical scrollable stream |
| **Lazy rendering** | IntersectionObserver with 700px margin — pages render on approach |
| **Thumbnails** | Sidebar with canvas previews (200px margin, 150px max height) |
| **Search** | Across all pages with debounced input (300ms), result count and navigation |
| **Zoom** | 50% to 200% via toolbar buttons or slider |
| **Page nav** | Direct page number input + scroll position tracking |
| **Rotate** | Clockwise/counter-clockwise per-page rotation |

<br>

## Architecture

```
pdf-viewer/
├── index.tsx (entry, osStore → path → FS → pdfjs)
├── hooks/
│   ├── usePdfDocument.ts (load PDF + progress)
│   └── usePdfSearch.ts (text search across pages)
├── components/
│   ├── ContinuousViewer (scrollable page list)
│   ├── PdfPage (IntersectionObserver lazy render)
│   ├── PageSidebar (search + thumbnails)
│   ├── PageThumbnail (canvas preview)
│   └── ViewerHeader (zoom controls, page input, rotate)
```

Uses **pdfjs-dist v3.11.174** for rendering (compatible with Electron 28). The entire PDF is loaded as a data URL from the kernel filesystem via `FsService.readDataUrl()`.

<br>

## Related

- [Filesystem API](../apis/fs-api.md)
- [VOLT Architecture — Kernel Filesystem Service](../architecture/volt-filesystem.md)

<br>

---

[← Back: Documentation Index](../index.md)
