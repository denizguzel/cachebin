import type { CacheEntry } from './cache-entry';

export interface ScanResult {
  entries: CacheEntry[];
  totalBytes: number;
  locationCount: number;
}
