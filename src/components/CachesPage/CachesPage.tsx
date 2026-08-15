import { useState } from 'react';
import { FolderOpen } from 'lucide-react';
import { CacheList } from '@/components/CacheList';
import { CacheListSkeleton } from '@/components/CacheListSkeleton';
import { CacheReviewDialog } from '@/components/CacheReviewDialog';
import { CacheToolbar } from '@/components/CacheToolbar';
import { SelectionBar } from '@/components/SelectionBar';
import { useCleanupSelection } from '@/hooks/useCleanupSelection';
import { applyCacheFilters, categoriesFrom, type CacheFilters, type CacheSortKey } from '@/lib/cache-filters';
import { formatBytes } from '@/lib/format';
import type { CacheEntry } from '@/types/cache-entry';
import type { RiskLevel } from '@/types/risk-level';
import type { ScanState } from '@/types/scan-state';

export interface CachesPageProps {
  entries: CacheEntry[];
  scanState: ScanState;
  category: string;
  defaultRisk: RiskLevel | 'all';
  onCategoryChange: (category: string) => void;
  onCleanup: (bytes: number) => void;
}

export function CachesPage({
  entries,
  scanState,
  category,
  defaultRisk,
  onCategoryChange,
  onCleanup,
}: CachesPageProps) {
  const [risk, setRisk] = useState<RiskLevel | 'all'>(defaultRisk);
  const [sort, setSort] = useState<CacheSortKey>('size-desc');
  const [reviewOpen, setReviewOpen] = useState(false);
  const { selectedIds, selected, selectedBytes, progress, toggle, toggleAll, clear, moveSelectedToTrash } =
    useCleanupSelection({ items: entries, onCleanup });

  const filters: CacheFilters = { category, risk, sort };
  const categories = categoriesFrom(entries);
  const visible = applyCacheFilters(entries, filters);
  const totalBytes = entries.reduce((sum, entry) => sum + entry.sizeBytes, 0);
  const allVisibleSelected = visible.length > 0 && visible.every((entry) => selectedIds.has(entry.id));

  const updateFilters = (patch: Partial<CacheFilters>) => {
    if (patch.category !== undefined) onCategoryChange(patch.category);
    if (patch.risk !== undefined) setRisk(patch.risk);
    if (patch.sort !== undefined) setSort(patch.sort);
  };

  return (
    <div className="mx-auto w-full max-w-[1320px]">
      <div className="flex items-start justify-between gap-6 pb-6 max-[720px]:flex-col max-[720px]:gap-3">
        <div className="min-w-0">
          <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">
            Rebuildable toolchain data
          </p>
          <p className="mt-[5px] max-w-[560px] text-xs leading-5 text-muted-tertiary">
            Inspect cache locations and their risk before deciding what to move to Trash. Cleaning happens only after
            your confirmation.
          </p>
        </div>
        <p className="m-0 shrink-0 text-right font-mono text-xs text-muted-tertiary max-[720px]:text-left">
          {scanState === 'pending' && entries.length === 0 ? (
            'Scanning…'
          ) : (
            <>
              <strong className="font-medium text-foreground">{formatBytes(totalBytes)}</strong> cleanable ·{' '}
              {entries.length} location{entries.length === 1 ? '' : 's'}
            </>
          )}
        </p>
      </div>

      {entries.length > 0 && <CacheToolbar filters={filters} categories={categories} onChange={updateFilters} />}

      {entries.length === 0 ? (
        scanState === 'pending' ? (
          <CacheListSkeleton />
        ) : (
          <div className="flex min-h-[220px] items-center justify-center gap-2.5 text-[13px] text-muted-tertiary">
            <FolderOpen size={18} className="shrink-0" />
            <p className="m-0">
              {scanState === 'ready'
                ? 'No caches found in this workspace.'
                : 'No scan results yet. Run a scan to list rebuildable toolchain data.'}
            </p>
          </div>
        )
      ) : (
        <CacheList
          entries={visible}
          totalCount={entries.length}
          selectedIds={selectedIds}
          allSelected={allVisibleSelected}
          onToggle={toggle}
          onToggleAll={() => toggleAll(visible.map((entry) => entry.id))}
        />
      )}

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
