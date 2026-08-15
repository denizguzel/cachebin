import type { WslDistro } from './wsl-distro';

export interface PlatformInfo {
  osName: string;
  osVersion: string;
  hostname: string;
  totalBytes: number;
  freeBytes: number;
  usedBytes: number;
  wslDistros: WslDistro[];
}
