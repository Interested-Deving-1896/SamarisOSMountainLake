# Photos

Image viewer with **gallery browsing**, **thumbnail generation**, **slideshow mode**, and **zoom/pan**.

<br>

## Features

- Grid view of images in a directory with lazy-loaded thumbnails
- Click to open full-size with zoom and pan
- Thumbnail generation via canvas resize with LRU cache (max 200 entries)
- Supports PNG, JPG, GIF, WebP, BMP, SVG
- Keyboard navigation (arrow keys between images)
- Fullscreen mode
- EXIF metadata display (date, camera, ISO, aperture, shutter speed)
- Basic image operations (rotate, delete)

<br>

## Architecture

```
PhotosApp (React)
├── PhotoGrid (thumbnail gallery with IntersectionObserver lazy load)
├── PhotoViewer (full-size image with zoom/pan controls)
├── PhotoToolbar (nav arrows, zoom, rotate, delete, fullscreen)
├── MetadataPanel (EXIF data display)
└── SlideshowPlayer (auto-advance with configurable interval)
```

Uses the `getThumbnail()` function from `fileThumbnails.ts` which caches resized versions (max 200 entries, 256px max dimension) and renders them via `<canvas>` drawImage.

<br>

## Related

- [Filesystem API](../apis/fs-api.md)
- [VOLT Architecture — Thumbnail Service](../architecture/volt-thumbnails.md)

<br>

---

[← Back: Documentation Index](../index.md)
