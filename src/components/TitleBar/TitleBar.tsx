import { Copy, Minus, Square, X } from 'lucide-react';
import { useEffect, useRef, useState } from 'react';
import type { MouseEvent } from 'react';
import { TitleBarButton } from '@/components/TitleBarButton';
import { getCurrentWindow } from '@tauri-apps/api/window';

const isTauri = typeof window !== 'undefined' && '__TAURI_INTERNALS__' in window;

export function TitleBar() {
  const windowApi = isTauri ? getCurrentWindow() : null;
  const [maximized, setMaximized] = useState(false);
  const pendingDrag = useRef<{ x: number; y: number } | null>(null);

  const handleMouseDown = (event: MouseEvent<HTMLDivElement>) => {
    if (!windowApi || !maximized || event.button !== 0) return;
    if (event.target instanceof HTMLElement && event.target.closest('button')) return;

    pendingDrag.current = { x: event.clientX, y: event.clientY };
  };

  const handleMouseMove = async (event: MouseEvent<HTMLDivElement>) => {
    const start = pendingDrag.current;
    if (!windowApi || !maximized || !start) return;

    const moved = Math.abs(event.clientX - start.x) + Math.abs(event.clientY - start.y);
    if (moved < 6) return;

    pendingDrag.current = null;
    await windowApi.toggleMaximize();
    await windowApi.startDragging();
  };

  const handleMouseUp = () => {
    pendingDrag.current = null;
  };

  const handleDoubleClick = async (event: MouseEvent<HTMLDivElement>) => {
    if (!windowApi || (event.target instanceof HTMLElement && event.target.closest('button'))) return;

    pendingDrag.current = null;
    event.preventDefault();
    event.stopPropagation();
    await windowApi.toggleMaximize();
  };

  useEffect(() => {
    if (!windowApi) return;

    let unlisten: (() => void) | undefined;
    const syncMaximized = async () => {
      setMaximized(await windowApi.isMaximized());
    };

    void syncMaximized();
    void windowApi
      .onResized(() => {
        void syncMaximized();
      })
      .then((cleanup) => {
        unlisten = cleanup;
      });

    return () => {
      unlisten?.();
    };
  }, [windowApi]);

  return (
    <div
      className="flex h-10 shrink-0 items-center border-b border-border-subtle bg-background"
      data-tauri-drag-region={maximized ? 'false' : 'deep'}
      onMouseDown={handleMouseDown}
      onMouseMove={(event) => void handleMouseMove(event)}
      onMouseUp={handleMouseUp}
      onDoubleClick={(event) => void handleDoubleClick(event)}
      role="toolbar"
      aria-label="Window title bar"
      tabIndex={-1}
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
          <TitleBarButton
            label={maximized ? 'Restore window' : 'Maximize'}
            onClick={() => void windowApi?.toggleMaximize()}
          >
            {maximized ? <Copy size={13} /> : <Square size={12} />}
          </TitleBarButton>
          <TitleBarButton label="Close" danger onClick={() => void windowApi?.close()}>
            <X size={15} />
          </TitleBarButton>
        </div>
      )}
    </div>
  );
}
