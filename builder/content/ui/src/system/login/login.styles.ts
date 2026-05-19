import "./login.css";

export const LOGIN_CLASSES = {
  root: "samaris-login",
  shell: "samaris-login__shell",
  wallpaper: "samaris-login__wallpaper",
  wallpaperLoaded: "samaris-login__wallpaper--loaded",
  diffusion: "samaris-login__diffusion",
  glow: "samaris-login__glow",
  core: "samaris-login__core",
  footer: "samaris-login__footer",
  power: "samaris-login__power"
} as const;

export const LOGIN_COPY = {
  loginTitle: "Welcome to Samaris",
  lockTitle: "Session locked"
} as const;
