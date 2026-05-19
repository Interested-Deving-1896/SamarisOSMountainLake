import React from "react";

export type SamarisToastItem = {
  id: string;
  title: string;
  message?: string;
};

export function ToastStack(props: {
  toasts: SamarisToastItem[];
  className?: string;
}) {
  return (
    <div className={`samaris-toast-stack${props.className ? ` ${props.className}` : ""}`} aria-live="polite">
      {props.toasts.map((toast) => (
        <div key={toast.id} className="samaris-toast" role="status">
          <b>{toast.title}</b>
          {toast.message ? <span>{toast.message}</span> : null}
        </div>
      ))}
    </div>
  );
}
