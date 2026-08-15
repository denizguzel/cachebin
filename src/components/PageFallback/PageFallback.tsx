import { Loader2 } from 'lucide-react';

export function PageFallback() {
  return (
    <div className="grid min-h-[40vh] place-items-center">
      <Loader2 className="animate-spin text-muted-foreground" size={20} />
    </div>
  );
}
