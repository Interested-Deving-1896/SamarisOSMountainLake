import React from "react";
import { archiveKernel, type ArchiveEntry } from "../../../services/kernel/archive";

export function useArchive(archivePath?: string) {
  const [entries, setEntries] = React.useState<ArchiveEntry[]>([]);
  const [loading, setLoading] = React.useState(false);
  const [extracting, setExtracting] = React.useState(false);
  const [error, setError] = React.useState<string | null>(null);
  const [successNotice, setSuccessNotice] = React.useState<string | null>(null);
  const [destination, setDestination] = React.useState("");
  const [archiveName, setArchiveName] = React.useState("");

  React.useEffect(() => {
    if (!archivePath) return;
    let cancelled = false;

    const parts = archivePath.split("/").filter(Boolean);
    setArchiveName(parts[parts.length - 1] || "Archive");
    setDestination(`/User/Downloads/${parts[parts.length - 1]?.replace(/\.(zip|tar|gz|tgz|bz2|7z)(\..*)?$/i, "") || "extracted"}`);

    const ap = archivePath;
    async function load() {
      if (!ap) return;
      setLoading(true);
      setError(null);
      try {
        const result = await archiveKernel.list(ap);
        if (!cancelled) setEntries(result);
      } catch (err) {
        if (!cancelled) setError(err instanceof Error ? err.message : "Failed to list archive");
      } finally {
        if (!cancelled) setLoading(false);
      }
    }

    void load();
    return () => { cancelled = true; };
  }, [archivePath]);

  const doExtract = React.useCallback(async () => {
    if (!archivePath || !destination) return;
    const ap = archivePath;
    const dest = destination;
    setExtracting(true);
    setError(null);
    setSuccessNotice(null);
    try {
      const result = await archiveKernel.extract(ap, dest);
      const fileCount = result.files?.length || 0;
      setSuccessNotice(
        fileCount > 0
          ? `Extracted ${fileCount} file${fileCount > 1 ? "s" : ""} to ${dest}`
          : `Extraction complete to ${dest}`
      );
    } catch (err) {
      setError(err instanceof Error ? err.message : "Extraction failed");
    } finally {
      setExtracting(false);
    }
  }, [archivePath, destination]);

  return { entries, loading, extracting, error, successNotice, archiveName, destination, doExtract };
}
