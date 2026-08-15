import { Badge } from '@/components/ui/badge';
import { Checkbox } from '@/components/ui/checkbox';
import { TableCell, TableRow } from '@/components/ui/table';
import { formatBytes } from '@/lib/format';
import type { CacheEntry } from '@/types/cache-entry';

export interface CacheRowProps {
  entry: CacheEntry;
  selected: boolean;
  onToggle: (id: string) => void;
}

export function CacheRow({ entry, selected, onToggle }: CacheRowProps) {
  const riskLabel = entry.risk.charAt(0).toUpperCase() + entry.risk.slice(1);
  const sourceLabel = entry.environment.kind === 'wsl' ? (entry.environment.distro ?? 'WSL') : 'Windows';

  return (
    <TableRow className={selected ? 'bg-muted' : undefined}>
      <TableCell className="w-11">
        <Checkbox checked={selected} onCheckedChange={() => onToggle(entry.id)} aria-label={`Select ${entry.name}`} />
      </TableCell>
      <TableCell className="min-w-0 max-w-[420px] pr-6 align-top">
        <div className="flex min-w-0 items-center gap-2">
          <span className="min-w-0 truncate text-sm font-medium">{entry.name}</span>
          <span className="shrink-0 rounded-full border border-border px-[7px] py-px font-mono text-[10px] leading-[1.5] text-muted-tertiary">
            {sourceLabel}
          </span>
        </div>
        <p className="mt-[3px] text-xs text-muted-foreground">{entry.description}</p>
        <p className="mt-1 truncate font-mono text-[11px] text-muted-tertiary">{entry.path}</p>
      </TableCell>
      <TableCell className="whitespace-nowrap pr-6 text-right font-mono text-[13px]">
        {formatBytes(entry.sizeBytes)}
      </TableCell>
      <TableCell className="pr-6">
        <Badge variant={entry.risk}>{riskLabel}</Badge>
      </TableCell>
      <TableCell className="whitespace-nowrap text-[11px] text-muted-tertiary max-[900px]:hidden">
        {entry.lastModified ?? '—'}
      </TableCell>
    </TableRow>
  );
}
