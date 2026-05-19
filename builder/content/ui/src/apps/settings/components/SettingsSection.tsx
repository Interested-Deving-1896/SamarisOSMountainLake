import React from "react";

export const SettingsSection = React.forwardRef<HTMLElement, { title: string; description?: string; children: React.ReactNode }>(
  function SettingsSection(props, ref) {
    return (
      <section ref={ref} className="settings__section">
        <div className="settings__sectionHead">
          <div className="settings__sectionTitle">{props.title}</div>
          {props.description ? <div className="settings__sectionDescription">{props.description}</div> : null}
        </div>
        <div className="settings__sectionBody">{props.children}</div>
      </section>
    );
  }
);
