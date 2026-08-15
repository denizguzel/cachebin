import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { toast } from 'sonner';
import { useTauriQuery } from '@/hooks/useTauriQuery';
import type { CachedLargeFiles } from '@/types/cached-large-files';
import type { LargeFile } from '@/types/large-file';

const POLL_INTERVAL_MS = 1500;

export function useLargeFiles() {
  const cachedQuery = useTauriQuery<undefined, CachedLargeFiles | null>({ command: 'load_cached_large_files' });
  const [files, setFiles] = useState<LargeFile[]>([]);
  const [scannedAt, setScannedAt] = useState<Date | null>(null);
  const [isScanning, setIsScanning] = useState(false);

  useEffect(() => {
    const cached = cachedQuery.data;
    if (!cached) return;

    if (cached.status === 'scanning') {
      setIsScanning(true);
    } else {
      setIsScanning(false);
      setFiles(cached.files);
      setScannedAt(new Date(cached.scannedAt));
    }
  }, [cachedQuery.data]);

  // A scan started before a page refresh keeps running in the backend; poll the cache file
  // until it flips back to ready so the in-progress state survives reloads.
  const status = cachedQuery.data?.status;
  const refetch = cachedQuery.refetch;

  useEffect(() => {
    if (status !== 'scanning') return;
    const timer = setInterval(() => refetch(), POLL_INTERVAL_MS);
    return () => clearInterval(timer);
  }, [status, refetch]);

  const scan = async () => {
    if (isScanning) return;
    setIsScanning(true);
    try {
      const fresh = await invoke<LargeFile[]>('scan_large_files');
      setFiles(fresh);
      setScannedAt(new Date());
      toast.success(`Large file scan complete · ${fresh.length} file${fresh.length === 1 ? '' : 's'} found.`);
    } catch (err) {
      toast.error('Large file scan failed', { description: String(err) });
    } finally {
      setIsScanning(false);
    }
  };

  return { files, scannedAt, isScanning, scan };
}
