---
name: react-component-creation
description: Use when creating or editing React components in this Tauri + React + TypeScript project. Enforces the folder-per-component layout (each component in its own folder under components/ with an index.ts that re-exports the component), functional components with an interface ComponentNameProps, const arrow functions for methods inside components, no unnecessary useMemo/useCallback (React Compiler is enabled), one component per file, flat components/ structure, avoiding god components, Tailwind-first styling (shadcn components + Tailwind utilities instead of custom CSS), and excludes shadcn components under src/components/ui/ which follow shadcn conventions instead.
---

# React Component Creation

## Folder layout

- Every component lives in its own folder: `src/components/<ComponentName>/`.
- The folder contains exactly one component file `<ComponentName>.tsx` — PascalCase, matching the exported component's name — plus an `index.ts`.
- `index.ts` re-exports the component:

  ```ts
  export * from './ComponentName';
  ```

- `components/` stays **flat**: no nested feature folders. Each new component gets a new top-level folder.
- **Exception: `src/components/ui/`** holds shadcn-generated components. These follow shadcn conventions (kebab-case files like `button.tsx`, exported via the registry CLI) and are **excluded from this skill's rules**. Never hand-roll a component that already exists in `src/components/ui/` — add or reuse it instead.

## Component shape

- Components are **functions**, never classes.
- Props are type-safe via an exported interface named `ComponentNameProps`, declared in the same file:

  ```tsx
  // src/components/ActivitySection/index.ts
  export * from './ActivitySection';

  // src/components/ActivitySection/ActivitySection.tsx
  import { Button } from '@/components/ui/button';

  export interface ActivitySectionProps {
    onOpenHistory: () => void;
  }

  export function ActivitySection({ onOpenHistory }: ActivitySectionProps) {
    return (
      <Button variant="ghost" size="sm" onClick={onOpenHistory}>
        History
      </Button>
    );
  }
  ```

- Consumers import from the folder (resolved through `index.ts`): `import { ActivitySection } from '@/components/ActivitySection';`
- Use `import type { ... }` for type-only imports.
- **Children**: when a component accepts children, derive them from `React.PropsWithChildren` instead of declaring `children: ReactNode`:

  ```tsx
  // src/components/SettingsCard/SettingsCard.tsx
  import type * as React from 'react';

  export interface SettingsCardProps extends React.PropsWithChildren {
    title: string;
    description: string;
  }

  export function SettingsCard({ title, description, children }: SettingsCardProps) {
    return (
      <section>
        <h2>{title}</h2>
        <p>{description}</p>
        {children}
      </section>
    );
  }
  ```

  Never write `children: ReactNode` in a props interface.

## Styling

- **Style with Tailwind utilities and shadcn components** — not custom CSS. App.css holds only design tokens (`:root`/`.dark` variables), the `@theme inline` mapping, and global base resets.
- **Use existing shadcn primitives before custom markup**: `Button`, `Badge`, `Dialog`, `Select`, `ToggleGroup`, `Checkbox`, `Table`, `Toast` (sonner `toast()`), `Separator`, `Skeleton`, etc. Only reach for a plain `div` when no shadcn component fits.
- Prefer semantic tokens over raw values: `bg-background`, `text-foreground`, `text-muted-foreground`, `bg-muted`, `border-border`, `text-safe`, `text-caution`, `text-danger` — never `bg-blue-500` or a one-off hex.
- Use `flex` + `gap-*` for spacing, `size-*` for equal width/height, `truncate` instead of manual ellipsis classes, and `cn()` for conditional classes.
- Do **not** add new component classes to App.css. If a layout needs repeated classes, prefer composing shadcn components or small Tailwind utilities inline.

## Methods inside components

- Any handler or helper defined inside a component is a `const` arrow function:

  ```tsx
  const handleKeyDown = (event: KeyboardEvent) => { ... };
  const selectView = (view: View) => { ... };
  ```

- Never use `function` declarations for inner methods.

## Memoization

- **Do not** add `useMemo` or `useCallback`. The React Compiler is enabled, so manual memoization is redundant and adds noise.
- Only reach for them if profiling shows a real bottleneck.

## Composition and size

- One component per file; no multi-component files.
- Avoid god components: if a component outgrows a single responsibility, extract smaller components, each in its own folder.
- Keep the render tree readable; prefer composition over prop drilling.
