import React from "react";
import systemWallpaperUrl from "../../assets/wallpapers/system-wallpaper.png";
import { connectivityStore } from "../connectivity/connectivityStore";
import { batteryStore } from "../battery/batteryStore";
import { securityStore } from "../session/securityStore";
import { systemSounds } from "../sounds/systemSounds";
import { LoginClock } from "./LoginClock";
import { LoginAvatar } from "./LoginAvatar";
import { LoginActions } from "./LoginActions";
import { PowerControls } from "./PowerControls";
import { LOGIN_ANIMATION_MS, type LoginPhase, phaseClass } from "./login.animations";
import { LOGIN_CLASSES, LOGIN_COPY } from "./login.styles";
import { userKernel, type SamarisUser } from "../../services/kernel/user";

type ScreenVariant = "login" | "lock";

function loadPrefs() {
  try { return JSON.parse(localStorage.getItem("samaris-os/settings-prefs") || "{}"); } catch { return {}; }
}

export function LoginScreen() {
  const prefs = React.useMemo(() => loadPrefs(), []);
  const security = React.useSyncExternalStore(
    (listener) => securityStore.subscribe(listener),
    () => securityStore.getState()
  );
  const connectivity = React.useSyncExternalStore(
    (listener) => connectivityStore.subscribe(listener),
    () => connectivityStore.getState()
  );
  const battery = React.useSyncExternalStore(
    (listener) => batteryStore.subscribe(listener),
    () => batteryStore.getState()
  );

  const [phase, setPhase] = React.useState<LoginPhase>(security.locked ? "LOCKED" : "UNLOCKED");
  const [password, setPassword] = React.useState("");
  const [error, setError] = React.useState("");
  const [wallpaperLoaded, setWallpaperLoaded] = React.useState(false);
  const [users, setUsers] = React.useState<SamarisUser[]>([]);
  const [selectedUser, setSelectedUser] = React.useState<SamarisUser | null>(null);
  const hasAuthenticatedRef = React.useRef(false);

  // Load users on mount
  React.useEffect(() => {
    userKernel.list().then(setUsers).catch(() => {});
  }, []);

  // Auto-login: skip if single user without password (or autoLogin enabled)
  React.useEffect(() => {
    if (!security.locked && !security.hasPassword && (users.length === 1 || prefs.autoLogin)) {
      hasAuthenticatedRef.current = true;
      setPhase("UNLOCKED");
      return;
    }
    if (users.length === 1 && !selectedUser) {
      setSelectedUser(users[0]);
    }
  }, [security, users, selectedUser, prefs.autoLogin]);

  React.useEffect(() => {
    const image = new Image();
    image.src = systemWallpaperUrl;
    image.decode?.().catch(() => {});
    image.onload = () => setWallpaperLoaded(true);
    if (image.complete) setWallpaperLoaded(true);
  }, []);

  React.useEffect(() => {
    if (security.locked) {
      if (phase === "UNLOCKED") setPhase("LOCKED");
      return;
    }
    if (phase === "AUTHENTICATING") { setPhase("UNLOCKING"); return; }
    if (phase === "LOCKED" || phase === "ENTERING_PASSWORD") setPhase("UNLOCKED");
  }, [phase, security.locked]);

  React.useEffect(() => {
    if (phase !== "UNLOCKING") return;
    const id = window.setTimeout(() => {
      setPhase("UNLOCKED");
      setPassword("");
      setError("");
    }, LOGIN_ANIMATION_MS.unlock);
    return () => window.clearTimeout(id);
  }, [phase]);

  // If no users exist yet, skip login entirely → onboarding takes over
  if (users.length === 0 && !selectedUser) {
    return null;
  }

  const visible = (security.locked && prefs.lockScreen !== false) || phase === "AUTHENTICATING" || phase === "UNLOCKING";
  const variant: ScreenVariant = !hasAuthenticatedRef.current ? "login" : "lock";

  async function handleSubmit() {
    if (phase === "AUTHENTICATING" || phase === "UNLOCKING") return;
    setError("");
    setPhase("AUTHENTICATING");
    try {
      const result: { ok: boolean; message?: string } | null = await securityStore.unlock(password, selectedUser?.username) as any;
      if (result && result.ok === false) {
        setPhase(password ? "ENTERING_PASSWORD" : "LOCKED");
        setError(result.message || "Incorrect password");
        systemSounds.play("error");
        return;
      }
      hasAuthenticatedRef.current = true;
      systemSounds.play("login");
    } catch {
      setPhase(password ? "ENTERING_PASSWORD" : "LOCKED");
      setError("Unable to authenticate");
      systemSounds.play("error");
    }
  }

  if (!visible) return null;

  const currentNetwork =
    connectivity.networks.find((network) => network.connected)?.label ||
    connectivity.currentNetworkLabel || "Offline";
  const batteryLabel = `${Math.round(battery.percentage ?? 85)}%`;
  const helperText =
    phase === "AUTHENTICATING" ? "Authenticating…"
    : phase === "UNLOCKING" ? "Entering Samaris…"
    : variant === "login" ? LOGIN_COPY.loginTitle
    : LOGIN_COPY.lockTitle;

  // User selection screen
  if (variant === "login" && users.length > 1 && !selectedUser) {
    return (
      <div className={[LOGIN_CLASSES.root, `${LOGIN_CLASSES.root}--login`, phaseClass(phase), wallpaperLoaded ? "is-ready" : "is-loading"].join(" ")}
        aria-modal="true" role="dialog" aria-label="Login screen">
        <div className={`${LOGIN_CLASSES.wallpaper} ${wallpaperLoaded ? LOGIN_CLASSES.wallpaperLoaded : ""}`.trim()}
          style={{ backgroundImage: `url(${systemWallpaperUrl})` }} />
        <div className={LOGIN_CLASSES.diffusion} />
        <div className={LOGIN_CLASSES.glow} />
        <main className={LOGIN_CLASSES.shell}>
          <section className={LOGIN_CLASSES.core}>
            <LoginClock />
            <div className="samaris-login__identity">
              <div className="samaris-login__name">Choose an account</div>
              <div className="samaris-login__userList">
                {users.map((u) => (
                  <button key={u.username} type="button" className="samaris-login__userBtn" onClick={() => setSelectedUser(u)}>
                    <LoginAvatar displayName={u.displayName} variant="login" />
                    <div className="samaris-login__userName">{u.displayName}</div>
                    <div className="samaris-login__userSub">@{u.username}{u.guest ? " (Guest)" : ""}</div>
                  </button>
                ))}
              </div>
            </div>
          </section>
          <footer className={LOGIN_CLASSES.footer}>
            <div className="samaris-login__status">
              <span className="samaris-login__statusItem"><span className="samaris-login__statusDot" /><span>{currentNetwork}</span></span>
              <span className="samaris-login__statusDivider" />
              <span className="samaris-login__statusItem"><span>{batteryLabel}</span></span>
            </div>
          </footer>
          <PowerControls onFallbackLock={() => void securityStore.lock()} />
        </main>
      </div>
    );
  }

  return (
    <div
      className={[LOGIN_CLASSES.root, `${LOGIN_CLASSES.root}--${variant}`, phaseClass(phase), wallpaperLoaded ? "is-ready" : "is-loading"].join(" ")}
      aria-modal="true" role="dialog" aria-label={variant === "login" ? "Login screen" : "Lock screen"}
    >
      {variant === "login" || prefs.lockWallpaper !== false ? (
        <div className={`${LOGIN_CLASSES.wallpaper} ${wallpaperLoaded ? LOGIN_CLASSES.wallpaperLoaded : ""}`.trim()}
          style={{ backgroundImage: `url(${systemWallpaperUrl})` }} />
      ) : (
        <div className={`${LOGIN_CLASSES.wallpaper} samaris-login__wallpaper--veil`} />
      )}
      <div className={LOGIN_CLASSES.diffusion} />
      <div className={LOGIN_CLASSES.glow} />

      <main className={LOGIN_CLASSES.shell}>
        <section className={LOGIN_CLASSES.core}>
          <LoginClock />

          <div className="samaris-login__identity">
            <LoginAvatar displayName={selectedUser?.displayName || security.displayName || "Samaris User"} variant={variant} />
            <div className="samaris-login__name">{selectedUser?.displayName || security.displayName || "Samaris User"}</div>
            {selectedUser && <div className="samaris-login__subtitle">@{selectedUser.username}</div>}
            <div className="samaris-login__subtitle">
              {variant === "login" ? "Enter your password to continue." : "Your session is protected."}
            </div>
            {prefs.loginMessage ? <div className="samaris-login__message">{prefs.loginMessage}</div> : null}
            <LoginActions
              password={password}
              busy={phase === "AUTHENTICATING" || phase === "UNLOCKING"}
              error={error}
              helper={helperText}
              onChangePassword={(value) => {
                setPassword(value);
                setError("");
                setPhase(value ? "ENTERING_PASSWORD" : "LOCKED");
              }}
              onSubmit={() => void handleSubmit()}
            />
            {users.length > 1 && variant === "login" && (
              <button type="button" className="samaris-login__switchUser" onClick={() => setSelectedUser(null)}>
                Switch account
              </button>
            )}
          </div>
        </section>

        <footer className={LOGIN_CLASSES.footer}>
          <div className="samaris-login__status">
            <span className="samaris-login__statusItem"><span className="samaris-login__statusDot" /><span>{currentNetwork}</span></span>
            <span className="samaris-login__statusDivider" />
            <span className="samaris-login__statusItem"><span>{batteryLabel}</span></span>
            <span className="samaris-login__statusDivider" />
            <span className="samaris-login__statusItem"><span>{security.guestMode ? "Guest mode" : "Primary account"}</span></span>
          </div>
        </footer>

        <PowerControls onFallbackLock={() => void securityStore.lock()} />
      </main>
    </div>
  );
}
