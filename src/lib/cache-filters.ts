import type { CacheEntry } from '@/types/cache-entry';
import type { RiskLevel } from '@/types/risk-level';

export type CacheSortKey = 'size-desc' | 'size-asc' | 'name' | 'recent';

export interface CacheFilters {
  category: string;
  risk: RiskLevel | 'all';
  sort: CacheSortKey;
}

export function categoriesFrom(entries: CacheEntry[]): string[] {
  return [...new Set(entries.map((entry) => entry.category))].sort();
}

export function applyCacheFilters(entries: CacheEntry[], filters: CacheFilters): CacheEntry[] {
  const filtered = entries.filter((entry) => {
    if (filters.category !== 'all' && entry.category !== filters.category) return false;
    if (filters.risk !== 'all' && entry.risk !== filters.risk) return false;
    return true;
  });

  return [...filtered].sort((a, b) => {
    switch (filters.sort) {
      case 'size-desc':
        return b.sizeBytes - a.sizeBytes;
      case 'size-asc':
        return a.sizeBytes - b.sizeBytes;
      case 'name':
        return a.name.localeCompare(b.name);
      case 'recent':
        return (b.lastModified ?? '').localeCompare(a.lastModified ?? '');
    }
  });
}
