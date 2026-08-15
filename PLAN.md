# Cachebin Plan

## 1. Product goal

Build a local desktop application for Windows and WSL2 that surfaces developer caches and project artifacts and provides a safe review-before-cleanup flow.

The app adapts the core idea of the original macOS cleanup tool to the Windows developer environment:

- Shows what is taking up space on disk in a legible way.
- Classifies rebuildable caches with risk levels.
- Never moves or deletes files without user confirmation.
- Moves items to the Recycle Bin / Trash whenever possible.
- Manages Linux developer data inside WSL2 distributions from a Windows-side app.
- Requires no network access, telemetry or analytics.

## 2. Technology and platform decision

- Desktop shell: Tauri 2
- UI: React + TypeScript
- Styling: Tailwind CSS v4
- UI components: shadcn/ui-compatible, project-owned components
- Icons: Lucide
- Backend: Rust
- Package manager: Bun
- Package versions: exact in `package.json` and `bun.lock`
- Supported host: Windows 10/11
- Supported runtime environment: WSL2 + WSLg
- Windows build: Native Windows, or NSIS via `cargo-xwin` from WSL2

## 3. Product principles

- Local-first: data and scan results stay on the device.
- Evidence first: cleanable total, source and risk are visible before any action.
- Safe defaults: no automatic permanent deletion.
- Explicit risk: Safe, Caution and Risky are distinguished in text as well as color.
- Auditability: every cleanup is visible in history.
- Quiet, direct UI: plain monochrome surfaces, Geist typography, low visual noise.
- Single responsibility: pages, components, data models and Rust commands live in separate layers.

## 4. Information architecture

Main navigation:

1. Overview
2. Developer caches
3. Projects
4. Large files
5. History
6. Settings

### Overview

- Total disk usage
- Cleanable space
- Last scan time and status
- Largest cleanup opportunities
- Risk summary
- Recent cleanup activity

### Developer caches

Initial target tools and ecosystems:

- Node.js: npm, pnpm, Yarn, Bun
- Rust: Cargo
- Go: module and build cache
- Java/Kotlin: Gradle, Maven
- Python: pip, uv, Poetry, virtualenv caches
- Docker: image and build cache, and unsafe volume distinction
- .NET: NuGet and build outputs
- Android: Gradle, SDK cache and emulator data
- VS Code and JetBrains caches
- Playwright/Puppeteer browser caches
- Distribution-specific cache paths inside WSL2

Each item should show:

- Name and location
- Size
- Last accessed or modified time
- Rebuildability description
- Risk level
- Which environment it lives in: Windows or a WSL2 distribution

### Projects

Finds stale build and tooling artifacts inside projects:

- `node_modules`
- `dist`, `build`
- `.next`, `.nuxt`, `.turbo`, `.vite`
- `.venv`, `__pycache__`, `.pytest_cache`
- `target`
- `bin`, `obj`
- `Pods`, `DerivedData`
- `Library`, `Temp`, `.godot`
- Docker or tool-specific project caches

The Projects screen should group artifacts under the same project and let the user review them individually or as a group.

### Large files

- Largest files within a user-defined scope
- File path and size
- File type
- Windows/WSL2 source
- Confirmation before moving to the Recycle Bin / Trash

### History

- Moved items
- Operation time
- Total reclaimed space
- Source environment
- Success or error result of the operation

## 5. Risk model

### Safe

Rebuildable caches with low loss potential. Examples: package manager download cache, Cargo registry cache, build cache.

### Caution

Data that is expensive to re-download or rebuild. Examples: Android emulator image, large Docker image cache, IDE indexes.

### Risky

Paths that may hold user data, local project state, or content that cannot be easily regenerated. Examples: Docker volume, undefined files inside a project folder, runtime data.

Risk level is not shown by color alone; it is backed by a label, description and confirmation text.

## 6. Rust backend boundary

The React side only manages screen state and user interaction. File system, platform branching, scanning and moving operations happen on the Rust side.

Proposed commands:

- `scan_storage`
- `scan_caches`
- `scan_projects`
- `scan_large_files`
- `move_to_recycle_bin`
- `get_cleanup_history`
- `clear_cleanup_history`
- `get_platform_info`

Command contracts:

- Use explicit request/response types via serde.
- Errors should return meaningful, classified error codes to the user.
- Scan results must include file path, environment, size, risk, description and time fields.
- Windows and WSL2 paths should be normalized while showing the user the source path.
- Permanent deletion is not supported by default.
- Long scans must be cancellable.

