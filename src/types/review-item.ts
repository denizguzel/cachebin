import type { RiskLevel } from './risk-level';

export interface ReviewItem {
  id: string;
  name: string;
  path: string;
  sizeBytes: number;
  risk?: RiskLevel;
}
