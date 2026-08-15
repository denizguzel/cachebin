# Cachebin: Tauri 2 + React 19, TypeScript, Tailwind v4, Bun

## Development Commands

- `bun run dev` — Start the Vite dev server
- `bun run build` — Run lint, format, knip, typecheck, and Vite build
- `bun run lint` — Run Oxlint
- `bun run format` / `bun run format:check` — Format with Oxfmt / verify
- `bun run knip` — Run unused code/dependency analysis
- `bun tauri dev` — Run the app inside Tauri
- `cargo check` / `cargo test` — Rust checks and unit tests (from `src-tauri/`)

## Tech Stack

- **Desktop shell:** Tauri 2
- **UI:** React 19 + TypeScript + Tailwind CSS v4 + shadcn/ui components + Lucide icons
- **Tooling:** Vite, Bun (exact versions), React Compiler, date-fns, Oxlint, Oxfmt, Knip
- **Backend:** Rust (Tauri commands)

## Directory Structure

```
src/
├── components/   # One directory per component plus index.ts
├── data/         # Static and mock data
├── hooks/        # Shared hooks
├── lib/          # Shared helpers (format, date, selectors, environment)
└── types/        # One type per file, flat
src-tauri/src/    # Rust: lib.rs wiring plus one module per concern
```

## Code Conventions

- **Rust:** `lib.rs` is wiring only; thin commands delegate to logic modules; serde models with camelCase JSON and `#[serde(default)]`; `#[cfg(test)]` unit tests in every module; `tauri::Result` and `?`
- **TypeScript:** prefer `interface` over `type` (string-literal unions/primitives stay `type`); `import type`; no `any`; no enums (string-literal unions instead)
- **React:** functional components; inner methods as `const` arrows; no `useMemo`/`useCallback` (React Compiler is enabled); folder-per-component with `index.ts` (exception: `src/components/ui/` holds shadcn-generated components and follows shadcn conventions)
- **Styling:** shadcn components + Tailwind utilities only; App.css holds design tokens (`:root`/`.dark`) and global base resets, no component CSS; semantic tokens (`bg-background`, `text-muted-foreground`, `text-safe`) instead of raw values
- **Hooks:** flat in `src/hooks/`; one `useHookName.ts` per hook; parameters go through an exported `useHookNameProps` interface; no explicit return type
- **Formatting:** Oxfmt (printWidth 120, single quotes); LF line endings via `.gitattributes`
- **Skills:** code rules live in `.agents/skills/` (react-component-creation, rust-tauri-code, typescript-conventions)

## Verification

- For frontend changes run `bun run lint`, `bun run format:check`, `bun run knip`, and `bun run build`
- For Rust changes run `cargo test` and `cargo check`
- Do not claim completion without fresh command evidence
