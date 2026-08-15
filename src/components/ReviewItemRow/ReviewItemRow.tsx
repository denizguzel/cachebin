import { formatBytes } from '@/lib/format';
import type { ReviewItem } from '@/types/review-item';

export interface ReviewItemRowProps {
  item: ReviewItem;
}

export function ReviewItemRow({ item }: ReviewItemRowProps) {
  return (
    <div className="flex items-baseline gap-3 border-t border-border-subtle py-2">
      <span className="min-w-0 flex-1 truncate text-[13px]">{item.name}</span>
      <span className="shrink-0 font-mono text-xs text-muted-foreground">{formatBytes(item.sizeBytes)}</span>
    </div>
  );
}
