export interface MetricProps {
  label: string;
  value: string;
  detail: string;
}

export function Metric({ label, value, detail }: MetricProps) {
  return (
    <div className="min-w-0 px-5 py-5">
      <p className="m-0 text-[11px] text-muted-tertiary">{label}</p>
      <p className="mb-[3px] mt-[7px] font-mono text-[20px] tracking-[-0.04em] text-foreground">{value}</p>
      <p className="m-0 text-[11px] text-muted-tertiary">{detail}</p>
    </div>
  );
}
