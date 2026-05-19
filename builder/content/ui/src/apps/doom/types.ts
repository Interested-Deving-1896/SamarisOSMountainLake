export type DosInstance = {
  stop?: () => Promise<void> | void;
};

export type DosFactory = (
  element: HTMLElement,
  options: Record<string, unknown>
) => DosInstance;
