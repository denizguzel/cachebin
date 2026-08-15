import { AlertTriangle } from 'lucide-react';
import { Checkbox } from '@/components/ui/checkbox';

export interface RiskyWarningProps {
  acknowledged: boolean;
  onChange: (checked: boolean) => void;
}

export function RiskyWarning({ acknowledged, onChange }: RiskyWarningProps) {
  return (
    <div className="flex items-start gap-2.5 rounded-[8px] border border-danger-border bg-danger-surface p-3">
      <AlertTriangle className="mt-0.5 shrink-0 text-danger" size={15} />
      <div className="min-w-0">
        <p className="text-xs font-semibold text-danger-strong">Risky items selected</p>
        <p className="mt-0.5 text-xs leading-5 text-muted-foreground">
          Risky items may contain important data or local project state. Moving them to Trash keeps them recoverable
          only until the Trash is emptied.
        </p>
        <label className="mt-2.5 flex items-start gap-2 text-xs text-muted-foreground" htmlFor="risk-ack">
          <Checkbox
            id="risk-ack"
            className="mt-0.5"
            checked={acknowledged}
            onCheckedChange={(checked) => onChange(checked === true)}
          />
          <span className="leading-5">I understand and still want to move Risky items to Trash.</span>
        </label>
      </div>
    </div>
  );
}
