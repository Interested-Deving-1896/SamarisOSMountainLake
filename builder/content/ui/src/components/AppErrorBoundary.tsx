import React from "react";
import "./errorBoundary.css";

type State = { hasError: boolean; error: Error | null; dismissed: boolean };

export class AppErrorBoundary extends React.Component<{ children: React.ReactNode; name: string; onClose?: () => void }, State> {
  constructor(props: { children: React.ReactNode; name: string; onClose?: () => void }) {
    super(props);
    this.state = { hasError: false, error: null, dismissed: false };
  }

  static getDerivedStateFromError(error: Error) {
    return { hasError: true, error, dismissed: false };
  }

  componentDidCatch(error: Error) {
    console.error(`[AppErrorBoundary] ${this.props.name} crashed:`, error);
  }

  handleContinue = () => {
    this.setState({ dismissed: true });
  };

  render() {
    const { hasError, error, dismissed } = this.state;
    const { name, onClose } = this.props;

    return (
      <div className="error-boundary" role="presentation">
        {this.props.children}

        {hasError && !dismissed && (
          <div className="error-boundary__overlay" role="alertdialog" aria-label={`${name} crashed`}>
            <div className="error-boundary__card">
              <div className="error-boundary__glyph">
                <svg width="28" height="28" viewBox="0 0 24 24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round">
                  <circle cx="12" cy="12" r="10" />
                  <line x1="12" y1="8" x2="12" y2="12" />
                  <line x1="12" y1="16" x2="12.01" y2="16" />
                </svg>
              </div>
              <div className="error-boundary__title">{name} encountered an error</div>
              <div className="error-boundary__message">
                {error?.message || "An unexpected error occurred. The app may not work correctly."}
              </div>
              <div className="error-boundary__actions">
                <button type="button" className="error-boundary__btn error-boundary__btn--secondary" onClick={this.handleContinue}>
                  Continue
                </button>
                {onClose && (
                  <button type="button" className="error-boundary__btn" onClick={onClose}>
                    Close Window
                  </button>
                )}
              </div>
            </div>
          </div>
        )}
      </div>
    );
  }
}
