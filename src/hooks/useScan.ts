import { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';
import { toast } from 'sonner';
import { useTauriQuery } from '@/hooks/useTauriQuery';
import { formatBytes } from '@/lib/format';
import type { CachedScan } from '@/types/cached-scan';
import type { PlatformInfo } from '@/types/platform-info';
import type { ScanProgress } from '@/types/scan-progress';
import type { ScanResult } from '@/types/scan-result';
import type { ScanState } from '@/types/scan-state';

export function useScan() {
  const platformQuery = useTauriQuery<undefined, PlatformInfo>({ command: 'get_platform_info' });
  const cachedQuery = useTauriQuery<undefined, CachedScan | null>({ command: 'load_cached_scan' });
  const [progress, setProgress] = useState<ScanProgress | null>(null);
  const [result, setResult] = useState<ScanResult | null>(null);
  const [lastScanAt, setLastScanAt] = useState<Date | null>(null);
  const [isScanning, setIsScanning] = useState(false);

  useEffect(() => {
    if (cachedQuery.data) {
      setResult(cachedQuery.data.result);
      setLastScanAt(new Date(cachedQuery.data.scannedAt));
    }
  }, [cachedQuery.data]);

  useEffect(() => {
    let unlisten: (() => void) | undefined;
    try {
      void listen<ScanProgress>('scan://progress', (event) => {
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

  const platform = platformQuery.data;
  const scanState: ScanState = isScanning ? 'pending' : result ? 'ready' : 'idle';

  const rescan = async (): Promise<number | null> => {
    if (isScanning) return null;
    setIsScanning(true);
    setProgress(null);
    try {
      const fresh = await invoke<ScanResult>('scan_storage');
      setResult(fresh);
      setLastScanAt(new Date());
      toast.success(`Scan complete · ${formatBytes(fresh.totalBytes)} found across your workspace.`);
      return fresh.totalBytes;
    } catch (err) {
      toast.error('Scan failed', { description: String(err) });
      return null;
    } finally {
      setIsScanning(false);
      setProgress(null);
    }
  };

  return {
    platform,
    result,
    scanState,
    progress,
    lastScanAt,
    rescan,
  };
}
