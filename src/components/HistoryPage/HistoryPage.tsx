import { ActivityRow } from '@/components/ActivityRow';
import type { ActivityEvent } from '@/types/activity-event';

export interface HistoryPageProps {
  history: ActivityEvent[];
}

export function HistoryPage({ history }: HistoryPageProps) {
  const entries = [...history].reverse();

  if (entries.length === 0) {
    return (
      <div className="mx-auto flex min-h-[220px] w-full max-w-[1320px] flex-col items-center justify-center gap-2.5 text-center">
        <p className="text-sm font-medium">No history yet</p>
        <p className="max-w-[420px] text-xs leading-5 text-muted-tertiary">
          Scans and cleanups you perform will appear here.
        </p>
      </div>
    );
  }

  return (
    <div className="mx-auto w-full max-w-[1320px]">
      <p className="pb-6 text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">Scans and cleanups</p>
      <div className="border-t border-border">
        {entries.map((item) => (
          <ActivityRow event={item} datePattern="MMM d, yyyy · HH:mm" key={item.id} />
        ))}
      </div>
    </div>
  );
}
