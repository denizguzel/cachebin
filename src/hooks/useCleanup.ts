import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import type { TrashReport } from '@/types/trash-report';

export interface CleanupProgress {
  current: number;
  total: number;
}

export function useCleanup() {
  const [isCleaning, setIsCleaning] = useState(false);
  const [progress, setProgress] = useState<CleanupProgress | null>(null);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    try {
      void listen<CleanupProgress>('cleanup://progress', (event) => {
        setProgress(event.payload);
      }).then((fn) => {
        unlisten = fn;
      });
    } catch {
      // Tauri event bridge is unavailable outside the desktop shell.
    }
    return () => {
      unlisten?.();
    };
  }, []);

  const moveToTrash = async (paths: string[]): Promise<TrashReport> => {
    setIsCleaning(true);
    setProgress({ current: 0, total: paths.length });
    try {
      return await invoke<TrashReport>('move_to_trash', { paths });
    } finally {
      setIsCleaning(false);
      setProgress(null);
    }
  };

  return { moveToTrash, isCleaning, progress };
}
