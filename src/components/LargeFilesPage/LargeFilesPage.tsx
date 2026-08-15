import { useState } from 'react';
import { FolderOpen, RefreshCw, ScanLine } from 'lucide-react';
import { CacheReviewDialog } from '@/components/CacheReviewDialog';
import { LargeFileRow } from '@/components/LargeFileRow';
import { SelectionBar } from '@/components/SelectionBar';
import { Button } from '@/components/ui/button';
import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { useCleanupSelection } from '@/hooks/useCleanupSelection';
import { useLargeFiles } from '@/hooks/useLargeFiles';
import { formatDate } from '@/lib/date';
import { sourceLabel } from '@/lib/environment';
import { formatBytes } from '@/lib/format';
import type { LargeFile } from '@/types/large-file';

export interface LargeFilesPageProps {
  onCleanup: (bytes: number) => void;
}

export function LargeFilesPage({ onCleanup }: LargeFilesPageProps) {
  const { files, scannedAt, isScanning, scan } = useLargeFiles();
  const [source, setSource] = useState('all');
  const [reviewOpen, setReviewOpen] = useState(false);

  const sources = [...new Set(files.map((file) => sourceLabel(file.environment)))].sort();
  const visible = source === 'all' ? files : files.filter((file) => sourceLabel(file.environment) === source);
  const totalBytes = files.reduce((sum, file) => sum + file.sizeBytes, 0);

  const { selectedIds, selected, selectedBytes, progress, toggle, clear, moveSelectedToTrash } = useCleanupSelection({
    items: files,
    onCleanup,
  });

  const lastScan = scannedAt ? `Last scan · ${formatDate(scannedAt, 'MMM d, HH:mm')}` : 'Not scanned yet';

  if (files.length === 0 && !isScanning) {
    return (
      <div className="mx-auto flex min-h-[220px] w-full max-w-[1320px] flex-col items-center justify-center gap-2.5 text-center">
        <FolderOpen size={18} className="shrink-0 text-muted-tertiary" />
        <p className="m-0 text-sm font-medium">No large files yet</p>
        <p className="max-w-[420px] text-xs leading-5 text-muted-tertiary">
          Run a scan to find files of at least 100 MB inside your workspace folders and WSL homes.
        </p>
        <Button size="sm" onClick={() => void scan()}>
          <ScanLine size={14} /> Scan large files
        </Button>
      </div>
    );
  }

  return (
    <div className="mx-auto w-full max-w-[1320px]">
      <div className="flex items-start justify-between gap-6 pb-6 max-[720px]:flex-col max-[720px]:gap-3">
        <div className="min-w-0">
          <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">
            Largest files in your workspace
          </p>
          <p className="mt-[5px] max-w-[560px] text-xs leading-5 text-muted-tertiary">
            Files of at least 100 MB across your project folders and WSL homes, sorted by size.
          </p>
        </div>
        <div className="flex shrink-0 flex-wrap items-center justify-end gap-3">
          <span className="whitespace-nowrap text-xs text-muted-tertiary">{lastScan}</span>
          <Button variant="outline" size="sm" onClick={() => void scan()} disabled={isScanning}>
            {isScanning ? <RefreshCw className="animate-spin" size={14} /> : <ScanLine size={14} />}
            {isScanning ? 'Scanning…' : 'Scan large files'}
          </Button>
        </div>
      </div>

      <div className="flex items-center gap-3 border-b border-border pb-4">
        <Select value={source} onValueChange={setSource}>
          <SelectTrigger size="sm" className="w-[170px]" aria-label="Filter by source">
            <SelectValue placeholder="All sources" />
          </SelectTrigger>
          <SelectContent>
            <SelectGroup>
              <SelectItem value="all">All sources</SelectItem>
              {sources.map((label) => (
                <SelectItem key={label} value={label}>
                  {label}
                </SelectItem>
              ))}
            </SelectGroup>
          </SelectContent>
        </Select>
        <span className="ml-auto whitespace-nowrap text-xs text-muted-tertiary">
          <strong className="font-medium text-foreground">{formatBytes(totalBytes)}</strong> in {files.length} file
          {files.length === 1 ? '' : 's'}
        </span>
      </div>

      <div className="border-b border-border">
        {visible.length === 0 ? (
          <p className="py-8 text-center text-xs text-muted-tertiary">No files match this source filter.</p>
        ) : (
          visible.map((file: LargeFile) => (
            <LargeFileRow file={file} key={file.id} selected={selectedIds.has(file.id)} onToggle={toggle} />
          ))
        )}
      </div>

      {selectedIds.size > 0 && (
        <SelectionBar
          count={selectedIds.size}
          bytes={selectedBytes}
          onClear={clear}
          onReview={() => setReviewOpen(true)}
        />
      )}

      <CacheReviewDialog
        open={reviewOpen}
        items={selected}
        progress={progress}
        onClose={() => setReviewOpen(false)}
        onConfirm={moveSelectedToTrash}
      />
    </div>
  );
}
