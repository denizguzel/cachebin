import { useEffect, useState } from 'react';
import { Check } from 'lucide-react';
import { toast } from 'sonner';
import { LocalDataSection } from '@/components/LocalDataSection';
import { PreferencesSection } from '@/components/PreferencesSection';
import { ScanLocationsSection } from '@/components/ScanLocationsSection';
import { WslDistrosSection } from '@/components/WslDistrosSection';
import { Button } from '@/components/ui/button';
import { useTauriQuery } from '@/hooks/useTauriQuery';
import type { PlatformInfo } from '@/types/platform-info';
import type { RiskLevel } from '@/types/risk-level';
import type { Settings } from '@/types/settings';

export interface SettingsPageProps {
  settings: Settings | null;
  scanDirOptions: string[];
  onUpdate: (next: Settings) => Promise<Settings>;
  onClearHistory: () => void;
}

export function SettingsPage({ settings, scanDirOptions, onUpdate, onClearHistory }: SettingsPageProps) {
  const platformQuery = useTauriQuery<undefined, PlatformInfo>({ command: 'get_platform_info' });
  const [draft, setDraft] = useState<Settings | null>(null);
  const [saving, setSaving] = useState(false);

  useEffect(() => {
    if (settings && draft === null) {
      setDraft(settings);
    }
  }, [settings, draft]);

  if (draft === null) {
    return (
      <div className="mx-auto flex min-h-[220px] w-full max-w-[1320px] items-center justify-center">
        <p className="text-sm text-muted-tertiary">Loading settings…</p>
      </div>
    );
  }

  const patch = (partial: Partial<Settings>) => {
    setDraft((prev) => prev && { ...prev, ...partial });
  };

  const toggleDir = (dir: string) => {
    setDraft((prev) => {
      if (!prev) return prev;
      const scanDirs = prev.scanDirs.includes(dir)
        ? prev.scanDirs.filter((value) => value !== dir)
        : [...prev.scanDirs, dir];
      return { ...prev, scanDirs };
    });
  };

  const toggleDistro = (name: string) => {
    setDraft((prev) => {
      if (!prev) return prev;
      const disabledDistros = prev.disabledDistros.includes(name)
        ? prev.disabledDistros.filter((value) => value !== name)
        : [...prev.disabledDistros, name];
      return { ...prev, disabledDistros };
    });
  };

  const handleSave = async () => {
    setSaving(true);
    try {
      await onUpdate(draft);
      toast.success('Settings saved.');
    } catch (err) {
      toast.error('Failed to save settings', { description: String(err) });
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="mx-auto w-full max-w-[900px]">
      <div className="pb-6">
        <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">Scan behavior</p>
        <p className="mt-[5px] max-w-[560px] text-xs leading-5 text-muted-tertiary">
          Choose which locations Cachebin scans for projects and large files, and how it behaves on startup.
        </p>
      </div>

      <ScanLocationsSection options={scanDirOptions} selected={draft.scanDirs} onToggle={toggleDir} />
      <div className="mt-6">
        <WslDistrosSection
          distros={platformQuery.data?.wslDistros ?? []}
          disabled={draft.disabledDistros}
          onToggle={toggleDistro}
        />
      </div>
      <div className="mt-6">
        <PreferencesSection
          autoScanOnStartup={draft.autoScanOnStartup}
          defaultRiskFilter={draft.defaultRiskFilter}
          onAutoScanChange={(checked) => patch({ autoScanOnStartup: checked })}
          onRiskChange={(value: RiskLevel | 'all') => patch({ defaultRiskFilter: value })}
        />
      </div>
      <div className="mt-6">
        <LocalDataSection onClearHistory={onClearHistory} />
      </div>

      <div className="mt-8 flex justify-end gap-3">
        <Button onClick={() => void handleSave()} disabled={saving}>
          {saving ? null : <Check size={15} />}
          {saving ? 'Saving…' : 'Save settings'}
        </Button>
      </div>
    </div>
  );
}
