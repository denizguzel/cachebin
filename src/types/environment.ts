export interface Environment {
  kind: 'windows' | 'wsl';
  distro: string | null;
}
