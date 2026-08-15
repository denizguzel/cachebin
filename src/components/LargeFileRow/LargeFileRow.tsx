import { Checkbox } from '@/components/ui/checkbox';
import { sourceLabel } from '@/lib/environment';
import { formatBytes } from '@/lib/format';
import type { LargeFile } from '@/types/large-file';

export interface LargeFileRowProps {
  file: LargeFile;
  selected: boolean;
  onToggle: (id: string) => void;
}

export function LargeFileRow({ file, selected, onToggle }: LargeFileRowProps) {
  return (
    <div className="flex items-center gap-3 border-b border-border-subtle py-3 last:border-0">
      <Checkbox checked={selected} onCheckedChange={() => onToggle(file.id)} aria-label={`Select ${file.name}`} />
      <div className="min-w-0 flex-1">
        <p className="truncate text-[13px]">{file.name}</p>
        <p className="mt-0.5 truncate font-mono text-[11px] text-muted-tertiary">{file.path}</p>
      </div>
      <span className="hidden shrink-0 rounded-full bg-surface-secondary px-2 py-0.5 text-[11px] text-muted-foreground min-[480px]:inline">
        {file.fileType}
      </span>
      <span className="shrink-0 rounded-full bg-surface-secondary px-2 py-0.5 text-[11px] text-muted-foreground">
        {sourceLabel(file.environment)}
      </span>
      <span className="w-[76px] shrink-0 text-right font-mono text-xs">{formatBytes(file.sizeBytes)}</span>
    </div>
  );
}
