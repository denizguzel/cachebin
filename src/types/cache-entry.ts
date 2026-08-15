import type { Environment } from './environment';
import type { RiskLevel } from './risk-level';

export interface CacheEntry {
  id: string;
  category: string;
  name: string;
  path: string;
  environment: Environment;
  sizeBytes: number;
  risk: RiskLevel;
  description: string;
  lastModified: string | null;
  rebuildable: boolean;
}
