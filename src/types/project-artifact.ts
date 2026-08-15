import type { Environment } from './environment';
import type { RiskLevel } from './risk-level';

export interface ProjectArtifact {
  id: string;
  projectPath: string;
  name: string;
  path: string;
  environment: Environment;
  sizeBytes: number;
  risk: RiskLevel;
  description: string;
}
