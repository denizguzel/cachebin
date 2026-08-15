import { formatBytes } from '@/lib/format';
import type { PlatformInfo } from '@/types/platform-info';

export interface StorageOverviewProps {
  platform: PlatformInfo;
  cleanableBytes: number;
}

export function StorageOverview({ platform, cleanableBytes }: StorageOverviewProps) {
  const total = platform.totalBytes;
  const used = platform.usedBytes;
  const free = platform.freeBytes;
  const percentUsed = total > 0 ? Math.round((used / total) * 100) : 0;
  const percentCleanable = total > 0 ? Math.round((cleanableBytes / total) * 100) : 0;

  return (
    <section className="py-[31px] pb-7" aria-labelledby="storage-heading">
      <div className="flex items-start justify-between gap-4">
        <div>
          <h2 id="storage-heading" className="m-0 text-[15px] font-semibold tracking-[-0.02em]">
            Storage at a glance
          </h2>
          <p className="mt-[5px] text-xs leading-5 text-muted-tertiary">
            {formatBytes(total)} drive · {formatBytes(used)} currently used
          </p>
        </div>
        <span className="font-mono text-xs text-muted-foreground">{percentUsed}% used</span>
      </div>
      <div className="mt-[22px] flex h-2.5 overflow-hidden rounded-full bg-surface-tertiary" aria-hidden="true">
        <span className="bg-foreground" style={{ width: `${percentUsed}%` }} />
        <span className="bg-muted-foreground" style={{ width: `${percentCleanable}%` }} />
      </div>
      <div className="mt-3 flex flex-wrap gap-x-[18px] gap-y-2 text-[11px] text-muted-foreground">
        <span className="inline-flex items-center gap-1.5">
          <i className="inline-block size-[7px] rounded-full bg-foreground" /> Used{' '}
          <strong className="font-mono text-[10px] font-normal text-foreground">{formatBytes(used)}</strong>
        </span>
        <span className="inline-flex items-center gap-1.5">
          <i className="inline-block size-[7px] rounded-full bg-muted-foreground" /> Cleanable{' '}
          <strong className="font-mono text-[10px] font-normal text-foreground">{formatBytes(cleanableBytes)}</strong>
        </span>
        <span className="inline-flex items-center gap-1.5">
          <i className="inline-block size-[7px] rounded-full border border-border-strong bg-surface-tertiary" /> Free{' '}
          <strong className="font-mono text-[10px] font-normal text-foreground">{formatBytes(free)}</strong>
        </span>
      </div>
    </section>
  );
}
