const thumbnailCache = new Map<string, string>();
const CACHE_MAX = 200;

function getCacheKey(path: string, size: number): string {
  return `${path}@${size}`;
}

export async function getThumbnail(path: string, size: number, fs: { readDataUrl: (p: string) => Promise<{ dataUrl: string }> }): Promise<string | null> {
  const key = getCacheKey(path, size);
  const cached = thumbnailCache.get(key);
  if (cached) return cached;

  const lower = path.toLowerCase();
  try {
    if (lower.match(/\.(png|jpg|jpeg|gif|webp)$/)) {
      const { dataUrl } = await fs.readDataUrl(path);
      const thumb = await resizeImage(dataUrl, size);
      if (thumb) {
        setCache(key, thumb);
        return thumb;
      }
    }
  } catch {}
  return null;
}

function resizeImage(dataUrl: string, size: number): Promise<string | null> {
  return new Promise((resolve) => {
    const img = new Image();
    img.crossOrigin = "anonymous";
    img.onload = () => {
      try {
        const canvas = document.createElement("canvas");
        const ratio = Math.min(size / img.width, size / img.height);
        canvas.width = Math.round(img.width * ratio);
        canvas.height = Math.round(img.height * ratio);
        const ctx = canvas.getContext("2d");
        if (!ctx) { resolve(null); return; }
        ctx.imageSmoothingEnabled = true;
        ctx.imageSmoothingQuality = "high";
        ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
        resolve(canvas.toDataURL("image/png"));
      } catch {
        resolve(null);
      }
    };
    img.onerror = () => resolve(null);
    img.src = dataUrl;
  });
}

function setCache(key: string, value: string) {
  if (thumbnailCache.size >= CACHE_MAX) {
    const firstKey = thumbnailCache.keys().next().value;
    if (firstKey) thumbnailCache.delete(firstKey);
  }
  thumbnailCache.set(key, value);
}

export function clearThumbnailCache() {
  thumbnailCache.clear();
}
