import { Minus, Square, X } from 'lucide-react';
import { TitleBarButton } from '@/components/TitleBarButton';
import { getCurrentWindow } from '@tauri-apps/api/window';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export function TitleBar() {
  const windowApi = isTauri ? getCurrentWindow() : null;

  return (
    <div
      className="flex h-10 shrink-0 items-center border-b border-border-subtle bg-background"
      data-tauri-drag-region="deep"
    >
      <div className="flex items-center gap-2 pl-3">
        <span
          className="grid size-[18px] place-items-center rounded-[5px] bg-foreground text-[10px] font-semibold text-background"
          aria-hidden="true"
        >
          C
        </span>
        <span className="text-[13px] font-semibold tracking-[-0.01em]">Cachebin</span>
      </div>
      {isTauri && (
        <div className="ml-auto flex h-full">
          <TitleBarButton label="Minimize" onClick={() => void windowApi?.minimize()}>
            <Minus size={15} />
          </TitleBarButton>
          <TitleBarButton label="Maximize" onClick={() => void windowApi?.toggleMaximize()}>
            <Square size={12} />
          </TitleBarButton>
          <TitleBarButton label="Close" danger onClick={() => void windowApi?.close()}>
            <X size={15} />
          </TitleBarButton>
        </div>
      )}
    </div>
  );
}
