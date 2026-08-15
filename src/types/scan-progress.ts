export interface ScanProgress {
  phase: string;
  environment: string | null;
  current: number;
  total: number;
}