## 7. WSL2 architecture

WSL2 support is handled as follows in the first version:

1. The app on the Windows host lists WSL distributions.
2. An accessible filesystem root is determined for each distribution.
3. The scan is applied by Rust to Windows and WSL2 sources with separate identities.
4. Results carry a source label such as `Windows`, `Ubuntu`, `Debian`.
5. Access errors do not stop the whole app; the affected source is shown as a separate error state.
6. If a WSL distribution is not running, the user is offered a start or retry state.

Performance and permission behavior for accessing Linux files inside WSL will be measured separately. For project files, `/mnt/c` and in-distribution Linux paths will be kept apart.

## 8. UI implementation plan

### Phase 1: Shell and design system

- [x] Tauri + React + Rust + Bun scaffold
- [x] Exact package install
- [x] Tailwind v4 integration
- [x] shadcn configuration
- [x] Geist fonts
- [x] Responsive sidebar and Overview prototype
- [x] `DESIGN.md` design contract
- [x] Split components away from god components

### Phase 2: Data models and Rust scanning foundation

- [x] Shared `ScanResult`, `CacheEntry`, `ProjectArtifact`, `LargeFile`, `RiskLevel` types
- [x] Windows platform info
- [x] WSL2 distribution discovery
- [x] First Windows cache scanners
- [x] First WSL2 cache scanners
- [x] Tauri invoke command contracts
- [x] Scan progress and cancellation support
- [x] Feed the React Overview screen with real Tauri responses
- [x] Rust unit tests (every module) + Windows `cargo test` manifest workaround
- [x] Oxlint / Oxfmt / Knip integration and build pipeline

### Phase 3: Developer caches screen

- [x] List real scan results
- [x] Category and risk filters
- [x] Sort by size
- [x] Cache descriptions
- [x] Review selected items
- [x] Cleanup confirmation flow (design; Recycle Bin move lands in Phase 4)

### Phase 4: Safe cleanup

- [x] Windows Recycle Bin integration
- [x] Safe move strategy for WSL2
- [x] Risk-based confirmation screens
- [x] Operation progress state
- [x] Partial failure report
- [x] Cleanup history recording

### Phase 5: Projects and Large files

- [x] Project root discovery
- [x] Artifact grouping
- [x] Large file scanning
- [ ] Path scope settings
- [ ] Exclude/include rules

### Phase 6: Settings and distribution

- [x] Windows locations to scan
- [x] WSL distributions to scan
- [x] Default risk filter
- [x] Scan scheduling (automatic startup scan)
- [x] Local data and history settings (clear history)
- [x] Windows NSIS installer
- [~] Linux AppImage/DEB build (config ready; build needs a Linux environment)
- [x] Signing and release pipeline (GitHub Actions: `tauri-action` + Tauri signing secrets)

## 9. Testing and verification

At every phase:

- `bun install --frozen-lockfile`
- `bun run lint`
- `bun run format:check`
- `bun run knip`
- `bun run build` (lint + format + knip + typecheck + Vite build)
- `cargo check`
- Rust unit tests
- Tauri smoke test
- Windows native smoke test
- WSL2 access test
- Keyboard navigation and focus test
- Light/dark theme check
- Narrow window responsive check

The file-move feature should specifically be tested for:

- Permission denied
- File changing during scan
- WSL distribution being stopped
- Empty results
- Partial failure
- User cancellation
- History record after moving to the Recycle Bin / Trash

## 10. First real delivery criteria

The first functional milestone must complete this flow:

1. The user opens the Overview screen.
2. Windows and at least one WSL2 distribution are scanned.
3. Cleanable space is shown with source and risk information.
4. The user filters items on the Developer caches screen.
5. The user selects one or more Safe items.
6. After an explicit confirmation, items are moved to a recoverable location.
7. The operation result is visible on the History screen.
8. Every error indicates which source it affected.

## 11. Current status and next step

Phase 2 complete: Rust scanning foundation — shared data models (`models.rs`), Windows platform info (`get_platform_info`), WSL2 distribution discovery (`wsl.rs`), the first Windows/WSL2 cache catalog (`scanner.rs`), the `scan_storage`/`cancel_scan` commands and `scan://progress` + cancellation support. The React Overview screen is fed with real Tauri responses; shadcn `Skeleton` is shown while loading, and mock data was fully removed. Every Rust module has unit tests; the Common-Controls v6 manifest issue that broke `cargo test` on Windows was solved via `build.rs` + `windows-app-manifest.xml`. Tooling (Oxlint/Oxfmt/Knip) and the build pipeline are integrated, and `.gitattributes` normalizes LF.

