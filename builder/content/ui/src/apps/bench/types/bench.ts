export type BenchRun = {
  run_id: string;
  mode: "quick" | "full" | "stress" | "watch" | "ci";
  duration_seconds: number;
  iterations: number;
  warmup_iterations: number;
  cold_run: boolean;
  median_score: number;
  mean_score: number;
  min_score: number;
  max_score: number;
  stddev_score: number;
  confidence: "high" | "medium" | "low";
  reliability_flags: string[];
};

export type BenchHardware = {
  class: string;
  model: string;
  cpu: string;
  cpu_cores: number;
  ram_gb: number;
  gpu: string;
  storage_type: string;
  storage_model: string;
  arch: string;
  battery_or_ac: string;
  thermal_state: string;
};

export type BenchEnvironment = {
  display_resolution: string;
  display_scale: number;
  network_connected: boolean;
  running_from_usb: boolean;
  running_in_vm: boolean;
  power_mode: string;
  temperature_celsius?: number;
  kernel_version: string;
  system_uptime_seconds: number;
};

export type BenchCategoryScores = {
  system: number;
  ui: number;
  memory: number;
  kernel: number;
  graphics: number;
  ai: number;
  browser: number;
  filesystem: number;
  stability: number;
};

export type BenchResult = {
  version: string;
  timestamp: string;
  run: BenchRun;
  hardware: BenchHardware;
  environment: BenchEnvironment;
  os: { name: string; version: string; build: string; commit_hash: string | null; release_channel: string };
  overall: { score: number; score_out_of: number; normalized_score: number; badge: string; validity: string };
  category_scores: BenchCategoryScores;
  metrics: Record<string, any>;
  comparison: { baselines: any[]; comparison_validity: string };
  optimizer: { fitness_score: number; bottlenecks: string[]; recommendations: any[] };
};

export type BenchHistoryEntry = {
  timestamp: string;
  run_id: string;
  mode: string;
  score: number;
  badge: string;
  duration_seconds: number;
  hardware_class: string;
  category_scores: BenchCategoryScores;
  reliability_flags: string[];
};

export type BenchHistory = {
  entries: BenchHistoryEntry[];
  max_entries: number;
};

export type BenchBaseline = {
  source: "self_measured" | "imported";
  import_label?: string;
  imported_at: string;
  measurement_date?: string;
  hardware: Partial<BenchHardware>;
  os: { name: string; version: string };
  overall: { score: number; badge: string };
  category_scores: BenchCategoryScores;
};

export type BenchmarkMode = "quick" | "full" | "stress" | "ci";

export const BADGE_COLORS: Record<string, string> = {
  Legendary: "#f59e0b",
  Exceptional: "#10b981",
  Excellent: "#3b82f6",
  "Very Good": "#6366f1",
  Good: "#8b5cf6",
  "Needs Optimization": "#f97316",
  "Critical Optimization Needed": "#ef4444",
};

export const CATEGORY_LABELS: Record<string, string> = {
  system: "System",
  ui: "UI",
  memory: "Memory",
  kernel: "Kernel",
  graphics: "Graphics",
  ai: "AI",
  browser: "Browser",
  filesystem: "Filesystem",
  stability: "Stability",
};
