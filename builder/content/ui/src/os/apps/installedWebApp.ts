import { appLoader } from "./appLoader";
import { appStoreKernel } from "../../services/kernel/appStore";
import type { InstalledApp } from "../../services/kernel/appStore";

export type InstalledWebAppWindowParams = {
  launchUrl: string;
  title: string;
  subtitle: string;
  source: "App Store";
  windowPreferenceKey: string;
  preferredWidth: number;
  preferredHeight: number;
};

function validateLaunchUrl(launchUrl: string) {
  let parsed: URL;
  try {
    parsed = new URL(launchUrl);
  } catch {
    throw new Error("Invalid launch URL for this installed app.");
  }

  if (parsed.protocol !== "http:" && parsed.protocol !== "https:") {
    throw new Error("This installed app uses an unsupported protocol.");
  }
  if (parsed.hostname !== "127.0.0.1" && parsed.hostname !== "localhost") {
    throw new Error("This installed app points to an external target.");
  }
}

function toPreferredDimension(value: unknown, fallback: number) {
  return typeof value === "number" && Number.isFinite(value) && value >= 480 ? value : fallback;
}

export function getInstalledWebAppWindowParams(app: InstalledApp): InstalledWebAppWindowParams {
  const launchUrl = String(app.launchUrl || "");
  if (!launchUrl) {
    throw new Error(app.launchError || "This installed app is not launchable yet.");
  }
  validateLaunchUrl(launchUrl);

  const title = app.manifest?.displayName || app.repoName || app.appId;
  const preferredWidth = toPreferredDimension(app.manifest?.samaris?.preferredWidth, 1180);
  const preferredHeight = toPreferredDimension(app.manifest?.samaris?.preferredHeight, 780);

  return {
    launchUrl,
    title,
    subtitle: "App Store",
    source: "App Store",
    windowPreferenceKey: `installed-web-app:${app.appId}`,
    preferredWidth,
    preferredHeight
  };
}

export async function openInstalledWebApp(app: InstalledApp) {
  const result = await appStoreKernel.startApp(app.appId);
  if (!result.ok) {
    console.error("Failed to start app server", result.error);
    return;
  }

  const launchUrl = result.launchUrl || app.launchUrl;
  const updatedApp = { ...app, launchUrl };

  const params = getInstalledWebAppWindowParams(updatedApp);
  return await appLoader.openApp("installed-web-app", {
    windowParams: params,
    forceNewWindow: true
  });
}
