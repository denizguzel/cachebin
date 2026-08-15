import type { Environment } from '@/types/environment';

export function sourceLabel(environment: Environment): string {
  return environment.kind === 'wsl' ? (environment.distro ?? 'WSL') : 'Windows';
}
