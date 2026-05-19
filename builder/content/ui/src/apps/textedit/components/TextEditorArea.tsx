import React from "react";

export function TextEditorArea(props: {
  value: string;
  loading: boolean;
  onChange: (value: string) => void;
}) {
  return (
    <textarea
      className="textedit__area"
      value={props.value}
      disabled={props.loading}
      onChange={(event) => props.onChange(event.target.value)}
      spellCheck={false}
    />
  );
}
