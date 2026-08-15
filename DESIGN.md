# Design

## Source of truth

- Status: Active
- Last refreshed: 2026-08-14
- Primary product surfaces: Overview, Developer caches, Projects, Large files, History, Settings
- Evidence reviewed: the original macOS Swift codebase, the current Tauri scaffold, and `https://vercel.com/design.md`

## Brand

- Personality: Calm, precise, trustworthy, developer-native.
- Trust signals: Local-first language, explicit cleanup risk, recoverable actions, visible scan state, and no invented telemetry.
- Avoid: Marketing hype, decorative gradients, dense card walls, ambiguous destructive actions, and Vercel brand assets or authorship claims.

## Product goals

- Goals: Make reclaimable developer storage immediately legible, explain why each item is safe or risky, and make review-before-cleanup fast.
- Non-goals: Permanent deletion without confirmation, opaque background cleanup, a full filesystem explorer, or a generic system-optimizer dashboard.
- Success signals: A user can identify the largest safe opportunity in one viewport, understand the risk before acting, and recover from an accidental cleanup through Trash/history.

## Personas and jobs

- Primary personas: Developers using Windows and WSL2; engineers with multiple repositories and toolchains; maintainers of local build environments.
- User jobs: Find reclaimable caches, inspect stale project artifacts, review large files, and clean only what they understand.
- Key contexts of use: A compact desktop window, often while disk space is constrained and the user needs a quick, low-risk decision.

## Information architecture

- Primary navigation: Overview, Developer caches, Projects, Large files, History, Settings.
- Core routes/screens: One desktop shell initially; navigation changes the active workspace view without losing the shell context.
- Content hierarchy: Current storage state → cleanable total → largest opportunities → risk and action details → recent activity.

## Design principles

- Lead with evidence: Put the current cleanable amount and its basis before secondary controls.
- Make risk explicit: Safety, caution, and destructive states use text plus restrained semantic color.
- Prefer alignment over decoration: Use a shared grid, clear baselines, open space, and typography before borders or surfaces.
- Preserve agency: Review and confirmation precede cleanup; actions describe what will happen.
- Tradeoffs: The first pass uses representative data while the Rust scanner is being connected. Any provisional value must be easy to replace with typed command data.

## Visual language

- Color: Neutral monochrome foundation; green, amber, and red only for meaningful cleanup states and always paired with a label.
- Typography: Geist Sans for UI and data; Geist Mono for paths, identifiers, and command-like values.
- Spacing/layout rhythm: Tailwind utilities over a 12-column desktop composition that collapses to a single readable column on narrow windows.
- Shape/radius/elevation: Restrained 8–12px radii, thin borders only for grouping or interaction, no ornamental shadows.
- Motion: Still by default; short state transitions only when they explain scan or cleanup progress, with reduced-motion support.
- Imagery/iconography: Lucide icons only where they improve recognition; no decorative illustrations or icon tiles.

## Components

- Existing components to reuse: Tauri command bridge in `src-tauri/src/lib.rs`; the existing command will be replaced/extended by scanner commands later.
- New/changed components: shadcn/ui-compatible `Button` and `Badge`; dashboard shell, navigation, storage summary, opportunity list, and recent activity sections.
- Variants and states: Primary action, outline action, ghost navigation, destructive cleanup, safe/caution/risky status, loading, empty, error, and success.
- Token/component ownership: Tailwind v4 and CSS variables own tokens; shadcn components live under `src/components/ui`; page composition lives in `src/App.tsx` and `src/App.css`.

## Accessibility

- Target standard: WCAG 2.2 AA intent.
- Keyboard/focus behavior: Native buttons and links, visible `:focus-visible`, logical source order, and no hover-only actions.
- Contrast/readability: Semantic colors never carry meaning alone; body copy stays readable in light and dark modes.
- Screen-reader semantics: Landmarks, one descriptive `h1`, ordered headings, labelled progress meter, and text alternatives for visual bars.
- Reduced motion and sensory considerations: Respect `prefers-reduced-motion`; no flashing, autoplay, or decorative animation.

## Responsive behavior

- Supported breakpoints/devices: Desktop-first Tauri window, tablet widths, and narrow WSL/Windows windows.
- Layout adaptations: Sidebar becomes a compact top navigation; two-column evidence areas stack; tables/list rows retain readable minimum widths or scroll locally.
- Touch/hover differences: Actions remain fully keyboard and pointer accessible; hover adds emphasis only, not information.

## Interaction states

- Loading: Preserve the last known values and show a concise inline scanning state.
- Empty: Explain what is being scanned and what the user can do next; do not fill the screen with placeholder cards.
- Error: State whether the scan failed, preserve prior results, and offer retry/details.
- Success: Confirm the exact reviewed or moved amount and where it can be recovered.
- Disabled: Disable only actions that cannot be performed and explain why nearby.
- Offline/slow network, if applicable: The product is local-first; scanner work must not require network access.

## Content voice

- Tone: Direct, calm, technically literate, and specific.
- Terminology: Use “cleanable” for detected reclaimable space, “Move to Trash” for recoverable cleanup, and “Risk” for data-loss likelihood.
- Microcopy rules: Sentence case, concrete action labels, explicit units, no exaggerated claims, and no em dashes.

## Implementation constraints

- Framework/styling system: Tauri 2, React, TypeScript, Rust, Bun, Tailwind CSS v4, and shadcn/ui-compatible copy-owned components.
- Design-token constraints: Use CSS variables and Tailwind semantic utilities; do not add a second component library or a visible theme switcher.
- Performance constraints: Keep the shell lightweight, avoid chart/icon libraries beyond the selected Lucide package, and keep scanner data out of the render tree until needed.
- Compatibility constraints: Windows and WSL2 are first-class targets; Linux development may require GTK/WebKit native packages.
- Test/screenshot expectations: `bun run build` must pass after UI changes; run Tauri smoke checks on a host with native prerequisites before claiming command integration.

## Open questions

- [ ] Which Windows and WSL2 cache locations are included in the first Rust scanner contract?
- [ ] Should the initial shell use a full window or preserve a compact menu-bar-like mode on Windows?
- [ ] What exact cleanup confirmation and recovery-history model should be shared across Windows and WSL2?
