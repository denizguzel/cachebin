import { useState } from 'react';
import { Loader2 } from 'lucide-react';
import { RiskyWarning } from '@/components/RiskyWarning';
import { ReviewItemList } from '@/components/ReviewItemList';
import { Button } from '@/components/ui/button';
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
} from '@/components/ui/dialog';
import { formatBytes } from '@/lib/format';
import { useOnChange } from '@/hooks/useOnChange';
import type { CleanupProgress } from '@/hooks/useCleanup';
import type { ReviewItem } from '@/types/review-item';

export interface CacheReviewDialogProps {
  open: boolean;
  items: ReviewItem[];
  progress: CleanupProgress | null;
  onClose: () => void;
  onConfirm: (paths: string[]) => Promise<boolean>;
}

export function CacheReviewDialog({ open, items, progress, onClose, onConfirm }: CacheReviewDialogProps) {
  const [confirming, setConfirming] = useState(false);
  const [riskAcknowledged, setRiskAcknowledged] = useState(false);
  const totalBytes = items.reduce((sum, item) => sum + item.sizeBytes, 0);
  const hasRisky = items.some((item) => item.risk === 'risky');

  useOnChange({
    value: open,
    onNext: (next) => {
      if (next) {
        setRiskAcknowledged(false);
      }
    },
  });

  const handleConfirm = async () => {
    setConfirming(true);
    try {
      const moved = await onConfirm(items.map((item) => item.path));
      if (moved) onClose();
    } finally {
      setConfirming(false);
    }
  };

  const cleaning = confirming && progress !== null;
  const moveLabel = cleaning
    ? `Moving ${progress.current} of ${progress.total}…`
    : confirming
      ? 'Moving…'
      : 'Move to Trash';

  return (
    <Dialog
      open={open}
      onOpenChange={(next) => {
        if (!next) onClose();
      }}
    >
      <DialogContent className="max-w-[600px]">
        <DialogHeader>
          <DialogTitle>Review selected items</DialogTitle>
          <DialogDescription>Choose what moves to Trash. Recovery stays available in History.</DialogDescription>
        </DialogHeader>

        <ReviewItemList items={items} />

        {hasRisky && <RiskyWarning acknowledged={riskAcknowledged} onChange={setRiskAcknowledged} />}

        <DialogFooter>
          <p className="mr-auto text-xs text-muted-foreground">
            {items.length} item{items.length === 1 ? '' : 's'} · {formatBytes(totalBytes)} reclaimable
          </p>
          <Button variant="outline" size="sm" onClick={onClose} disabled={confirming}>
            Cancel
          </Button>
          <Button
            variant="destructive"
            size="sm"
            disabled={confirming || (hasRisky && !riskAcknowledged)}
            onClick={handleConfirm}
          >
            {confirming ? <Loader2 className="animate-spin" size={14} /> : null}
            {moveLabel}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
