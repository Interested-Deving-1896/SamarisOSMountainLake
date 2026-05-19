import type { FsService } from "../../services/fs/types";

export type TrashEntry = {
  id: string;
  name: string;
  originalPath: string;
  deletedAt: string;
  size: number;
  kind: "file" | "dir";
};

const INDEX_PATH = "/User/Trash/.trash_index.json";

function generateId(): string {
  return `${Date.now()}-${Math.random().toString(36).slice(2, 8)}`;
}

function parseIndex(raw: string): TrashEntry[] {
  try {
    const parsed = JSON.parse(raw);
    if (Array.isArray(parsed)) return parsed as TrashEntry[];
    return [];
  } catch {
    return [];
  }
}

export class TrashIndex {
  private fs: FsService;
  private entries: TrashEntry[] = [];
  private loaded = false;

  constructor(fs: FsService) {
    this.fs = fs;
  }

  async load(): Promise<TrashEntry[]> {
    if (this.loaded) return this.entries;
    try {
      const result = await this.fs.read(INDEX_PATH);
      this.entries = parseIndex(result.content);
    } catch {
      this.entries = [];
    }
    this.loaded = true;
    return this.entries;
  }

  private async save(): Promise<void> {
    try {
      await this.fs.write(INDEX_PATH, JSON.stringify(this.entries, null, 2));
    } catch {
      // silently fail
    }
  }

  async add(name: string, originalPath: string, size: number, kind: "file" | "dir"): Promise<TrashEntry> {
    await this.load();
    const entry: TrashEntry = {
      id: generateId(),
      name,
      originalPath,
      deletedAt: new Date().toISOString(),
      size,
      kind,
    };
    this.entries.push(entry);
    await this.save();
    return entry;
  }

  async remove(name: string): Promise<void> {
    await this.load();
    this.entries = this.entries.filter((e) => e.name !== name);
    await this.save();
  }

  async get(name: string): Promise<TrashEntry | undefined> {
    await this.load();
    return this.entries.find((e) => e.name === name);
  }

  async getAll(): Promise<TrashEntry[]> {
    await this.load();
    return [...this.entries];
  }

  async clear(): Promise<void> {
    this.entries = [];
    this.loaded = true;
    await this.save();
  }

  async search(query: string): Promise<TrashEntry[]> {
    await this.load();
    const q = query.toLowerCase();
    return this.entries.filter(
      (e) =>
        e.name.toLowerCase().includes(q) ||
        e.originalPath.toLowerCase().includes(q)
    );
  }

  async getTotalSize(): Promise<number> {
    await this.load();
    return this.entries.reduce((sum, e) => sum + e.size, 0);
  }

  async getCount(): Promise<number> {
    await this.load();
    return this.entries.length;
  }
}
