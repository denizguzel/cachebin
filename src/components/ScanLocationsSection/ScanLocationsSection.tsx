import { Checkbox } from '@/components/ui/checkbox';
import { SettingsCard } from '@/components/SettingsCard';

export interface ScanLocationsSectionProps {
  options: string[];
  selected: string[];
  onToggle: (dir: string) => void;
}

export function ScanLocationsSection({ options, selected, onToggle }: ScanLocationsSectionProps) {
  return (
    <SettingsCard
      title="Windows scan locations"
      description="Top-level folders under your user profile scanned for project artifacts and large files."
    >
      <div className="grid grid-cols-1 gap-1 min-[600px]:grid-cols-2">
        {options.map((dir) => (
          <label className="flex items-center gap-2.5 rounded-md px-1 py-1.5 text-[13px]" key={dir}>
            <Checkbox
              checked={selected.includes(dir)}
              onCheckedChange={() => onToggle(dir)}
              aria-label={`Scan ${dir}`}
            />
            <span>{dir}</span>
          </label>
        ))}
      </div>
    </SettingsCard>
  );
}
