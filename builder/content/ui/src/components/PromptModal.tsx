import React, { useEffect, useRef, useState } from "react";
import { X } from "lucide-react";
import "./promptModal.css";

export function PromptModal(props: {
  title: string;
  subtitle?: string;
  placeholder?: string;
  defaultValue?: string;
  confirmLabel?: string;
  onCancel: () => void;
  onConfirm: (value: string) => void;
}) {
  const [value, setValue] = useState(props.defaultValue || "");
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    inputRef.current?.focus();
    inputRef.current?.select();
  }, []);

  return (
    <div
      className="pm__backdrop"
      role="presentation"
      onPointerDown={(e) => {
        if (e.target === e.currentTarget) props.onCancel();
      }}
    >
      <div className="pm" role="dialog" aria-label={props.title} onPointerDown={(e) => e.stopPropagation()}>
        <div className="pm__head">
          <div className="pm__titles">
            <div className="pm__title">{props.title}</div>
            {props.subtitle ? <div className="pm__subtitle">{props.subtitle}</div> : null}
          </div>
          <button type="button" className="pm__close" aria-label="Close" onClick={props.onCancel}>
            <X size={16} strokeWidth={2.2} />
          </button>
        </div>
        <div className="pm__body">
          <input
            ref={inputRef}
            className="pm__input"
            value={value}
            placeholder={props.placeholder}
            onChange={(e) => setValue(e.target.value)}
            onKeyDown={(e) => {
              if (e.key === "Escape") props.onCancel();
              if (e.key === "Enter") props.onConfirm(value.trim());
            }}
          />
        </div>
        <div className="pm__actions">
          <button type="button" className="pm__btn pm__btn--ghost" onClick={props.onCancel}>
            Cancel
          </button>
          <button
            type="button"
            className="pm__btn pm__btn--primary"
            onClick={() => props.onConfirm(value.trim())}
            disabled={!value.trim()}
          >
            {props.confirmLabel || "OK"}
          </button>
        </div>
      </div>
    </div>
  );
}

export function ConfirmModal(props: {
  title: string;
  subtitle?: string;
  confirmLabel?: string;
  danger?: boolean;
  onCancel: () => void;
  onConfirm: () => void;
}) {
  return (
    <div
      className="pm__backdrop"
      role="presentation"
      onPointerDown={(e) => {
        if (e.target === e.currentTarget) props.onCancel();
      }}
    >
      <div className="pm" role="dialog" aria-label={props.title} onPointerDown={(e) => e.stopPropagation()}>
        <div className="pm__head">
          <div className="pm__titles">
            <div className="pm__title">{props.title}</div>
            {props.subtitle ? <div className="pm__subtitle">{props.subtitle}</div> : null}
          </div>
          <button type="button" className="pm__close" aria-label="Close" onClick={props.onCancel}>
            <X size={16} strokeWidth={2.2} />
          </button>
        </div>
        <div className="pm__actions">
          <button type="button" className="pm__btn pm__btn--ghost" onClick={props.onCancel}>
            Cancel
          </button>
          <button
            type="button"
            className={`pm__btn ${props.danger ? "pm__btn--danger" : "pm__btn--primary"}`}
            onClick={props.onConfirm}
          >
            {props.confirmLabel || "Confirm"}
          </button>
        </div>
      </div>
    </div>
  );
}

export function InfoModal(props: {
  title: string;
  subtitle?: string;
  onClose: () => void;
  children: React.ReactNode;
}) {
  return (
    <div
      className="pm__backdrop"
      role="presentation"
      onPointerDown={(e) => {
        if (e.target === e.currentTarget) props.onClose();
      }}
    >
      <div className="pm" role="dialog" aria-label={props.title} onPointerDown={(e) => e.stopPropagation()}>
        <div className="pm__head">
          <div className="pm__titles">
            <div className="pm__title">{props.title}</div>
            {props.subtitle ? <div className="pm__subtitle">{props.subtitle}</div> : null}
          </div>
          <button type="button" className="pm__close" aria-label="Close" onClick={props.onClose}>
            <X size={16} strokeWidth={2.2} />
          </button>
        </div>
        <div className="pm__body">{props.children}</div>
        <div className="pm__actions">
          <button type="button" className="pm__btn pm__btn--primary" onClick={props.onClose}>
            Done
          </button>
        </div>
      </div>
    </div>
  );
}
