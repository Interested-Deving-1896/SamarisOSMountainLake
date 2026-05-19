declare global {
  interface Window {
    __VOLT_READY__?: boolean;
  }
}

class BootSync {
  markReady() {
    window.__VOLT_READY__ = true;
    window.dispatchEvent(new Event("volt:boot-ready"));
  }

  isReady() {
    return Boolean(window.__VOLT_READY__);
  }
}

export const bootSync = new BootSync();
