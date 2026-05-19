import React from "react";
import { ArrowRight } from "lucide-react";

function LegalNotice() {
  return (
    <div className="samaris-onboarding__licenseScroll">
      <h3>Samaris OS 1.0 Mountain Lake — Public Alpha Legal Notice</h3>

      <p><b>1. ALPHA STATUS.</b> Samaris OS Mountain Lake is an experimental pre-release software prototype. It is not a finished consumer operating system. Features may be incomplete, data loss is possible, and hardware compatibility is not guaranteed. This software is provided for VM testing, UI/UX feedback, architecture review, and early public visibility. Do not use on production systems, critical infrastructure, or devices containing sensitive data.</p>

      <p><b>2. ACCEPTANCE.</b> By downloading, installing, booting, copying, evaluating, or using Samaris OS in any form, you acknowledge that you have read, understood, and agree to be bound by all terms of this Legal Notice. If you do not agree, do not download, boot, or use Samaris OS.</p>

      <p><b>3. LICENSE GRANT.</b> Samaris OS is provided for personal, educational, research, review, and non-commercial testing purposes. Commercial use, including but not limited to corporate evaluation, institutional deployment, resale, embedding in commercial products, or use in revenue-generating environments, requires prior written authorization from the licensor.</p>

      <p><b>4. RESTRICTIONS.</b> You may not: (a) redistribute or publish official Samaris OS ISOs or builds without written permission; (b) fork, rebrand, clone, or present the Samaris proprietary layer — including the desktop shell, UI assets, visual identity, brand marks, logos, icons, wallpapers, sounds, and official documentation — as another product; (c) remove or alter any proprietary notices or markings; (d) use the "Samaris" name or logo without explicit written permission; (e) decompile, reverse-engineer, or extract the Samaris proprietary components beyond what applicable law permits.</p>

      <p><b>5. OPEN SOURCE COMPONENTS.</b> Samaris OS incorporates open-source components including but not limited to the Linux kernel, GRUB, systemd, Chromium, Node.js, React, Vite, Tailwind CSS, and various other third-party libraries. Each open-source component remains governed by its own applicable license (GPL, LGPL, MIT, Apache 2.0, BSD, etc.). Nothing in this Legal Notice limits, supersedes, or restricts any rights granted by those upstream licenses. The Samaris-specific product layer — the desktop interface, visual identity, brand assets, proprietary configuration, packaging, and documentation — is protected separately.</p>

      <p><b>6. PRIVACY POLICY.</b> Samaris OS is designed and built on a local-first privacy principle:</p>
      <p style={{ marginLeft: 16 }}><b>No Telemetry:</b> Samaris OS does NOT collect telemetry, behavioral analytics, usage tracking, or crash reports by default. No data is sent to any server without explicit user action.<br />
      <b>No Mandatory Accounts:</b> No online account, registration, or cloud service is required to use the desktop. The system boots directly into a guest-accessible environment.<br />
      <b>Local-First Data:</b> Your files, documents, settings, passwords, encryption keys, and session data remain on your local device by default. Cloud features are opt-in only.<br />
      <b>No Data Selling:</b> Samaris OS does not sell personal data to advertisers, third parties, or data brokers.<br />
      <b>Optional Online Services:</b> If optional online features are introduced (update checks, license verification, app stores), they will process only the data minimally required and will be clearly documented.</p>

      <p><b>7. THIRD-PARTY NETWORK BEHAVIOR.</b> Chromium and other embedded components may attempt network connections for their own purposes (update checks, certificate validation, safe browsing, etc.). Samaris OS configures these components to minimize unwanted connections. Users should review the privacy settings of included third-party components for complete transparency.</p>

      <p><b>8. NO WARRANTY.</b> Samaris OS is provided "AS IS" and "AS AVAILABLE", without any warranty of any kind, express or implied, including but not limited to the warranties of merchantability, fitness for a particular purpose, title, non-infringement, compatibility, security, accuracy, or uninterrupted operation. The entire risk as to the quality, performance, and safety of the software rests with you.</p>

      <p><b>9. LIMITATION OF LIABILITY.</b> To the maximum extent permitted by applicable law, the licensor shall not be liable for any direct, indirect, incidental, special, consequential, exemplary, or punitive damages — including but not limited to data loss, hardware damage, system failure, lost profits, loss of privacy, or business interruption — arising from the use or inability to use Samaris OS, even if advised of the possibility of such damages.</p>

      <p><b>10. CONTACT.</b> For licensing inquiries, security reports, partnership proposals, or any questions regarding this Legal Notice: <b>contact.samaris.os@gmail.com</b>. Security vulnerabilities should be reported privately to this address and not disclosed publicly before responsible disclosure.</p>

      <p style={{ opacity: 0.5, fontSize: "11px", marginTop: 16 }}>Full document: SamarisOS-main/LEGAL.md · Samaris OS Mountain Lake · 2026</p>
    </div>
  );
}

export function LicenseScreen(props: {
  accepted: boolean;
  onToggleAccepted: (next: boolean) => void;
  onNext: () => void;
}) {
  return (
    <>
      <h1 className="samaris-onboarding__brandTitle">Samaris OS</h1>
      <div className="samaris-onboarding__progressLine" />
      <h1 className="samaris-onboarding__title is-tight">License Agreement</h1>
      <p className="samaris-onboarding__lead is-small">
        Please review and accept the Samaris OS terms before continuing.
      </p>
      <div className="samaris-onboarding__licenseBox">
        <LegalNotice />
        <label className="samaris-onboarding__checkbox">
          <input type="checkbox" checked={props.accepted} onChange={(event) => props.onToggleAccepted(event.target.checked)} />
          <span>I have read and accept the Samaris OS License Agreement.</span>
        </label>
        <div className="samaris-onboarding__navRow">
          <button
            type="button"
            className="samaris-onboarding__arrow samaris-onboarding__arrow--form"
            title="Accept and continue"
            aria-label="Accept and continue"
            onClick={props.onNext}
            disabled={!props.accepted}
          >
            <ArrowRight size={26} strokeWidth={2.1} />
          </button>
        </div>
      </div>
    </>
  );
}
