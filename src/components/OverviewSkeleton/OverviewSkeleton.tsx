import { Skeleton } from '@/components/ui/skeleton';

export function OverviewSkeleton() {
  return (
    <div className="mx-auto w-full max-w-[1320px]" aria-hidden="true">
      <div className="flex min-h-[230px] flex-col items-start justify-between gap-6 border-b border-border pb-[38px] min-[721px]:flex-row min-[721px]:items-end">
        <div className="w-full max-w-[540px]">
          <Skeleton className="h-3 w-28" />
          <Skeleton className="mt-5 h-11 w-full max-w-[420px]" />
          <Skeleton className="mt-4 h-4 w-3/4" />
        </div>
        <Skeleton className="h-10 w-52" />
      </div>

      <div className="py-[31px] pb-7">
        <Skeleton className="h-4 w-40" />
        <Skeleton className="mt-5 h-2.5 w-full rounded-full" />
        <div className="mt-3 flex flex-wrap gap-x-[18px] gap-y-2">
          <Skeleton className="h-3 w-28" />
          <Skeleton className="h-3 w-28" />
          <Skeleton className="h-3 w-28" />
        </div>
      </div>

      <div className="grid grid-cols-2 divide-x divide-border border-y border-border min-[721px]:grid-cols-4">
        {Array.from({ length: 4 }).map((_, index) => (
          <div className="px-5 py-5" key={index}>
            <Skeleton className="h-3 w-20" />
            <Skeleton className="mt-3 h-6 w-24" />
            <Skeleton className="mt-2 h-3 w-28" />
          </div>
        ))}
      </div>

      <div className="mt-[38px] grid grid-cols-1 gap-12 min-[721px]:grid-cols-[minmax(0,1.3fr)_minmax(320px,0.7fr)]">
        <div>
          <Skeleton className="h-4 w-44" />
          {Array.from({ length: 4 }).map((_, index) => (
            <div className="mt-5 flex items-center gap-3.5 border-b border-border-subtle pb-3.5" key={index}>
              <Skeleton className="size-4" />
              <Skeleton className="h-4 w-40" />
              <Skeleton className="h-3 flex-1" />
              <Skeleton className="h-4 w-14" />
            </div>
          ))}
        </div>
        <div>
          <Skeleton className="h-4 w-36" />
          <Skeleton className="mt-5 h-4 w-full" />
          <Skeleton className="mt-3 h-4 w-3/4" />
          <Skeleton className="mt-3 h-4 w-2/3" />
        </div>
      </div>
    </div>
  );
}
