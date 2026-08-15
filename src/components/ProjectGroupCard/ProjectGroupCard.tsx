import { Checkbox } from '@/components/ui/checkbox';
import { Badge } from '@/components/ui/badge';
import { formatBytes } from '@/lib/format';
import type { ProjectGroup } from '@/types/project-group';

export interface ProjectGroupCardProps {
  group: ProjectGroup;
  selectedIds: Set<string>;
  onToggle: (id: string) => void;
  onToggleAll: (ids: string[]) => void;
}

export function ProjectGroupCard({ group, selectedIds, onToggle, onToggleAll }: ProjectGroupCardProps) {
  const allSelected = group.items.every((item) => selectedIds.has(item.id));

  return (
    <section className="border border-border rounded-[10px]" aria-label={`Project ${group.projectName}`}>
      <div className="flex items-center gap-3 border-b border-border px-4 py-3">
        <Checkbox
          checked={allSelected}
          onCheckedChange={() => onToggleAll(group.items.map((item) => item.id))}
          aria-label={`Select all in ${group.projectName}`}
        />
        <div className="min-w-0 flex-1">
          <p className="truncate text-sm font-semibold">{group.projectName}</p>
          <p className="mt-0.5 truncate font-mono text-[11px] text-muted-tertiary">{group.projectPath}</p>
        </div>
        <span className="whitespace-nowrap font-mono text-sm">{formatBytes(group.totalBytes)}</span>
        <span className="rounded-full bg-surface-secondary px-2 py-0.5 text-[11px] text-muted-foreground">
          {group.source}
        </span>
      </div>
      <div>
        {group.items.map((item) => (
          <div
            className="flex items-center gap-3 border-b border-border-subtle px-4 py-2.5 last:border-0"
            key={item.id}
          >
            <Checkbox
              checked={selectedIds.has(item.id)}
              onCheckedChange={() => onToggle(item.id)}
              aria-label={`Select ${item.name}`}
            />
            <div className="min-w-0 flex-1">
              <p className="truncate text-[13px]">{item.name}</p>
              <p className="mt-0.5 truncate text-[11px] text-muted-tertiary">{item.description}</p>
            </div>
            <Badge variant={item.risk}>{item.risk}</Badge>
            <span className="w-[70px] shrink-0 text-right font-mono text-xs">{formatBytes(item.sizeBytes)}</span>
          </div>
        ))}
      </div>
    </section>
  );
}
