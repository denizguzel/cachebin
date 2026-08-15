export interface TrashFailure {
  path: string;
  error: string;
}

export interface TrashReport {
  moved: string[];
  failed: TrashFailure[];
}
