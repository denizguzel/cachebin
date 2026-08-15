import type { ProjectArtifact } from './project-artifact';

export interface ProjectGroup {
  projectPath: string;
  projectName: string;
  source: string;
  items: ProjectArtifact[];
  totalBytes: number;
}
