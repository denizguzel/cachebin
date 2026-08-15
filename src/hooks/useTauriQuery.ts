import { useEffect, useState } from 'react';
import { invoke, type InvokeArgs } from '@tauri-apps/api/core';

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

  useEffect(() => {
    if (!enabled) {
      setIsPending(false);
      return;
    }

    let active = true;
    setIsPending(true);
    setIsFetching(true);

    let promise: Promise<TResult>;
    try {
      promise = dedupeInvoke(command, args) as Promise<TResult>;
    } catch (err) {
      if (active) {
        setError(err);
        setIsFetching(false);
        setIsPending(false);
      }
      return () => {
        active = false;
      };
    }

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
