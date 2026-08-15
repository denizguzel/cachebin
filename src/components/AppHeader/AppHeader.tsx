import { CircleHelp, Menu, RefreshCw, ScanLine } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Tooltip, TooltipContent, TooltipTrigger } from '@/components/ui/tooltip';
import type { ScanProgress } from '@/types/scan-progress';
import type { ScanState } from '@/types/scan-state';

export interface AppHeaderProps {
  title: string;
  scanState: ScanState;
  scanProgress: ScanProgress | null;
  onScan: () => void;
  onOpenNavigation: () => void;
}

export function AppHeader({ title, scanState, scanProgress, onScan, onOpenNavigation }: AppHeaderProps) {
  const scanLabel =
    scanState === 'pending'
      ? scanProgress
        ? `Scanning ${scanProgress.current}/${scanProgress.total}…`
        : 'Scanning'
      : scanState === 'ready'
        ? 'Scan ready'
        : 'Scan now';

  return (
    <header className="flex min-h-[82px] items-center gap-4 border-b border-border-subtle px-[clamp(20px,4vw,48px)] py-[18px] max-[720px]:min-h-[72px] max-[720px]:px-5 max-[720px]:py-4">
      <button
        className="hidden size-[34px] shrink-0 place-items-center rounded-md border-0 bg-transparent text-muted-foreground transition-colors hover:bg-muted hover:text-foreground max-[900px]:order-first max-[900px]:ml-[-8px] max-[900px]:grid"
        type="button"
        onClick={onOpenNavigation}
        aria-label="Open navigation"
      >
        <Menu size={20} />
      </button>
      <div className="min-w-0">
        <p className="text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">
          Local storage overview
        </p>
        <h1 className="mt-0.5 text-[19px] font-semibold tracking-[-0.025em] max-[480px]:text-[17px]">{title}</h1>
      </div>
      <div className="ml-auto flex items-center gap-2">
        <Tooltip>
          <TooltipTrigger asChild>
            <Button variant="outline" size="icon" aria-label="Help" className="max-[480px]:hidden">
              <CircleHelp size={16} />
            </Button>
          </TooltipTrigger>
          <TooltipContent side="bottom" align="end">
            Scan finds cache folders in your workspace and shows how much space each one can free up.
          </TooltipContent>
        </Tooltip>
        <Button onClick={onScan} disabled={scanState === 'pending'}>
          {scanState === 'pending' ? <RefreshCw className="animate-spin" size={15} /> : <ScanLine size={15} />}
          {scanLabel}
        </Button>
      </div>
    </header>
  );
}
