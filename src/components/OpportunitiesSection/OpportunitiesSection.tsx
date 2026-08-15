import { ArrowUpRight, ChevronRight, Terminal } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { formatBytes } from '@/lib/format';
import type { Opportunity } from '@/types/opportunity';

export interface OpportunitiesSectionProps {
  opportunities: Opportunity[];
  onReview: () => void;
  onOpenCategory: (category: string) => void;
}

export function OpportunitiesSection({ opportunities, onReview, onOpenCategory }: OpportunitiesSectionProps) {
  return (
    <section className="min-w-0" aria-labelledby="opportunities-heading">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h2 id="opportunities-heading" className="m-0 text-[15px] font-semibold tracking-[-0.02em]">
            Largest opportunities
          </h2>
          <p className="mt-[5px] text-xs leading-5 text-muted-tertiary">Sorted by reclaimable space</p>
        </div>
        <Button variant="ghost" size="sm" onClick={onReview}>
          View all <ChevronRight size={14} />
        </Button>
      </div>
      <div className="mt-5 border-t border-border">
        {opportunities.map((item) => (
          <div
            className="flex items-center gap-3.5 border-b border-border-subtle py-3.5 max-[480px]:gap-2"
            key={item.name}
          >
            <div className="w-[205px] min-w-0 max-[720px]:w-auto max-[720px]:flex-1">
              <div className="flex min-w-0 items-center gap-2">
                <Terminal size={15} className="shrink-0 text-muted-tertiary" />
                <span className="truncate text-sm font-medium">{item.name}</span>
              </div>
              <span className="ml-[23px] mt-[5px] block text-[11px] text-muted-tertiary">{item.detail}</span>
            </div>
            <div
              className="h-[5px] flex-1 overflow-hidden rounded-full bg-surface-tertiary max-[720px]:hidden"
              aria-hidden="true"
            >
              <span className="block h-full rounded-full bg-foreground" style={{ width: `${item.percent}%` }} />
            </div>
            <span className="w-[58px] shrink-0 text-right font-mono text-sm max-[480px]:w-[55px]">
              {formatBytes(item.sizeBytes)}
            </span>
            <Button
              variant="ghost"
              size="icon"
              className="size-7 rounded-md"
              aria-label={`Open ${item.name} in Developer caches`}
              title={`Open ${item.name} in Developer caches`}
              onClick={() => onOpenCategory(item.name)}
            >
              <ArrowUpRight size={16} />
            </Button>
          </div>
        ))}
      </div>
    </section>
  );
}
