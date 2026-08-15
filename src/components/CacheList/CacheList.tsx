import { CacheRow } from '@/components/CacheRow';
import { Checkbox } from '@/components/ui/checkbox';
import { Table, TableBody, TableHead, TableHeader, TableRow } from '@/components/ui/table';
import type { CacheEntry } from '@/types/cache-entry';

export interface CacheListProps {
  entries: CacheEntry[];
  totalCount: number;
  selectedIds: Set<string>;
  allSelected: boolean;
  onToggle: (id: string) => void;
  onToggleAll: () => void;
}

export function CacheList({ entries, totalCount, selectedIds, allSelected, onToggle, onToggleAll }: CacheListProps) {
  return (
    <div className="min-w-0">
      <p className="mb-1 mt-4 text-[11px] text-muted-tertiary">
        {entries.length} of {totalCount} location{totalCount === 1 ? '' : 's'}
      </p>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-11">
              <Checkbox
                checked={allSelected}
                onCheckedChange={onToggleAll}
                aria-label={allSelected ? 'Deselect all visible' : 'Select all visible'}
              />
            </TableHead>
            <TableHead className="text-[10px] uppercase tracking-[0.08em] text-muted-tertiary">Cache</TableHead>
            <TableHead className="whitespace-nowrap text-right font-mono text-[10px] text-muted-tertiary">
              Size
            </TableHead>
            <TableHead className="text-[10px] uppercase tracking-[0.08em] text-muted-tertiary">Risk</TableHead>
            <TableHead className="text-[10px] uppercase tracking-[0.08em] text-muted-tertiary max-[900px]:hidden">
              Modified
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {entries.map((entry) => (
            <CacheRow key={entry.id} entry={entry} selected={selectedIds.has(entry.id)} onToggle={onToggle} />
          ))}
        </TableBody>
      </Table>
    </div>
  );
}