Phase 3 complete: Developer caches screen — `CachesPage` listing real scan results (folder-per-component: `CacheToolbar`, `CacheList`, `CacheRow`, `CacheReviewDialog`), category/risk filters, size/name/date sorting, source (Windows / WSL distribution) and risk label display, selection → review flow and the cleanup confirmation design (dialog; the "Move to Trash" button stayed disabled until the Recycle Bin integration).

Scan performance: the original macOS app's batch enumerator approach was ported to Windows — `size::dir_size` now does one query per directory via `NtQueryDirectoryFile` + `FileIdBothDirectoryInformation` (no per-file syscall; also valid over WSL2/9P), and the Swift project-artifact scan was ported through the `scan_projects` command (marker/glob, skip-dir, stop recursion once found, >10 MB threshold, top-100). **Known limitation — deliberately not done:** Windows hardlink counting (`NumberOfLinks`) was left out in favor of speed; the macOS batch API provides link counts for free while Windows needs a per-file handle (each a network round-trip on WSL/9P). NtQueryDirectoryFile directory-information classes also do not include link counts. As a result, pnpm/Bun/Cargo hardlink stores on Windows are reported by logical size (slightly inflated); on Unix they are counted correctly with the free `nlink` from `stat`.

Next step: **Phase 6 — distribution.** The settings page is done (below); remaining work is distribution: Windows NSIS installer, signing and release pipeline, Linux AppImage/DEB build.

Phase 4 complete: Safe cleanup — `cleanup.rs` offers two-tier moving: local Windows paths go to the Recycle Bin via the `trash` crate, while WSL paths (`\\wsl.localhost\<distro>\...`) move to the in-distro XDG Trash (`~/.local/share/Trash`) through `wsl.exe -d <distro> sh -s` (script via stdin, path base64-encoded — no shell escaping). `move_to_trash` now returns `TrashReport { moved, failed }`: on partial failure successful items keep moving and failures are reported in the report; `cleanup://progress` events are emitted throughout. `CacheReviewDialog` includes risk-based confirmation (an acknowledgment checkbox for Risky items, "Moving X of Y" progress during the move). Cleanup results are recorded in the activity history with the reclaimed bytes. 77 Rust unit tests; `cargo check` and the frontend build are warning-free.

Phase 5 complete: Projects and Large files — `scan_projects` results are grouped by project root on `ProjectsPage` (`ProjectGroupCard` per project; individual or project-wide selection, risk label, source badge), and `scan_large_files` scans Windows workspace roots + running WSL homes via a `NtQueryDirectoryFile` batch walk for files ≥100 MB (type label: Archive/Video/Audio/Image/Database/Executable/Code/Docs/Font; unknown and numeric extensions collapse to `Other`). The selection → review → Trash flow is shared with the caches pattern (`useCleanupSelection` hook + `SelectionBar` + generalized `CacheReviewDialog`). Long-running scan commands are async via `spawn_blocking` — the UI does not freeze; the header button shows live progress. The large-files cache is persistent with a `status` (scanning/ready) field: the page does not rescan on every open, it reads from cache; a manual "Scan large files" triggers the scan; the `scanning` state survives a refresh via polling; after a crash the stale marker is reset to `ready` on startup. 92 Rust unit tests; `cargo check` and the frontend build are warning-free.

Phase 6 (Settings) complete: the `Settings` model was extended (`scan_dirs`, `disabled_distros`, `default_risk_filter`, `auto_scan_on_startup`) and validated via `sanitize` (only known Windows directories + a valid risk filter). `get_settings`/`update_settings`/`get_scan_dir_options` commands were added and wired into the scan logic: `projects::scan`/`large::scan` scan the configured Windows directories, and `scan_storage`/`scan_large_files` skip disabled distributions. `SettingsPage` — Windows scan location checkboxes (options come from a single Rust source), WSL distribution toggles, default risk filter, automatic startup scan, history clearing. Duplication guards: the scan-dir list has a single source (`settings::SCAN_DIR_OPTIONS`), and `format_time`/`now_rfc3339` moved to a shared `time.rs`. 94 Rust unit tests; `cargo check` and the frontend build are warning-free. Remaining distribution work: NSIS installer, signing/release pipeline, Linux AppImage/DEB.
