import { CheckCircle2, CircleX, Loader2, Trash2 } from 'lucide-react';
import { formatDate } from '@/lib/date';
import { formatBytes } from '@/lib/format';
import type { ActivityEvent } from '@/types/activity-event';

export interface ActivityRowProps {
  event: ActivityEvent;
  datePattern: string;
}

export function ActivityRow({ event, datePattern }: ActivityRowProps) {
  const isCleanup = event.kind === 'cleanup';

  if (isCleanup) {
    return (
      <div className="flex items-center gap-[11px] border-b border-border-subtle py-3.5">
        <Trash2 className="shrink-0 text-muted-foreground" size={16} />
        <div className="min-w-0 flex-1">
          <p className="text-sm font-medium">Moved to Trash</p>
          <p className="mt-1 text-xs text-muted-tertiary">{formatDate(new Date(event.at), datePattern)}</p>
        </div>
        <span className="whitespace-nowrap font-mono text-xs text-muted-foreground max-[480px]:hidden">
          {formatBytes(event.bytes)} recovered
        </span>
      </div>
    );
  }

  if (event.status === 'pending') {
    return (
      <div className="flex items-center gap-[11px] border-b border-border-subtle py-3.5">
        <Loader2 className="shrink-0 animate-spin text-caution" size={16} />
        <div className="min-w-0 flex-1">
          <p className="text-sm font-medium">Scanning…</p>
          <p className="mt-1 text-xs text-muted-tertiary">{formatDate(new Date(event.at), datePattern)}</p>
        </div>
      </div>
    );
  }

  if (event.status === 'error') {
    return (
      <div className="flex items-center gap-[11px] border-b border-border-subtle py-3.5">
        <CircleX className="shrink-0 text-danger" size={16} />
        <div className="min-w-0 flex-1">
          <p className="text-sm font-medium">Scan failed</p>
          <p className="mt-1 text-xs text-muted-tertiary">{formatDate(new Date(event.at), datePattern)}</p>
        </div>
      </div>
    );
  }

  return (
    <div className="flex items-center gap-[11px] border-b border-border-subtle py-3.5">
      <CheckCircle2 className="shrink-0 text-safe" size={16} />
      <div className="min-w-0 flex-1">
        <p className="text-sm font-medium">Scan completed</p>
        <p className="mt-1 text-xs text-muted-tertiary">{formatDate(new Date(event.at), datePattern)}</p>
      </div>
      <span className="whitespace-nowrap font-mono text-xs text-muted-foreground max-[480px]:hidden">
        {formatBytes(event.bytes)} found
      </span>
    </div>
  );
}
