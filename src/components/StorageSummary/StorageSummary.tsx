import { Metric } from '@/components/Metric';
import { formatBytes } from '@/lib/format';
import { formatDate } from '@/lib/date';
import type { ActivityEvent } from '@/types/activity-event';
import type { ScanState } from '@/types/scan-state';

export interface StorageSummaryProps {
  cleanableBytes: number;
  largest: { name: string; sizeBytes: number } | null;
  locationCount: number;
  scanState: ScanState;
  lastCleanup: ActivityEvent | null;
}

export function StorageSummary({
  cleanableBytes,
  largest,
  locationCount,
  scanState,
  lastCleanup,
}: StorageSummaryProps) {
  return (
    <section
      className="grid grid-cols-2 divide-y divide-border border-y border-border min-[721px]:grid-cols-4 min-[721px]:divide-y-0 min-[721px]:divide-x"
      aria-label="Storage summary"
    >
      <Metric
        label="Cleanable space"
        value={formatBytes(cleanableBytes)}
        detail={`Across ${locationCount} locations`}
      />
      <Metric
        label="Largest category"
        value={largest?.name ?? '—'}
        detail={largest ? `${formatBytes(largest.sizeBytes)} available` : 'No data yet'}
      />
      <Metric
        label="Last cleanup"
        value={lastCleanup ? formatBytes(lastCleanup.bytes) : '—'}
        detail={
          lastCleanup
            ? `Recovered ${formatDate(new Date(lastCleanup.at), 'MMM d, HH:mm')}`
            : 'No cleanups yet this session'
        }
      />
      <Metric
        label="Scan coverage"
        value={`${locationCount} paths`}
        detail={
          scanState === 'pending' ? 'Scanning now' : scanState === 'ready' ? 'All checks completed' : 'Not scanned yet'
        }
      />
    </section>
  );
}
