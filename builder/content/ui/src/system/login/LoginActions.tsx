import React from "react";
import { ArrowRight } from "lucide-react";
import { PasswordDots } from "./PasswordDots";

export function LoginActions(props: {
  password: string;
  placeholder?: string;
  busy?: boolean;
  error?: string;
  helper?: string;
  onChangePassword: (value: string) => void;
  onSubmit: () => void;
}) {
  const [focused, setFocused] = React.useState(false);
  const inputRef = React.useRef<HTMLInputElement | null>(null);

  return (
    <div className="samaris-login__passwordZone">
      <div className="samaris-login__line" />
      <label
        className="samaris-login__passwordLabel"
        onClick={() => inputRef.current?.focus()}
        aria-label="Password"
      >
        <PasswordDots count={props.password.length} focused={focused} />
        <input
          ref={inputRef}
          className="samaris-login__passwordInput"
          type="password"
          name="samaris-session-passcode"
          value={props.password}
          placeholder={props.placeholder || "Password"}
          autoComplete="off"
          autoCorrect="off"
          autoCapitalize="none"
          spellCheck={false}
          data-form-type="other"
          onFocus={() => setFocused(true)}
          onBlur={() => setFocused(false)}
          onChange={(event) => props.onChangePassword(event.target.value)}
          onKeyDown={(event) => {
            if (event.key === "Enter") props.onSubmit();
          }}
        />
      </label>
      <div className="samaris-login__line" />

      <button
        type="button"
        className="samaris-login__submit"
        aria-label="Enter Samaris"
        disabled={props.busy}
        onClick={props.onSubmit}
      >
        <ArrowRight size={24} strokeWidth={2.2} />
      </button>

      {props.helper ? <div className="samaris-login__helper">{props.helper}</div> : null}
      {props.error ? <div className="samaris-login__error">{props.error}</div> : null}
    </div>
  );
}
