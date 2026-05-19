import React from "react";
import { Disc3, Edit3, LibraryBig, ListMusic, Plus, Sparkles, Trash2, Waves, type LucideIcon } from "lucide-react";
import type { MusicPlaylist, MusicSection } from "../types";

const sections: Array<{ id: MusicSection; label: string; icon: LucideIcon }> = [
  { id: "library", label: "Songs", icon: LibraryBig },
  { id: "albums", label: "Albums", icon: Disc3 },
  { id: "playlists", label: "Playlists", icon: ListMusic }
];

export function MusicSidebar(props: {
  activeSection: MusicSection;
  onSelectSection: (section: MusicSection) => void;
  trackCount: number;
  playlists: MusicPlaylist[];
  selectedPlaylistId: string | null;
  selectedPlaylistName?: string;
  onSelectPlaylist: (playlistId: string) => void;
  onCreatePlaylist: () => void;
  onDeletePlaylist: (playlistId: string) => void;
  onRenamePlaylist: (playlistId: string) => void;
  onOpenPlayer: () => void;
  canDeletePlaylist: boolean;
}) {
  return (
    <aside className="music__sidebar">
      <div className="music__brand">
        <div className="music__brandMark">Music</div>
        <div className="music__brandMeta">{props.trackCount} tracks synced from ~/Music</div>
      </div>

      <div className="music__nav">
        {sections.map((section) => {
          const Icon = section.icon;
          return (
            <button
              key={section.id}
              type="button"
              className={`music__navItem ${props.activeSection === section.id ? "music__navItem--active" : ""}`}
              onClick={() => props.onSelectSection(section.id)}
            >
              <Icon size={16} strokeWidth={2.15} />
              <span>{section.label}</span>
            </button>
          );
        })}
      </div>

      <div className="music__playlistPanel">
        <div className="music__sectionHead">
          <span>Playlists</span>
          <button type="button" className="music__sectionAction" onClick={props.onCreatePlaylist} aria-label="Create playlist">
            <Plus size={14} strokeWidth={2.25} />
          </button>
        </div>
        <div className="music__playlistList">
          {props.playlists.map((playlist) => (
            <button
              key={playlist.id}
              type="button"
              className={`music__playlistItem ${props.selectedPlaylistId === playlist.id ? "music__playlistItem--active" : ""}`}
              onClick={() => props.onSelectPlaylist(playlist.id)}
            >
              <ListMusic size={14} strokeWidth={2.1} />
              <span>{playlist.name}</span>
            </button>
          ))}
          {!props.playlists.length ? <div className="music__playlistEmpty">No playlists yet.</div> : null}
        </div>
        {props.canDeletePlaylist && props.selectedPlaylistId ? (
          <>
            <button
              type="button"
              className="music__playlistAction"
              onClick={() => props.onRenamePlaylist(props.selectedPlaylistId!)}
            >
              <Edit3 size={14} strokeWidth={2.2} />
              <span>Rename “{props.selectedPlaylistName}”</span>
            </button>
            <button
              type="button"
              className="music__playlistDelete"
              onClick={() => props.onDeletePlaylist(props.selectedPlaylistId!)}
            >
              <Trash2 size={14} strokeWidth={2.2} />
              <span>Delete “{props.selectedPlaylistName}”</span>
            </button>
          </>
        ) : null}
      </div>

      <button type="button" className="music__sidebarPlayer" onClick={props.onOpenPlayer}>
        <div className="music__sidebarPlayerGlyph">
          <Waves size={18} strokeWidth={2.2} />
        </div>
        <div>
          <div className="music__sidebarPlayerTitle">Player</div>
          <div className="music__sidebarPlayerMeta">Open the immersive full player.</div>
        </div>
      </button>

      <div className="music__sidebarTip">
        <Sparkles size={14} strokeWidth={2.2} />
        <span>Drop audio files anywhere in Music to import them instantly.</span>
      </div>
    </aside>
  );
}
