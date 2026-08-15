import { Checkbox } from '@/components/ui/checkbox';
import { SettingsCard } from '@/components/SettingsCard';
import type { WslDistro } from '@/types/wsl-distro';

export interface WslDistrosSectionProps {
  distros: WslDistro[];
  disabled: string[];
  onToggle: (name: string) => void;
}

export function WslDistrosSection({ distros, disabled, onToggle }: WslDistrosSectionProps) {
  return (
    <SettingsCard
      title="WSL distributions"
      description="Disabled distributions are skipped by scans. Stopped distributions are never scanned."
    >
      {distros.length === 0 ? (
        <p className="text-xs text-muted-tertiary">No WSL distributions detected.</p>
      ) : (
        distros.map((distro) => {
          const enabled = !disabled.includes(distro.name);
          return (
            <label className="flex items-center gap-2.5 rounded-md px-1 py-1.5 text-[13px]" key={distro.name}>
              <Checkbox
                checked={enabled}
                onCheckedChange={() => onToggle(distro.name)}
                aria-label={`Scan ${distro.name}`}
              />
              <span className="font-medium">{distro.name}</span>
              <span className="text-xs text-muted-tertiary">{distro.state}</span>
            </label>
          );
        })
      )}
    </SettingsCard>
  );
}
