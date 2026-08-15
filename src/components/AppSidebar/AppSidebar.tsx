import { Settings2, ShieldCheck, X } from 'lucide-react';
import { navItems } from '@/data/dashboard';
import type { View } from '@/types/view';

export interface AppSidebarProps {
  activeView: View;
  mobileNavOpen: boolean;
  onSelectView: (view: View) => void;
  onClose: () => void;
}

export function AppSidebar({ activeView, mobileNavOpen, onSelectView, onClose }: AppSidebarProps) {
  return (
    <aside
      className={`flex w-[238px] shrink-0 flex-col border-r border-border-subtle bg-background px-4 pb-[18px] pt-[22px] max-[900px]:fixed max-[900px]:inset-y-0 max-[900px]:left-0 max-[900px]:z-20 max-[900px]:-translate-x-full max-[900px]:shadow-[12px_0_30px_rgb(0_0_0/0.12)] max-[900px]:transition-transform max-[900px]:duration-[180ms] ${
        mobileNavOpen ? 'max-[900px]:translate-x-0' : ''
      }`}
    >
      <div className="flex items-center justify-between">
        <button
          className="grid size-7 place-items-center rounded-md border-0 bg-foreground text-[13px] font-semibold text-background transition-opacity hover:opacity-80"
          type="button"
          onClick={() => onSelectView('overview')}
          aria-label="Go to overview"
        >
          C
        </button>
        <button
          className="hidden size-9 place-items-center border-0 bg-transparent text-muted-foreground max-[900px]:grid"
          type="button"
          onClick={onClose}
          aria-label="Close navigation"
        >
          <X size={18} />
        </button>
      </div>

      <div className="mt-8 flex items-center gap-[7px] text-[11px] text-muted-foreground">
        <span className="size-[6px] rounded-full bg-safe" aria-hidden="true" />
        <span>Local workspace</span>
        <span className="font-mono text-[10px] text-muted-tertiary">WSL2</span>
      </div>

      <nav className="mt-8 flex flex-1 flex-col gap-1" aria-label="Primary navigation">
        <p className="mb-2 ml-2.5 text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">Workspace</p>
        {navItems.map((item) => {
          const Icon = item.icon;
          const selected = activeView === item.id;

          return (
            <button
              className={`flex w-full items-center gap-2.5 rounded-[7px] border-0 px-2.5 py-[9px] text-left text-[13px] transition-colors hover:bg-muted hover:text-foreground ${
                selected ? 'bg-muted text-foreground' : 'bg-transparent text-muted-foreground'
              }`}
              key={item.id}
              type="button"
              aria-current={selected ? 'page' : undefined}
              onClick={() => onSelectView(item.id)}
            >
              <Icon size={16} strokeWidth={selected ? 2 : 1.7} />
              <span>{item.label}</span>
            </button>
          );
        })}

        <p className="mb-2 ml-2.5 mt-8 text-[10px] font-medium uppercase tracking-[0.08em] text-muted-tertiary">
          System
        </p>
        <button
          className={`flex w-full items-center gap-2.5 rounded-[7px] border-0 px-2.5 py-[9px] text-left text-[13px] transition-colors hover:bg-muted hover:text-foreground ${
            activeView === 'settings' ? 'bg-muted text-foreground' : 'bg-transparent text-muted-foreground'
          }`}
          type="button"
          aria-current={activeView === 'settings' ? 'page' : undefined}
          onClick={() => onSelectView('settings')}
        >
          <Settings2 size={16} strokeWidth={1.7} />
          <span>Settings</span>
        </button>
      </nav>

      <div className="border-t border-border-subtle px-2 pt-[18px]">
        <div className="flex items-start gap-2.5">
          <ShieldCheck className="mt-0.5 text-safe" size={16} />
          <div>
            <p className="text-xs font-medium">Files stay local</p>
            <p className="mt-1 text-[11px] leading-4 text-muted-tertiary">No analytics. Cleanup goes to Trash.</p>
          </div>
        </div>
        <p className="mt-5 font-mono text-[10px] text-muted-tertiary">v0.1.0 · Tauri 2</p>
      </div>
    </aside>
  );
}
