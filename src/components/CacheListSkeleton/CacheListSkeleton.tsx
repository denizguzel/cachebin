import { Skeleton } from '@/components/ui/skeleton';

export function CacheListSkeleton() {
  return (
    <div aria-hidden="true">
      <Skeleton className="mb-1 mt-4 h-3 w-32" />
      <div className="mt-2 border-t border-border">
        {Array.from({ length: 6 }).map((_, index) => (
          <div className="flex items-center gap-4 border-b border-border-subtle py-3.5" key={index}>
            <Skeleton className="size-4 shrink-0" />
            <div className="min-w-0 flex-1">
              <Skeleton className="h-4 w-40" />
              <Skeleton className="mt-2 h-3 w-2/3" />
            </div>
            <Skeleton className="h-4 w-14" />
            <Skeleton className="h-4 w-12" />
            <Skeleton className="h-4 w-20 max-[900px]:hidden" />
          </div>
        ))}
      </div>
    </div>
  );
}
