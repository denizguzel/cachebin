import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { useOnChange } from '@/hooks/useOnChange';
import { useTauriQuery } from '@/hooks/useTauriQuery';
import type { ActivityEvent } from '@/types/activity-event';

const MAX_EVENTS = 30;

export function useHistory() {
  const query = useTauriQuery<undefined, ActivityEvent[]>({ command: 'get_history' });
  const [events, setEvents] = useState<ActivityEvent[]>([]);
  const [initialized, setInitialized] = useState(false);

  useOnChange({
    value: query.data,
    onNext: (data) => {
      if (data) {
        setInitialized(true);
        setEvents(data);
      }
    },
  });

  useEffect(() => {
    if (initialized) {
      // Pending events only exist while their scan is in flight; persisting them would leave
      // a stale "Scanning…" row behind if the page reloads or the app closes mid-scan.
      const toSave = events.filter((event) => event.status !== 'pending');
      void invoke('save_history', { events: toSave });
    }
  }, [events, initialized]);

  const updateEvents = (updater: (prev: ActivityEvent[]) => ActivityEvent[]) => {
    setEvents((prev) => updater(prev).slice(-MAX_EVENTS));
  };

  const recordScanStart = (): string => {
    const id = `${Date.now()}-${Math.random().toString(36).slice(2)}`;
    updateEvents((prev) => [...prev, { id, kind: 'scan', status: 'pending', at: new Date().toISOString(), bytes: 0 }]);
    return id;
  };

  const completeScan = (id: string, status: 'success' | 'error', bytes: number) => {
    updateEvents((prev) => prev.map((event) => (event.id === id ? { ...event, status, bytes } : event)));
  };

  const recordCleanup = (bytes: number) => {
    updateEvents((prev) => [
      ...prev,
      {
        id: `${Date.now()}-${Math.random().toString(36).slice(2)}`,
        kind: 'cleanup',
        status: 'success',
        at: new Date().toISOString(),
        bytes,
      },
    ]);
  };

  const clearHistory = () => {
    setEvents([]);
  };

  return { history: events, recordScanStart, completeScan, recordCleanup, clearHistory };
}
