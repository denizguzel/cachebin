export type ActivityKind = 'scan' | 'cleanup';
export type ActivityStatus = 'pending' | 'success' | 'error';

export interface ActivityEvent {
  id: string;
  kind: ActivityKind;
  status: ActivityStatus;
  at: string;
  bytes: number;
}
