import type { ScanResult } from './scan-result';

export interface CachedScan {
  scannedAt: string;
  result: ScanResult;
}
