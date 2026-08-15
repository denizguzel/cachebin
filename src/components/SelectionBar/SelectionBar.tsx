import { Button } from '@/components/ui/button';
import { formatBytes } from '@/lib/format';

export interface SelectionBarProps {
  count: number;
  bytes: number;
  onClear: () => void;
  onReview: () => void;
}

export function SelectionBar({ count, bytes, onClear, onReview }: SelectionBarProps) {
  return (
    <section
      className="fixed bottom-6 left-1/2 z-30 flex -translate-x-1/2 items-center gap-6 rounded-[10px] border border-border-strong bg-background py-2.5 pl-[18px] pr-3 shadow-[0_12px_30px_rgb(0_0_0/0.12)] max-[720px]:right-5 max-[720px]:left-5 max-[720px]:translate-x-0"
      aria-label="Selection summary"
    >
      <div>
        <p className="text-sm font-medium">
          {count} item{count === 1 ? '' : 's'} selected
        </p>
        <p className="mt-0.5 text-xs text-muted-tertiary">{formatBytes(bytes)} reclaimable</p>
      </div>
      <div className="flex items-center gap-2">
        <Button variant="ghost" size="sm" onClick={onClear}>
          Clear
        </Button>
        <Button size="sm" onClick={onReview}>
          Review
        </Button>
      </div>
    </section>
  );
}
