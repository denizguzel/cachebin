import type { RiskLevel } from './risk-level';

export interface Settings {
  zoom: number;
  scanDirs: string[];
  disabledDistros: string[];
  defaultRiskFilter: RiskLevel | 'all';
  autoScanOnStartup: boolean;
}
