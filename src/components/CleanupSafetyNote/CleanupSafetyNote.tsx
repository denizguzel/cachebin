import { AlertTriangle } from 'lucide-react';

export function CleanupSafetyNote() {
  return (
    <section
      className="mt-[42px] flex items-start gap-[9px] border-t border-border pt-[18px] text-[11px] leading-relaxed text-muted-tertiary"
      aria-label="Cleanup safety note"
    >
      <AlertTriangle size={16} className="shrink-0 text-caution" />
      <p className="m-0 max-w-[720px]">
        <strong className="font-medium text-muted-foreground">Review before cleaning.</strong> Safe items can be
        rebuilt, but Docker volumes, project data, and active toolchains may require extra care.
      </p>
    </section>
  );
}
