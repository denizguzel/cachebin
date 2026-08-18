import { useEffect, useState } from 'react';
import { invoke, type InvokeArgs } from '@tauri-apps/api/core';
import { useOnChange } from '@/hooks/useOnChange';

const inFlight = new Map<string, Promise<unknown>>();

function dedupeInvoke(command: string, args: InvokeArgs | undefined): Promise<unknown> {
  const key = `${command}:${JSON.stringify(args ?? null)}`;
  const existing = inFlight.get(key);
  if (existing) {
    return existing;
  }

  const promise = invoke(command, args);
  inFlight.set(key, promise);
  void promise.finally(() => {
    inFlight.delete(key);
  });
  return promise;
}

export interface useTauriQueryProps<TArgs extends InvokeArgs | undefined> {
  command: string;
  args?: TArgs;
  enabled?: boolean;
}

export function useTauriQuery<TArgs extends InvokeArgs | undefined, TResult>({
  command,
  args,
  enabled = true,
}: useTauriQueryProps<TArgs>) {
  const [data, setData] = useState<TResult | null>(null);
  const [error, setError] = useState<unknown>(null);
  const [isPending, setIsPending] = useState(enabled);
  const [isFetching, setIsFetching] = useState(false);
  const [requestId, setRequestId] = useState(0);
  const requestKey = `${command}:${JSON.stringify(args ?? null)}:${requestId}:${enabled}`;

  useOnChange({
    value: enabled,
    onNext: (next) => {
      if (!next) {
        setIsPending(false);
        setIsFetching(false);
      }
    },
  });

  useOnChange({
    value: requestKey,
    onNext: () => {
      if (enabled) {
        setIsPending(true);
        setIsFetching(true);
      }
    },
  });

  useEffect(() => {
    if (!enabled) {
      return;
    }

    let active = true;

    const promise = dedupeInvoke(command, args) as Promise<TResult>;

    void promise
      .then((result) => {
        if (!active) return;
        setData(result);
        setError(null);
      })
      .catch((err: unknown) => {
        if (!active) return;
        setError(err);
      })
      .finally(() => {
        if (active) {
          setIsFetching(false);
          setIsPending(false);
        }
      });

    return () => {
      active = false;
    };
  }, [command, args, enabled, requestId]);

  return {
    data,
    error,
    isPending,
    isFetching,
    refetch: () => setRequestId((id) => id + 1),
  };
}
