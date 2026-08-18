import { useState } from 'react';

export interface useOnChangeProps<T> {
  value: T;
  onNext: (value: T) => void;
}

// Render-phase state adjustment: runs `onNext` once when `value` changes identity,
// without an effect (which would trigger a cascading extra render).
export function useOnChange<T>({ value, onNext }: useOnChangeProps<T>) {
  const [prev, setPrev] = useState(value);

  if (prev !== value) {
    setPrev(value);
    onNext(value);
  }
}
