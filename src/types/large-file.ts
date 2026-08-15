import type { Environment } from './environment';

export interface LargeFile {
  id: string;
  name: string;
  path: string;
  environment: Environment;
  sizeBytes: number;
  fileType: string;
  lastModified: string | null;
}
