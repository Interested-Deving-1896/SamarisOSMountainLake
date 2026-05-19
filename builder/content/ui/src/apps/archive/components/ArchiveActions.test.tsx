import { describe, it, expect, vi } from "vitest";
import { render, screen, fireEvent } from "@testing-library/react";
import { ArchiveActions } from "./ArchiveActions";

describe("ArchiveActions", () => {
  it("renders archive filename and extract button", () => {
    render(
      <ArchiveActions archivePath="/path/to/archive.zip" destination="/dest" onExtract={vi.fn()} extracting={false} />
    );

    expect(screen.getByText("archive.zip")).toBeDefined();
    expect(screen.getByText("Extract All")).toBeDefined();
  });

  it("disables button during extraction", () => {
    render(
      <ArchiveActions archivePath="/test.zip" destination="/dest" onExtract={vi.fn()} extracting={true} />
    );

    const button = screen.getByRole("button");
    expect(button).toBeDisabled();
    expect(screen.getByText("Extracting…")).toBeDefined();
  });

  it("calls onExtract on click", () => {
    const onExtract = vi.fn();
    render(
      <ArchiveActions archivePath="/test.zip" destination="/dest" onExtract={onExtract} extracting={false} />
    );

    fireEvent.click(screen.getByRole("button"));
    expect(onExtract).toHaveBeenCalledOnce();
  });
});
