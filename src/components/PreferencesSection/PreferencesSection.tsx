import { Checkbox } from '@/components/ui/checkbox';
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { SettingsCard } from '@/components/SettingsCard';
import type { RiskLevel } from '@/types/risk-level';

const riskOptions: { value: RiskLevel | 'all'; label: string }[] = [
  { value: 'all', label: 'All risk levels' },
  { value: 'safe', label: 'Safe only' },
  { value: 'caution', label: 'Safe + Caution' },
  { value: 'risky', label: 'Safe + Caution + Risky' },
];

export interface PreferencesSectionProps {
  autoScanOnStartup: boolean;
  defaultRiskFilter: RiskLevel | 'all';
  onAutoScanChange: (checked: boolean) => void;
  onRiskChange: (value: RiskLevel | 'all') => void;
}

export function PreferencesSection({
  autoScanOnStartup,
  defaultRiskFilter,
  onAutoScanChange,
  onRiskChange,
}: PreferencesSectionProps) {
  return (
    <SettingsCard title="Developer caches" description="How the developer caches screen behaves by default.">
      <div className="flex flex-wrap items-center gap-8">
        <label className="flex items-center gap-2.5 text-[13px]" htmlFor="auto-scan">
          <Checkbox
            id="auto-scan"
            checked={autoScanOnStartup}
            onCheckedChange={(checked) => onAutoScanChange(checked === true)}
          />
          <span>
            Scan automatically on startup
            <span className="ml-1 text-xs text-muted-tertiary">(otherwise the last scan is loaded from cache)</span>
          </span>
        </label>
        <div className="flex items-center gap-2.5 text-[13px]">
          <span>Default risk filter</span>
          <Select value={defaultRiskFilter} onValueChange={(value) => onRiskChange(value as RiskLevel | 'all')}>
            <SelectTrigger size="sm" className="w-[190px]" aria-label="Default risk filter">
              <SelectValue />
            </SelectTrigger>
            <SelectContent>
              <SelectGroup>
                {riskOptions.map((option) => (
                  <SelectItem key={option.value} value={option.value}>
                    {option.label}
                  </SelectItem>
                ))}
              </SelectGroup>
            </SelectContent>
          </Select>
        </div>
      </div>
    </SettingsCard>
  );
}
