---
name: typescript-conventions
description: Use when writing or editing TypeScript code in this Tauri + React + TypeScript project. Prefer exported `interface` over `type` for object shapes (data models, props, state, options) because interfaces give better error messages, merge declarations, and are the idiomatic default in this codebase; reserve `type` for cases only `type` can express (string-literal unions, tuples, primitives, utility types, object intersections). Also enforces type-only imports with `import type`, string-literal unions instead of enums, and no `any`. Trigger whenever the user mentions types/interfaces, type safety, a data model, props shape, or when adding/refactoring shared types or component props.
---

# TypeScript Conventions

This project treats type safety as a first-class concern. When you introduce or refactor any TypeScript shape — data models coming from the Rust commands, component props, hook return types, options objects — follow these rules.

## Prefer `interface` over `type` for object shapes

Declare object-shaped types with `export interface`, not `export type`:

```ts
// prefer
export interface CacheEntry {
  id: string;
  sizeBytes: number;
  risk: RiskLevel;
}

// avoid
export type CacheEntry = {
  id: string;
  sizeBytes: number;
  risk: RiskLevel;
};
```

Why: interfaces produce more readable compiler diagnostics, support declaration merging, and are the convention the rest of the codebase and the React components (props) already follow. `type` is not "wrong" — it is just not the default here.

## Use `type` only when `interface` cannot express it

Keep `export type` for shapes only `type` supports:

- string-literal unions — `export type RiskLevel = "safe" | "caution" | "risky";`
- tuples — `export type Size = [number, number];`
- primitives / utility types — `export type Id = string;` or `export type MaybeEntry = CacheEntry | null;`
- object intersections — `export type X = A & B;` (for object combination prefer extending an interface: `interface X extends A`)

## Component props are exported interfaces

Every component declares its props as an exported `interface ComponentNameProps` in the same file (this also matches the `react-component-creation` skill). Props never use `type`.

```ts
export interface ButtonProps {
  label: string;
  onClick: () => void;
}
```

## Type-only imports use `import type`

When importing something used only as a type, use `import type` so the bundler can erase it and the type system stays explicit:

```ts
import type { CacheEntry, RiskLevel } from '@/types/scan';
```

## No enums

Use string-literal unions instead of TypeScript `enum`:

```ts
// prefer
export type ScanState = 'idle' | 'pending' | 'ready';

// avoid
export enum ScanState {
  Idle = 'idle',
  Pending = 'pending',
  Ready = 'ready',
}
```

## No `any`

Never introduce `any`; it removes the type safety this project relies on. If a value's type is unknown, prefer `unknown` with explicit narrowing, or a precise interface.

## `src/types/` holds only types, never values

Files under `src/types/` contain type declarations only — `interface` and `type` aliases, nothing else. No functions, constants, or helpers may live there; TypeScript would still compile, but the folder's contract is "pure type definitions."

### One type declaration per file, flat

`src/types/` is flat (no subfolders) and each file declares exactly **one** type. A file name is the lower-kebab-case of the type it declares, so `CacheEntry` → `cache-entry.ts`, `ScanProgress` → `scan-progress.ts`, `RiskLevel` → `risk-level.ts`. A type that references another imports it with a relative `import type`:

```ts
// src/types/cache-entry.ts
import type { Environment } from "./environment";
import type { RiskLevel } from "./risk-level";

export interface CacheEntry {
  id: string;
  ...
}
```

Consumers import directly from the declaring file — no barrel, no `index.ts`:

```ts
import type { CacheEntry } from '@/types/cache-entry';
import type { ScanState } from '@/types/scan-state';
```

The only `index.ts` re-exports in the project live inside component folders (see the `react-component-creation` skill); `src/types/` never gets one.

- Keep a type's companion helper (a function that formats, labels, or derives something from the type) in `src/lib/`. A type in `src/types/` never ships display/format logic; put that logic in `src/lib/` next to the generic utilities.
- Generic utilities (date formatting, byte formatting) belong in `src/lib/` (e.g., `src/lib/date.ts`, `src/lib/format.ts`), written to be reused project-wide rather than named after a single caller.

## Conventions for model files

- Shared shapes that mirror the Rust backend use `camelCase` field names to match the serde `camelCase` contract, and each model is an exported `interface`.
- Derived UI shapes (e.g., `Opportunity` computed by a selector) and event payloads (e.g., `ScanProgress`) are still types: they get their own file under `src/types/` and are imported directly by consumers, never declared inside the `lib/` or `hooks/` file that produces them.
