import { useState } from 'react';
import { toast } from 'sonner';
import { useCleanup } from '@/hooks/useCleanup';
import type { ReviewItem } from '@/types/review-item';

export interface useCleanupSelectionProps {
  items: ReviewItem[];
  onCleanup: (bytes: number) => void;
}

export function useCleanupSelection({ items, onCleanup }: useCleanupSelectionProps) {
  const { moveToTrash, progress } = useCleanup();
  const [selectedIds, setSelectedIds] = useState<Set<string>>(() => new Set());

  const selected = items.filter((item) => selectedIds.has(item.id));
  const selectedBytes = selected.reduce((sum, item) => sum + item.sizeBytes, 0);

  const toggle = (id: string) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      if (next.has(id)) next.delete(id);
      else next.add(id);
      return next;
    });
  };

  const toggleAll = (ids: string[]) => {
    setSelectedIds((prev) => {
      const next = new Set(prev);
      const allIncluded = ids.length > 0 && ids.every((id) => next.has(id));
      for (const id of ids) {
        if (allIncluded) next.delete(id);
        else next.add(id);
      }
      return next;
    });
  };

  const clear = () => setSelectedIds(new Set());

  const moveSelectedToTrash = async (): Promise<boolean> => {
    try {
      const report = await moveToTrash(selected.map((item) => item.path));

      if (report.moved.length > 0) {
        const movedIds = new Set(selected.filter((item) => report.moved.includes(item.path)).map((item) => item.id));
        const movedBytes = selected
          .filter((item) => movedIds.has(item.id))
          .reduce((sum, item) => sum + item.sizeBytes, 0);
        onCleanup(movedBytes);
        setSelectedIds((prev) => {
          const next = new Set(prev);
          for (const id of movedIds) next.delete(id);
          return next;
        });
      }

      if (report.failed.length > 0) {
        toast.error(`Cleanup failed for ${report.failed.length} item${report.failed.length === 1 ? '' : 's'}`, {
          description: report.failed
            .slice(0, 3)
            .map((failure) => `${failure.path}: ${failure.error}`)
            .join('\n'),
        });
        return false;
      }

      toast.success(`Moved ${report.moved.length} item${report.moved.length === 1 ? '' : 's'} to Trash.`);
      return true;
    } catch (err) {
      toast.error('Cleanup failed', { description: String(err) });
      return false;
    }
  };

  return {
    selectedIds,
    selected,
    selectedBytes,
    progress,
    toggle,
    toggleAll,
    clear,
    moveSelectedToTrash,
  };
}
