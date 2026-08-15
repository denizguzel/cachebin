import { ActivitySection } from '@/components/ActivitySection';
import { CleanupSafetyNote } from '@/components/CleanupSafetyNote';
import { OpportunitiesSection } from '@/components/OpportunitiesSection';
import { OverviewHero } from '@/components/OverviewHero';
import { OverviewSkeleton } from '@/components/OverviewSkeleton';
import { StorageOverview } from '@/components/StorageOverview';
import { StorageSummary } from '@/components/StorageSummary';
import { largestCategory, opportunitiesFrom } from '@/lib/selectors';
import type { ActivityEvent } from '@/types/activity-event';
import type { PlatformInfo } from '@/types/platform-info';
import type { ScanResult } from '@/types/scan-result';
import type { ScanState } from '@/types/scan-state';

export interface OverviewPageProps {
  platform: PlatformInfo | null;
  result: ScanResult | null;
  scanState: ScanState;
  lastScanAt: Date | null;
  history: ActivityEvent[];
  onReview: () => void;
  onOpenHistory: () => void;
  onOpenCategory: (category: string) => void;
}

export function OverviewPage({
  platform,
  result,
  scanState,
  lastScanAt,
  history,
  onReview,
  onOpenHistory,
  onOpenCategory,
}: OverviewPageProps) {
  if (result === null || platform === null) {
    if (scanState === 'pending') {
      return <OverviewSkeleton />;
    }
    return (
      <div className="mx-auto flex w-full max-w-[1320px] min-h-[60vh] flex-col items-center justify-center gap-3 text-center">
        <p className="text-sm font-medium">No scan results yet</p>
        <p className="max-w-[420px] text-xs leading-5 text-muted-tertiary">
          Run a scan from the top bar to see your storage overview and cleanable opportunities.
        </p>
      </div>
    );
  }

  const opportunities = opportunitiesFrom(result.entries);
  const largest = largestCategory(result.entries);
  const lastCleanup = [...history].reverse().find((item) => item.kind === 'cleanup') ?? null;

  return (
    <div className="mx-auto w-full max-w-[1320px]">
      <OverviewHero cleanableBytes={result.totalBytes} lastScanAt={lastScanAt} onReview={onReview} />
      <StorageOverview platform={platform} cleanableBytes={result.totalBytes} />
      <StorageSummary
        cleanableBytes={result.totalBytes}
        largest={largest}
        locationCount={result.locationCount}
        scanState={scanState}
        lastCleanup={lastCleanup}
      />
      <div className="mt-[38px] grid grid-cols-1 gap-[42px] min-[721px]:grid-cols-[minmax(0,1.3fr)_minmax(320px,0.7fr)] min-[721px]:gap-12">
        <OpportunitiesSection opportunities={opportunities} onReview={onReview} onOpenCategory={onOpenCategory} />
        <ActivitySection history={history} onOpenHistory={onOpenHistory} />
      </div>
      <CleanupSafetyNote />
    </div>
  );
}
