import type * as React from 'react';

export interface SettingsCardProps {
  title: string;
  description: string;
  className?: string;
}

export function SettingsCard({
  title,
  description,
  className = '',
  children,
}: React.PropsWithChildren<SettingsCardProps>) {
  return (
    <section className={`border border-border rounded-[10px] ${className}`}>
      <div className="border-b border-border px-5 py-4">
        <h2 className="text-sm font-semibold">{title}</h2>
        <p className="mt-1 text-xs leading-5 text-muted-tertiary">{description}</p>
      </div>
      <div className="px-5 py-4">{children}</div>
    </section>
  );
}
