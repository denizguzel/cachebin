import type { Opportunity } from '@/types/opportunity';
import type { CacheEntry } from '@/types/cache-entry';

export function opportunitiesFrom(entries: CacheEntry[]): Opportunity[] {
  const byCategory = new Map<string, { sizeBytes: number; count: number }>();

  for (const entry of entries) {
    const current = byCategory.get(entry.category) ?? { sizeBytes: 0, count: 0 };
    byCategory.set(entry.category, {
      sizeBytes: current.sizeBytes + entry.sizeBytes,
      count: current.count + 1,
    });
  }

  const sorted = [...byCategory.entries()]
    .map(([name, { sizeBytes, count }]) => ({ name, sizeBytes, count }))
    .sort((a, b) => b.sizeBytes - a.sizeBytes);

  const max = sorted[0]?.sizeBytes ?? 1;

  return sorted.map(({ name, sizeBytes, count }) => ({
    name,
    sizeBytes,
    count,
    detail: `${count} location${count === 1 ? '' : 's'}`,
    percent: Math.round((sizeBytes / max) * 100),
  }));
}

export function largestCategory(entries: CacheEntry[]): { name: string; sizeBytes: number } | null {
  const opportunities = opportunitiesFrom(entries);
  return opportunities[0] ? { name: opportunities[0].name, sizeBytes: opportunities[0].sizeBytes } : null;
}
