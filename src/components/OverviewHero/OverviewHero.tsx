import { ArrowUpRight } from 'lucide-react';
import { Badge } from '@/components/ui/badge';
import { Button } from '@/components/ui/button';
import { formatDate } from '@/lib/date';
import { formatBytes } from '@/lib/format';

export interface OverviewHeroProps {
  cleanableBytes: number;
  lastScanAt: Date | null;
  onReview: () => void;
}

export function OverviewHero({ cleanableBytes, lastScanAt, onReview }: OverviewHeroProps) {
  const scanTime = lastScanAt ? `Last scan · ${formatDate(lastScanAt, 'MMM d, HH:mm')}` : 'Not scanned yet';

  return (
    <section className="flex min-h-0 flex-col items-start justify-between gap-6 border-b border-border pb-[30px] max-[720px]:gap-6 min-[721px]:min-h-[230px] min-[721px]:flex-row min-[721px]:items-end min-[721px]:gap-8 min-[721px]:pb-[38px]">
      <div>
        <div className="flex items-center gap-2">
          <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">{scanTime}</p>
          <Badge variant="safe">Local only</Badge>
        </div>
        <h2 className="mt-[11px] max-w-[680px] text-[clamp(30px,4vw,46px)] font-medium leading-[1.05] tracking-[-0.055em]">
          {formatBytes(cleanableBytes)} is ready for review.
        </h2>
        <p className="mt-4 max-w-[540px] text-[15px] leading-relaxed text-muted-foreground">
          Rebuildable developer data is using space on your local workspace. Review the largest opportunities before
          cleanup.
        </p>
      </div>
      <div className="w-full pb-0.5 min-[721px]:w-auto">
        <Button size="lg" className="w-full min-[721px]:w-auto" onClick={onReview}>
          Review cleanable space
          <ArrowUpRight size={16} />
        </Button>
        <p className="mt-2 text-right text-xs text-muted-tertiary max-[720px]:text-left">
          Nothing is moved automatically.
        </p>
      </div>
    </section>
  );
}
