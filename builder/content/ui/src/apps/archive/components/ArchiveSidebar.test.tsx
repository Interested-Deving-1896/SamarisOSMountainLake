import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { ArchiveSidebar } from "./ArchiveSidebar";

describe("ArchiveSidebar", () => {
  it("shows archive name and destination", () => {
    render(<ArchiveSidebar name="backup.tar.gz" destination="/User/Downloads/backup" />);

    expect(screen.getByText("backup.tar.gz")).toBeDefined();
    expect(screen.getByText("/User/Downloads/backup")).toBeDefined();
    expect(screen.getByText("Archive")).toBeDefined();
    expect(screen.getByText("Extract to")).toBeDefined();
  });
});
