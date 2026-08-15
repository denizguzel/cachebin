import { invoke } from '@tauri-apps/api/core';
import { useKey } from 'react-use';

export function useZoomShortcuts() {
  useKey(
    (event) => (event.ctrlKey || event.metaKey) && (event.key === '+' || event.key === '='),
    (event) => {
      event.preventDefault();
      void invoke('zoom_by', { delta: 0.1 });
    },
  );

  useKey(
    (event) => (event.ctrlKey || event.metaKey) && (event.key === '-' || event.key === '_'),
    (event) => {
      event.preventDefault();
      void invoke('zoom_by', { delta: -0.1 });
    },
  );

  useKey(
    (event) => (event.ctrlKey || event.metaKey) && event.key === '0',
    (event) => {
      event.preventDefault();
      void invoke('reset_zoom');
    },
  );
}
