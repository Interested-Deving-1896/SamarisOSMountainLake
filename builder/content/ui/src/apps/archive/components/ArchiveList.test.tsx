import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { ArchiveList } from "./ArchiveList";
import type { ArchiveEntry } from "../../../services/kernel/archive";

describe("ArchiveList", () => {
  it("shows loading spinner", () => {
    render(<ArchiveList entries={[]} loading={true} error={null} successNotice={null} />);
    expect(screen.getByText("Reading archive…")).toBeDefined();
  });

  it("shows error message", () => {
    render(<ArchiveList entries={[]} loading={false} error="Unsupported format" successNotice={null} />);
    expect(screen.getByText("Unsupported format")).toBeDefined();
  });

  it("shows empty message when no entries", () => {
    render(<ArchiveList entries={[]} loading={false} error={null} successNotice={null} />);
    expect(screen.getByText("Archive is empty.")).toBeDefined();
  });

  it("shows success notice when provided", () => {
    render(<ArchiveList entries={[]} loading={false} error={null} successNotice="Extracted 5 files to /dest" />);
    expect(screen.getByText("Extracted 5 files to /dest")).toBeDefined();
  });

  it("renders file and directory entries", () => {
    const entries: ArchiveEntry[] = [
      { name: "readme.md", kind: "file", size: 2048 },
      { name: "src", kind: "dir" },
      { name: "index.js", kind: "file", size: 512 }
    ];

    render(<ArchiveList entries={entries} loading={false} error={null} successNotice={null} />);

    expect(screen.getByText("readme.md")).toBeDefined();
    expect(screen.getByText("src")).toBeDefined();
    expect(screen.getByText("index.js")).toBeDefined();
  });

  it("shows size labels for files", () => {
    const entries: ArchiveEntry[] = [
      { name: "small.txt", kind: "file", size: 512 },
      { name: "large.bin", kind: "file", size: 3 * 1024 * 1024 }
    ];

    render(<ArchiveList entries={entries} loading={false} error={null} successNotice={null} />);

    expect(screen.getByText("1 KB")).toBeDefined();
    expect(screen.getByText("3.0 MB")).toBeDefined();
  });
});
