import { useState } from 'react';
import { ArrowUpRight, Database, RefreshCw, ScanLine, X } from 'lucide-react';
import { getCurrentWindow, Window } from '@tauri-apps/api/window';
import { Button } from '@/components/ui/button';
import { useHistory } from '@/hooks/useHistory';
import { useScan } from '@/hooks/useScan';
import { formatBytes } from '@/lib/format';

export interface CompactPopupProps {
  onOpenMain?: () => void;
}

export function CompactPopup({ onOpenMain }: CompactPopupProps) {
  const { result, scanState, progress, lastScanAt, rescan } = useScan();
  const { recordScanStart, completeScan } = useHistory();
  const [opening, setOpening] = useState(false);
  const currentWindow = getCurrentWindow();
  const cleanableBytes = result?.totalBytes ?? 0;
  const locationCount = result?.locationCount ?? 0;

  const handleScan = async () => {
    const id = recordScanStart();
    const bytes = await rescan();
    completeScan(id, bytes === null ? 'error' : 'success', bytes ?? 0);
  };

  const handleOpenMain = async () => {
    setOpening(true);
    try {
      const main = await Window.getByLabel('main');
      await main?.show();
      await main?.setFocus();
      await currentWindow.hide();
      onOpenMain?.();
    } finally {
      setOpening(false);
    }
  };

  return (
    <div className="flex h-screen flex-col overflow-hidden rounded-xl border border-border bg-background text-foreground shadow-2xl">
      <header
        className="flex h-10 shrink-0 items-center border-b border-border-subtle px-3"
        data-tauri-drag-region="deep"
      >
        <div className="flex items-center gap-2">
          <span
            className="grid size-[18px] place-items-center rounded-[5px] bg-foreground text-[10px] font-semibold text-background"
            aria-hidden="true"
          >
            C
          </span>
          <span className="text-[13px] font-semibold">Cachebin</span>
        </div>
        <button
          className="ml-auto grid size-7 place-items-center rounded-md border-0 bg-transparent text-muted-foreground hover:bg-muted hover:text-foreground"
          type="button"
          aria-label="Close popup"
          onClick={() => void currentWindow.hide()}
        >
          <X size={14} />
        </button>
      </header>

      <main className="flex min-h-0 flex-1 flex-col gap-4 overflow-y-auto p-4">
        <section className="rounded-lg border border-border bg-surface-secondary p-4">
          <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">Cleanable space</p>
          <p className="mt-2 text-3xl font-medium tracking-[-0.05em]">{formatBytes(cleanableBytes)}</p>
          <p className="mt-1 text-xs text-muted-tertiary">
            {locationCount} location{locationCount === 1 ? '' : 's'} found
          </p>
        </section>

        <div className="flex items-center gap-2 text-xs text-muted-tertiary">
          <Database size={14} />
          <span>{lastScanAt ? 'Last scan available' : 'No scan yet'}</span>
          {scanState === 'pending' && (
            <span className="ml-auto text-caution">
              {progress ? `Scanning ${progress.current}/${progress.total}…` : 'Scanning…'}
            </span>
          )}
        </div>

        <div className="mt-auto grid gap-2">
          <Button onClick={() => void handleScan()} disabled={scanState === 'pending'}>
            {scanState === 'pending' ? <RefreshCw className="animate-spin" size={14} /> : <ScanLine size={14} />}
            {scanState === 'pending' ? 'Scanning…' : 'Scan now'}
          </Button>
          <Button variant="outline" onClick={() => void handleOpenMain()} disabled={opening}>
            <ArrowUpRight size={14} /> Open Cachebin
          </Button>
        </div>
      </main>
    </div>
  );
}
