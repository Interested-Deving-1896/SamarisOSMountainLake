import { kernelClient } from "../../os/kernel/kernelClient";

export type MusicLibraryTrack = {
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
};

export type VideoLibraryItem = {
  id: string;
  path: string;
  fileName: string;
  title: string;
  format: string;
  size: number;
};

export const mediaKernel = {
  async musicLibrary() {
    const response = await kernelClient.request<MusicLibraryTrack[]>({
      type: "media.musicLibrary",
      data: {}
    });
    if (!response.data) throw new Error("media_music_library_missing");
    return response.data;
  },
  async videoLibrary() {
    const response = await kernelClient.request<VideoLibraryItem[]>({
      type: "media.videoLibrary",
      data: {}
    });
    if (!response.data) throw new Error("media_video_library_missing");
    return response.data;
  }
};
