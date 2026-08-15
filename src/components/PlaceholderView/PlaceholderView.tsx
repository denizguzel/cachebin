import { Database } from 'lucide-react';
import { viewCopy } from '@/data/dashboard';
import type { View } from '@/types/view';

export interface PlaceholderViewProps {
  view: Exclude<View, 'overview'>;
}

export function PlaceholderView({ view }: PlaceholderViewProps) {
  const copy = viewCopy[view];

  return (
    <div className="max-w-[560px] pt-[58px]">
      <div className="grid size-10 place-items-center rounded-[10px] border border-border text-muted-foreground">
        <Database size={20} />
      </div>
      <p className="mt-7 text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">
        Next workspace surface
      </p>
      <h2 className="mt-[11px] max-w-[680px] text-[clamp(30px,4vw,46px)] font-medium leading-[1.05] tracking-[-0.055em]">
        {copy.title}
      </h2>
      <p className="mt-4 max-w-[540px] text-[15px] leading-relaxed text-muted-foreground">{copy.description}</p>
      <div className="mb-5 mt-8 border-t border-border" />
      <p className="text-sm text-muted-foreground">
        The shell is ready. The Rust scanner contract will populate this view next.
      </p>
    </div>
  );
}
