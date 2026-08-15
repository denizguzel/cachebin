import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useTauriQuery } from '@/hooks/useTauriQuery';
import type { Settings } from '@/types/settings';

export function useSettings() {
  const settingsQuery = useTauriQuery<undefined, Settings>({ command: 'get_settings' });
  const optionsQuery = useTauriQuery<undefined, string[]>({ command: 'get_scan_dir_options' });
  const [settings, setSettings] = useState<Settings | null>(null);

  useEffect(() => {
    setSettings((prev) => prev ?? settingsQuery.data);
  }, [settingsQuery.data]);

  const update = async (next: Settings) => {
    const saved = await invoke<Settings>('update_settings', { settings: next });
    setSettings(saved);
    return saved;
  };

  return {
    settings,
    scanDirOptions: optionsQuery.data ?? [],
    update,
  };
}
