import React from "react";

export const FinderStatusBar = React.memo(function FinderStatusBar(props: {
  itemCount: number;
  selectedCount: number;
}) {
  return (
    <div className="finder-statusbar">
      <span>{props.itemCount} item{props.itemCount !== 1 ? "s" : ""}</span>
      {props.selectedCount > 0 ? (
        <span>{props.selectedCount} selected</span>
      ) : null}
    </div>
  );
});
