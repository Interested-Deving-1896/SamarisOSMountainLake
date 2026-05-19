import React from "react";
import PdfPage from "./PdfPage";

type ContinuousViewerProps = {
  getPage: (n: number) => Promise<any>;
  numPages: number;
  scale: number;
  activePage: number;
  scrollRef: React.RefObject<HTMLDivElement>;
  onVisible: (pageNumber: number) => void;
};

const ContinuousViewer: React.FC<ContinuousViewerProps> = ({ getPage, numPages, scale, activePage, scrollRef, onVisible }) => {
  return (
    <div ref={scrollRef} style={{ flex: 1, overflow: "auto", padding: 24 }}>
      <div style={{ margin: "0 auto", width: "100%", maxWidth: 1240 }}>
        {Array.from({ length: numPages }, (_, i) => i + 1).map((n) => (
          <PdfPage
            key={`${n}-${scale}`}
            getPage={getPage}
            pageNumber={n}
            scale={scale}
            active={n === activePage}
            onVisible={onVisible}
          />
        ))}
      </div>
    </div>
  );
};

export default React.memo(ContinuousViewer);
