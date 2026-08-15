import { Trash2 } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { SettingsCard } from '@/components/SettingsCard';

export interface LocalDataSectionProps {
  onClearHistory: () => void;
}

export function LocalDataSection({ onClearHistory }: LocalDataSectionProps) {
  return (
    <SettingsCard title="Local data" description="Clear activity history recorded on this device.">
      <div className="flex justify-end">
        <Button variant="outline" size="sm" onClick={onClearHistory}>
          <Trash2 size={14} /> Clear history
        </Button>
      </div>
    </SettingsCard>
  );
}
