---
name: react-hooks-creation
description: Use when creating or editing React hooks in this Tauri + React + TypeScript project. Hooks live flat in `src/hooks/` with one `useHookName.ts` file per hook. Enforces the `use` camelCase prefix, a single destructured props object typed by an exported `interface useHookNameProps` when the hook takes parameters, no explicit return type annotation (TypeScript infers it), inner helpers as `const` arrow functions, no useCallback/useMemo (React Compiler enabled), and extracting shared stateful logic into its own hook. Trigger whenever the user mentions creating/editing a hook, extracting shared logic, or a `use*` function.
---

# React Hook Creation

## Folder layout

- Hooks live **flat** in `src/hooks/` — no subfolders.
- One hook per file, named `useHookName.ts` (camelCase, `use` prefix): `useScan.ts`, `useTauriQuery.ts`.

## Hook shape

- Hooks are functions named with the `use` prefix.
- If a hook takes parameters, accept a **single props object** and type it with an exported `interface useHookNameProps` declared in the same file:

  ```ts
  export interface usePollProps {
    intervalMs?: number;
    enabled?: boolean;
  }

  export function usePoll({ intervalMs = 1000, enabled = true }: usePollProps) {
    // ...
  }
  ```

- Hooks that need no configuration take no arguments.
- Generics are fine when the hook is type-safe over inputs/outputs (for example a command wrapper); put the generic on the props interface: `interface useTauriQueryProps<TArgs>`.

## Return type

- **Do not** annotate the return type. TypeScript infers it from the returned object, so there is nothing to maintain by hand. Keep the returned shape small and readable.

## State, effects, and helpers

- Inner helpers are `const` arrow functions; never `function` declarations.
- Effects return a cleanup when they subscribe or spawn async work; guard async state updates with an `active` flag so unmounted hooks do not set state.
- Do not add `useMemo` or `useCallback` — the React Compiler is enabled.

## Composition

- One hook per file. If a hook outgrows a single responsibility, extract the shared logic into its own hook.
