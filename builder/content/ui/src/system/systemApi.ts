import { securityStore } from "./session/securityStore";
import { systemSounds } from "./sounds/systemSounds";

declare global {
  interface Window {
    samaris?: {
      system?: {
        sleep?: () => void | Promise<void>;
        restart?: () => void | Promise<void>;
        shutdown?: () => void | Promise<void>;
        lock?: () => void | Promise<void>;
      };
    };
  }
}

export function installSamarisSystemApi() {
  const current = window.samaris ?? {};
  const system = current.system ?? {};

  window.samaris = {
    ...current,
    system: {
      ...system,
      sleep: async () => {
        systemSounds.play("logout");
        await securityStore.lock();
      },
      restart: async () => {
        systemSounds.play("logout");
        window.setTimeout(() => window.location.reload(), 110);
      },
      shutdown: async () => {
        systemSounds.play("shutdown");
        await securityStore.lock();
      },
      lock: async () => {
        systemSounds.play("logout");
        await securityStore.lock();
      }
    }
  };
}
