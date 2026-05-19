import { useEffect, useState } from "react";
import * as pdfjsLib from "pdfjs-dist";
import workerUrl from "pdfjs-dist/build/pdf.worker.min?url";

pdfjsLib.GlobalWorkerOptions.workerSrc = workerUrl;

export function usePdfDocument(dataUrl: string) {
  const [doc, setDoc] = useState<pdfjsLib.PDFDocumentProxy | null>(null);
  const [numPages, setNumPages] = useState(0);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState<string | null>(null);
  const [progress, setProgress] = useState(0);

  useEffect(() => {
    if (!dataUrl) return;
    let cancelled = false;

    setDoc(null);
    setNumPages(0);
    setLoading(true);
    setError(null);
    setProgress(0);

    (async () => {
      try {
        const loadingTask = pdfjsLib.getDocument(dataUrl);
        loadingTask.onProgress = (data: { loaded: number; total: number }) => {
          if (data.total > 0) setProgress(Math.round((data.loaded / data.total) * 100));
        };
        const pdf = await loadingTask.promise;
        if (cancelled) return;
        setDoc(pdf);
        setNumPages(pdf.numPages);
      } catch (err) {
        if (cancelled) return;
        setError(err instanceof Error ? err.message : "Failed to load PDF.");
      } finally {
        if (!cancelled) setLoading(false);
      }
    })();

    return () => { cancelled = true; };
  }, [dataUrl]);

  return { doc, numPages, loading, error, progress };
}
