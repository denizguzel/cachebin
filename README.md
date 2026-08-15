# Cachebin

Developer-focused disk cleanup tool for Windows and WSL2, built with Tauri 2, React, TypeScript, Rust, and Bun.

Reclaimable developer storage made legible: find caches, stale project artifacts, and large files, understand the risk before cleaning, and recover from mistakes through Trash and history.

## Prerequisites

- **Bun** (>= 1.1) - package manager and JS runtime
- **Rust** (stable) - via [rustup](https://rustup.rs/)
- **Tauri system dependencies**:
  - Windows: [Microsoft C++ Build Tools](https://visualstudio.microsoft.com/visual-cpp-build-tools/) with the "Desktop development with C++" workload and the Windows 10/11 SDK (WebView2 is preinstalled on modern Windows 11)
  - Linux: GTK/WebKit native packages (see [Tauri docs](https://tauri.app/start/prerequisites/) for your distro)

## Setup

```sh
bun install
```

Note: on Windows, open a **new terminal** after installing Rust so `cargo` is on your PATH, otherwise `bun tauri dev` will fail with `program not found` for `cargo`.

## Run

Start the app in development mode with hot reload:

```sh
bun tauri dev
```

## Build & package

```sh
bun run build        # typecheck + frontend build
bun tauri build      # production bundle and installers
```

## Scripts

| Script            | Description                                 |
| ----------------- | ------------------------------------------- |
| `bun run dev`     | Run the Vite frontend only (no Tauri shell) |
| `bun run build`   | Typecheck and build the frontend            |
| `bun run preview` | Preview the built frontend                  |
| `bun tauri dev`   | Run the full desktop app in dev mode        |
| `bun tauri build` | Build a production desktop bundle           |

## Project structure

```
src/              React frontend (React 19, Tailwind v4, shadcn/ui-style components)
src-tauri/        Rust backend (Tauri 2 commands, capabilities, icons)
src-tauri/src/    Rust source (command bridge, scanner)
DESIGN.md         Design and product decisions
PLAN.md           Implementation plan
```

## Tech stack

- Tauri 2, Rust
- React 19, TypeScript
- Vite 8, Tailwind CSS v4, shadcn/ui-compatible components
- Bun

## Platform targets

Windows and WSL2 are first-class targets. Linux development may require additional native packages.
