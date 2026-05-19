import { describe, it, expect } from "vitest";
import { render, screen } from "@testing-library/react";
import { MusicLibrary } from "./MusicLibrary";
import type { MusicTrack, MusicAlbum } from "../types";

function makeTrack(id: string, overrides: Partial<MusicTrack> = {}): MusicTrack {
  return {
    id,
    path: `/path/${id}.mp3`,
    fileName: `${id}.mp3`,
    title: `Song ${id}`,
    artist: "Artist",
    album: "Album",
    genre: "Pop",
    year: 2024,
    durationSeconds: 180,
    durationLabel: "3:00",
    size: 4000,
    coverDataUrl: null,
    ...overrides
  };
}

const tracks = [makeTrack("a"), makeTrack("b"), makeTrack("c")];
const albums: MusicAlbum[] = [];

describe("MusicLibrary", () => {
  it("shows loading state", () => {
    render(
      <MusicLibrary
        tracks={[]}
        albums={[]}
        section="library"
        loading={true}
        importing={false}
        viewMode="list"
        activeTrackId={null}
        selectedAlbumId={null}
        onSelectTrack={() => {}}
        onPlayTrack={() => {}}
        onSelectAlbum={() => {}}
        onBackFromAlbum={() => {}}
        onRemoveFromPlaylist={() => {}}
        emptyLabel="No tracks yet"
        importNotice={null}
      />
    );
    expect(screen.getByText("Loading your music library…")).toBeDefined();
  });

  it("shows importing state", () => {
    render(
      <MusicLibrary
        tracks={[]}
        albums={[]}
        section="library"
        loading={false}
        importing={true}
        viewMode="list"
        activeTrackId={null}
        selectedAlbumId={null}
        onSelectTrack={() => {}}
        onPlayTrack={() => {}}
        onSelectAlbum={() => {}}
        onBackFromAlbum={() => {}}
        onRemoveFromPlaylist={() => {}}
        emptyLabel="No tracks yet"
        importNotice={null}
      />
    );
    expect(screen.getByText("Importing tracks into Samaris Music…")).toBeDefined();
  });

  it("shows empty state when no tracks", () => {
    render(
      <MusicLibrary
        tracks={[]}
        albums={[]}
        section="library"
        loading={false}
        importing={false}
        viewMode="list"
        activeTrackId={null}
        selectedAlbumId={null}
        onSelectTrack={() => {}}
        onPlayTrack={() => {}}
        onSelectAlbum={() => {}}
        onBackFromAlbum={() => {}}
        onRemoveFromPlaylist={() => {}}
        emptyLabel="No tracks yet"
        importNotice={null}
      />
    );
    expect(screen.getByText("No tracks yet")).toBeDefined();
  });

  it("shows tracks in list view", () => {
    render(
      <MusicLibrary
        tracks={tracks}
        albums={[]}
        section="library"
        loading={false}
        importing={false}
        viewMode="list"
        activeTrackId={null}
        selectedAlbumId={null}
        onSelectTrack={() => {}}
        onPlayTrack={() => {}}
        onSelectAlbum={() => {}}
        onBackFromAlbum={() => {}}
        onRemoveFromPlaylist={() => {}}
        emptyLabel="No tracks yet"
        importNotice={null}
      />
    );
    expect(screen.getByText("Song a")).toBeDefined();
    expect(screen.getByText("Song b")).toBeDefined();
    expect(screen.getByText("Song c")).toBeDefined();
  });

  it("shows album grid when section is albums", () => {
    const testAlbums: MusicAlbum[] = [{
      id: "album-1",
      title: "Test Album",
      artist: "Test Artist",
      year: 2024,
      coverDataUrl: null,
      tracks: [tracks[0]]
    }];

    render(
      <MusicLibrary
        tracks={tracks}
        albums={testAlbums}
        section="albums"
        loading={false}
        importing={false}
        viewMode="list"
        activeTrackId={null}
        selectedAlbumId={null}
        onSelectTrack={() => {}}
        onPlayTrack={() => {}}
        onSelectAlbum={() => {}}
        onBackFromAlbum={() => {}}
        onRemoveFromPlaylist={() => {}}
        emptyLabel="No tracks yet"
        importNotice={null}
      />
    );
    expect(screen.getByText("Test Album")).toBeDefined();
  });

  it("shows import notice when provided", () => {
    render(
      <MusicLibrary
        tracks={tracks}
        albums={[]}
        section="library"
        loading={false}
        importing={false}
        viewMode="list"
        activeTrackId={null}
        selectedAlbumId={null}
        onSelectTrack={() => {}}
        onPlayTrack={() => {}}
        onSelectAlbum={() => {}}
        onBackFromAlbum={() => {}}
        onRemoveFromPlaylist={() => {}}
        emptyLabel="No tracks yet"
        importNotice="5 files imported"
      />
    );
    expect(screen.getByText("5 files imported")).toBeDefined();
  });
});
