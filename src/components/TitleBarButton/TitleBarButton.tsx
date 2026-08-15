import type * as React from 'react';

export interface TitleBarButtonProps {
  label: string;
  onClick: () => void;
  danger?: boolean;
}

export function TitleBarButton({
  label,
  onClick,
  danger = false,
  children,
}: React.PropsWithChildren<TitleBarButtonProps>) {
  return (
    <button
      type="button"
      aria-label={label}
      onClick={onClick}
      className={`grid h-full w-11 place-items-center border-0 bg-transparent text-muted-foreground transition-colors hover:text-foreground ${
        danger ? 'hover:bg-danger hover:text-white' : 'hover:bg-muted'
      }`}
    >
      {children}
    </button>
  );
}
