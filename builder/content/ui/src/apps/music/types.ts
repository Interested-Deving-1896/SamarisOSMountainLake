export type MusicTrack = {
  id: string;
  path: string;
  fileName: string;
  title: string;
  artist: string;
  album: string;
  genre: string;
  year: number | null;
  durationSeconds: number;
  durationLabel: string;
  size: number;
  coverDataUrl: string | null;
  src?: string;
};

export type MusicSection = "library" | "albums" | "playlists";

export type MusicViewMode = "list" | "grid";

export type MusicSort = "title" | "artist" | "album" | "recent";

export type EqualizerPreset = "flat" | "bass-boost" | "vocal" | "acoustic" | "night";

export type RepeatMode = "off" | "all" | "one";

export type MusicPlaylist = {
  id: string;
  name: string;
  trackPaths: string[];
  createdAt: number;
  updatedAt: number;
};

export type MusicAlbum = {
  id: string;
  title: string;
  artist: string;
  year: number | null;
  coverDataUrl: string | null;
  tracks: MusicTrack[];
};
