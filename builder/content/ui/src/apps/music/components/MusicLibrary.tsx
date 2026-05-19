import React from "react";
import { Album, Disc3, Music4, Trash2 } from "lucide-react";
import { List } from "react-window";
import type { MusicAlbum, MusicSection, MusicTrack, MusicViewMode } from "../types";
import { startFileDrag } from "../../../os/filesystem/dragDrop";

const ROW_HEIGHT = 52;

type TrackListRowProps = {
  index: number;
  style: React.CSSProperties;
  tracks: MusicTrack[];
  activeTrackId: string | null;
  section: MusicSection;
  onSelectTrack: (id: string) => void;
  onPlayTrack: (id: string) => void;
  onRemoveFromPlaylist: (path: string) => void;
};

function TrackListRow({ index, style, tracks, activeTrackId, section, onSelectTrack, onPlayTrack, onRemoveFromPlaylist }: TrackListRowProps) {
  const track = tracks[index];
  if (!track) return <div style={style} />;

  return (
    <div
      style={style}
      role="listitem"
      draggable
      className={`music__row ${activeTrackId === track.id ? "music__row--active" : ""}`}
      onDragStart={(e) => {
        startFileDrag(e.dataTransfer, [{
          name: track.fileName,
          path: track.path,
          kind: "file",
          size: track.size
        }]);
      }}
    >
      <button type="button" className="music__rowMain" onClick={() => onSelectTrack(track.id)} onDoubleClick={() => onPlayTrack(track.id)}>
        <div className="music__titleCell">
          <span className="music__trackGlyph" aria-hidden="true">
            {track.coverDataUrl ? (
              <img src={track.coverDataUrl} alt="" className="music__trackGlyphImage" />
            ) : (
              <Music4 size={14} strokeWidth={2.15} />
            )}
          </span>
          <span className="music__trackTitle">{track.title}</span>
        </div>
        <span>{track.artist}</span>
        <span>{track.album}</span>
        <span className="music__timeCell">{track.durationLabel}</span>
      </button>
      {section === "playlists" ? (
        <button
          type="button"
          className="music__rowAction"
          aria-label="Remove from playlist"
          onClick={() => onRemoveFromPlaylist(track.path)}
        >
          <Trash2 size={14} strokeWidth={2.2} />
        </button>
      ) : null}
    </div>
  );
}

export function MusicLibrary(props: {
  tracks: MusicTrack[];
  albums: MusicAlbum[];
  section: MusicSection;
  loading: boolean;
  importing: boolean;
  viewMode: MusicViewMode;
  activeTrackId: string | null;
  selectedAlbumId: string | null;
  onSelectTrack: (trackId: string) => void;
  onPlayTrack: (trackId: string) => void;
  onSelectAlbum: (albumId: string) => void;
  onBackFromAlbum: () => void;
  onRemoveFromPlaylist: (trackPath: string) => void;
  emptyLabel: string;
  importNotice: string | null;
}) {
  const listBodyRef = React.useRef<HTMLDivElement | null>(null);
  const [listHeight, setListHeight] = React.useState(400);

  React.useEffect(() => {
    const el = listBodyRef.current;
    if (!el) return;
    const observer = new ResizeObserver((entries) => {
      for (const entry of entries) {
        setListHeight(entry.contentRect.height);
      }
    });
    observer.observe(el);
    return () => observer.disconnect();
  }, []);

  const rowData = React.useMemo(() => ({
    tracks: props.tracks,
    activeTrackId: props.activeTrackId,
    section: props.section,
    onSelectTrack: props.onSelectTrack,
    onPlayTrack: props.onPlayTrack,
    onRemoveFromPlaylist: props.onRemoveFromPlaylist,
  } as any), [props.tracks, props.activeTrackId, props.section, props.onSelectTrack, props.onPlayTrack, props.onRemoveFromPlaylist]);

  if (props.section === "albums" && !props.selectedAlbumId) {
    return (
      <section className="music__library">
        <div className="music__albumsGrid">
          {props.albums.map((album) => (
            <button
              key={album.id}
              type="button"
              className="music__albumCard"
              onClick={() => props.onSelectAlbum(album.id)}
            >
              <div className="music__albumCover">
                {album.coverDataUrl ? (
                  <img src={album.coverDataUrl} alt="" className="music__albumCoverImage" />
                ) : (
                  <Album size={28} strokeWidth={2.1} />
                )}
              </div>
              <div className="music__albumTitle">{album.title}</div>
              <div className="music__albumMeta">
                {album.artist}
                {album.year ? ` • ${album.year}` : ""}
              </div>
              <div className="music__albumMeta">{album.tracks.length} tracks</div>
            </button>
          ))}
          {!props.loading && props.albums.length === 0 ? <div className="music__empty">{props.emptyLabel}</div> : null}
        </div>
      </section>
    );
  }

  return (
    <section className="music__library">
      <div className="music__libraryHead">
        <span>{props.section === "albums" ? "Album track list" : "Title"}</span>
        <span>Artist</span>
        <span>Album</span>
        <span>Time</span>
      </div>
      {props.section === "albums" && props.selectedAlbumId ? (
        <div className="music__albumBackRow">
          <button type="button" className="music__albumBackBtn" onClick={props.onBackFromAlbum}>
            Back to albums
          </button>
        </div>
      ) : null}
      <div
        ref={listBodyRef}
        className={`music__libraryBody ${props.viewMode === "grid" ? "music__libraryBody--grid" : ""}`}
        role="list"
      >
        {props.loading ? <div className="music__empty">Loading your music library…</div> : null}
        {!props.loading && props.importing ? <div className="music__empty">Importing tracks into Samaris Music…</div> : null}
        {!props.loading && !props.importing && props.tracks.length === 0 && !props.importNotice ? <div className="music__empty">{props.emptyLabel}</div> : null}
        {props.importNotice ? <div className="music__importNotice">{props.importNotice}</div> : null}
        {props.viewMode === "grid"
          ? props.tracks.map((track) => (
              <button
                key={track.id}
                type="button"
                role="listitem"
                draggable
                className={`music__tile ${props.activeTrackId === track.id ? "music__tile--active" : ""}`}
                onClick={() => props.onSelectTrack(track.id)}
                onDoubleClick={() => props.onPlayTrack(track.id)}
                onDragStart={(e) => {
                  startFileDrag(e.dataTransfer, [{
                    name: track.fileName,
                    path: track.path,
                    kind: "file",
                    size: track.size
                  }]);
                }}
              >
                <div className="music__tileCover">
                  {track.coverDataUrl ? (
                    <img src={track.coverDataUrl} alt="" className="music__tileCoverImage" />
                  ) : (
                    <Disc3 size={24} strokeWidth={2.1} />
                  )}
                </div>
                <div className="music__tileTitle">{track.title}</div>
                <div className="music__tileMeta">{track.artist}</div>
                <div className="music__tileMeta">{track.album}</div>
              </button>
            ))
          : !props.loading && !props.importing && props.tracks.length > 0 ? (
              <List
                defaultHeight={listHeight}
                rowCount={props.tracks.length}
                rowHeight={ROW_HEIGHT}
                rowComponent={TrackListRow}
                rowProps={rowData}
              />
            ) : null}
      </div>
    </section>
  );
}
