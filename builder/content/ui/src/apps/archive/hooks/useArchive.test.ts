import { describe, it, expect, vi, beforeEach } from "vitest";
import { renderHook, act, waitFor } from "@testing-library/react";
import { useArchive } from "./useArchive";

vi.mock("../../../services/kernel/archive", () => ({
  archiveKernel: {
    list: vi.fn(),
    extract: vi.fn()
  }
}));

import { archiveKernel } from "../../../services/kernel/archive";

beforeEach(() => {
  vi.clearAllMocks();
});

describe("useArchive", () => {
  it("returns defaults when no archivePath provided", () => {
    const { result } = renderHook(() => useArchive(undefined));
    expect(result.current.entries).toEqual([]);
    expect(result.current.loading).toBe(false);
    expect(result.current.extracting).toBe(false);
    expect(result.current.error).toBeNull();
    expect(result.current.successNotice).toBeNull();
  });

  it("loads entries on archivePath change", async () => {
    vi.mocked(archiveKernel.list).mockResolvedValue([
      { name: "file.txt", kind: "file", size: 100 },
      { name: "subdir", kind: "dir" }
    ]);

    const { result } = renderHook(() => useArchive("/path/to/archive.zip"));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.entries).toHaveLength(2);
    expect(result.current.entries[0].name).toBe("file.txt");
    expect(result.current.archiveName).toBe("archive.zip");
  });

  it("sets error on list failure", async () => {
    vi.mocked(archiveKernel.list).mockRejectedValue(new Error("Read error"));

    const { result } = renderHook(() => useArchive("/bad.zip"));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    expect(result.current.error).toBe("Read error");
    expect(result.current.entries).toEqual([]);
  });

  it("extracts and shows success notice", async () => {
    vi.mocked(archiveKernel.list).mockResolvedValue([]);
    vi.mocked(archiveKernel.extract).mockResolvedValue({ ok: true, path: "/dest", files: ["/dest/f1.txt", "/dest/f2.txt"] });

    const { result } = renderHook(() => useArchive("/test.zip"));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    await act(async () => {
      await result.current.doExtract();
    });

    expect(result.current.successNotice).toContain("Extracted 2 files");
  });

  it("sets error on extraction failure", async () => {
    vi.mocked(archiveKernel.list).mockResolvedValue([]);
    vi.mocked(archiveKernel.extract).mockRejectedValue(new Error("Disk full"));

    const { result } = renderHook(() => useArchive("/test.zip"));

    await waitFor(() => {
      expect(result.current.loading).toBe(false);
    });

    await act(async () => {
      await result.current.doExtract();
    });

    expect(result.current.error).toBe("Disk full");
  });

  it("forms destination path from archive name", () => {
    const { result } = renderHook(() => useArchive("/User/Downloads/backup.tar.gz"));

    expect(result.current.destination).toBe("/User/Downloads/backup");
    expect(result.current.archiveName).toBe("backup.tar.gz");
  });

  it("handles .tgz double extension in destination", () => {
    const { result } = renderHook(() => useArchive("/path/data.tgz"));
    expect(result.current.destination).toBe("/User/Downloads/data");
  });
});
