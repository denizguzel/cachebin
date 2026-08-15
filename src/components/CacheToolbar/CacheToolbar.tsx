import { Select, SelectContent, SelectGroup, SelectItem, SelectTrigger, SelectValue } from '@/components/ui/select';
import { ToggleGroup, ToggleGroupItem } from '@/components/ui/toggle-group';
import type { CacheFilters } from '@/lib/cache-filters';

const riskOptions: { value: CacheFilters['risk']; label: string }[] = [
  { value: 'all', label: 'All' },
  { value: 'safe', label: 'Safe' },
  { value: 'caution', label: 'Caution' },
  { value: 'risky', label: 'Risky' },
];

const sortOptions: { value: CacheFilters['sort']; label: string }[] = [
  { value: 'size-desc', label: 'Largest first' },
  { value: 'size-asc', label: 'Smallest first' },
  { value: 'name', label: 'Name A-Z' },
  { value: 'recent', label: 'Recently modified' },
];

export interface CacheToolbarProps {
  filters: CacheFilters;
  categories: string[];
  onChange: (patch: Partial<CacheFilters>) => void;
}

export function CacheToolbar({ filters, categories, onChange }: CacheToolbarProps) {
  return (
    <div className="flex flex-wrap items-center gap-3 border-y border-border py-3">
      <ToggleGroup
        type="single"
        variant="outline"
        size="sm"
        value={filters.risk}
        onValueChange={(value) => {
          if (value) onChange({ risk: value as CacheFilters['risk'] });
        }}
        aria-label="Risk filter"
      >
        {riskOptions.map((option) => (
          <ToggleGroupItem key={option.value} value={option.value}>
            {option.label}
          </ToggleGroupItem>
        ))}
      </ToggleGroup>

      <Select value={filters.category} onValueChange={(value) => onChange({ category: value })}>
        <SelectTrigger size="sm" className="w-[160px]" aria-label="Filter by category">
          <SelectValue placeholder="All categories" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            <SelectItem value="all">All categories</SelectItem>
            {categories.map((category) => (
              <SelectItem key={category} value={category}>
                {category}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>

      <Select value={filters.sort} onValueChange={(value) => onChange({ sort: value as CacheFilters['sort'] })}>
        <SelectTrigger size="sm" className="w-[160px]" aria-label="Sort caches">
          <SelectValue placeholder="Sort" />
        </SelectTrigger>
        <SelectContent>
          <SelectGroup>
            {sortOptions.map((option) => (
              <SelectItem key={option.value} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectGroup>
        </SelectContent>
      </Select>
    </div>
  );
}
