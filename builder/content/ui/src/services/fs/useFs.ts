import type { FsService } from "./types";
import { kernelFs } from "../kernel/fs";

export function useFs(): FsService {
  return kernelFs;
}
