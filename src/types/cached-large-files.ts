import type { LargeFile } from './large-file';

export interface CachedLargeFiles {
  status: 'scanning' | 'ready';
  scannedAt: string;
  files: LargeFile[];
}
