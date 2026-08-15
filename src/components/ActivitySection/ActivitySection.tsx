import { History } from 'lucide-react';
import { ActivityRow } from '@/components/ActivityRow';
import { Button } from '@/components/ui/button';
import type { ActivityEvent } from '@/types/activity-event';

const RECENT_LIMIT = 5;

export interface ActivitySectionProps {
  history: ActivityEvent[];
  onOpenHistory: () => void;
}

export function ActivitySection({ history, onOpenHistory }: ActivitySectionProps) {
  const entries = [...history].reverse().slice(0, RECENT_LIMIT);

  return (
    <section className="min-w-0" aria-labelledby="activity-heading">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h2 id="activity-heading" className="m-0 text-[15px] font-semibold tracking-[-0.02em]">
            Recent activity
          </h2>
          <p className="mt-[5px] text-xs leading-5 text-muted-tertiary">Scans and cleanups</p>
        </div>
        <Button variant="ghost" size="sm" onClick={onOpenHistory}>
          <History size={14} /> History
        </Button>
      </div>
      <div className="mt-5 border-t border-border">
        {entries.length === 0 ? (
          <div className="border-b border-border-subtle py-3.5 text-xs text-muted-tertiary">
            No activity yet. Run a scan to see it here.
          </div>
        ) : (
          entries.map((item) => <ActivityRow event={item} datePattern="MMM d, HH:mm" key={item.id} />)
        )}
      </div>
    </section>
  );
}
