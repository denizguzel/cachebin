import { ReviewItemRow } from '@/components/ReviewItemRow';
import { formatBytes } from '@/lib/format';
import type { ReviewItem } from '@/types/review-item';
import type { RiskLevel } from '@/types/risk-level';

const riskOrder: RiskLevel[] = ['safe', 'caution', 'risky'];

const riskDot: Record<RiskLevel, string> = {
  safe: 'bg-safe',
  caution: 'bg-caution',
  risky: 'bg-danger',
};

export interface ReviewItemListProps {
  items: ReviewItem[];
}

export function ReviewItemList({ items }: ReviewItemListProps) {
  const riskGroups = riskOrder
    .map((risk) => ({ risk, group: items.filter((item) => item.risk === risk) }))
    .filter(({ group }) => group.length > 0);
  const unrisked = items.filter((item) => item.risk === undefined);

  return (
    <div className="max-h-[55vh] overflow-y-auto">
      {riskGroups.map(({ risk, group }) => {
        const groupTotal = group.reduce((sum, item) => sum + item.sizeBytes, 0);
        const label = risk.charAt(0).toUpperCase() + risk.slice(1);

        return (
          <section className="border-t border-border-subtle pt-4 first:border-0 first:pt-0" key={risk}>
            <h3 className="mb-0.5 flex items-center gap-2 text-[13px] font-semibold">
              <span className={`size-[7px] rounded-full ${riskDot[risk]}`} aria-hidden="true" />
              <span>{label}</span>
              <span className="ml-auto font-mono text-[11px] font-normal text-muted-tertiary">
                {group.length} item{group.length === 1 ? '' : 's'} · {formatBytes(groupTotal)}
              </span>
            </h3>
            {group.map((item) => (
              <ReviewItemRow item={item} key={item.id} />
            ))}
          </section>
        );
      })}

      {unrisked.length > 0 && (
        <section className="border-t border-border-subtle pt-4 first:border-0 first:pt-0">
          <h3 className="mb-0.5 flex items-center gap-2 text-[13px] font-semibold">
            <span>Selected items</span>
            <span className="ml-auto font-mono text-[11px] font-normal text-muted-tertiary">
              {unrisked.length} item{unrisked.length === 1 ? '' : 's'} ·{' '}
              {formatBytes(unrisked.reduce((sum, item) => sum + item.sizeBytes, 0))}
            </span>
          </h3>
          {unrisked.map((item) => (
            <ReviewItemRow item={item} key={item.id} />
          ))}
        </section>
      )}
    </div>
  );
}
