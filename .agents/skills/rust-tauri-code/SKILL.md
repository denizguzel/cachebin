---
name: rust-tauri-code
description: Use when writing or refactoring Rust code in the src-tauri side of this Tauri 2 project. Enforces a modular structure (lib.rs as wiring only; one module per concern: commands.rs, menu.rs, settings.rs, zoom.rs, window.rs), thin Tauri commands that delegate to logic modules, single-direction module dependencies, managed state via app.manage and tauri State, serde-based typed settings with #[serde(default)] persisted to app_data_dir/settings.json, and tauri::Result error handling.
---

# Rust (Tauri 2) Code Conventions

## Module layout

- `src/lib.rs` is **wiring only**: `tauri::Builder`, `.invoke_handler`, `.setup`, `.on_menu_event`. No business logic.
- One module per concern, each in its own file:
  - `commands.rs` — Tauri commands (thin; delegate to logic modules)
  - `menu.rs` — native menu construction plus menu item id constants
  - `settings.rs` — typed persisted settings
  - `zoom.rs` — zoom state and logic
  - `window.rs` — window sizing logic
- `src/main.rs` only calls `cachebin_lib::run()`.

## Commands

- Commands are thin and delegate:

  ```rust
  #[tauri::command]
  pub fn zoom_by(delta: f64, app: AppHandle) {
      zoom::zoom_by(&app, delta);
  }
  ```

- Register with `tauri::generate_handler![commands::zoom_by, ...]`.

## Dependencies and state

- Keep dependencies **single-direction**, no cycles: `settings` <- `zoom` <- `commands`.
- Use `AppHandle` + `Manager` (`app.state::<T>()`) for shared state.
- Manage state in `.setup` via `app.manage(...)`.
- State types are thin newtypes around `Mutex`:

  ```rust
  pub struct ZoomState(Mutex<f64>);
  ```

- Lock, mutate, drop the guard, then perform side effects.

## Settings persistence

- Typed `Settings` struct with `serde::{Serialize, Deserialize}` and `#[serde(default)]` so new fields stay backward compatible.
- Persist pretty JSON to `app_data_dir()/settings.json`.
- Expose small helpers: `init` (load + manage) and `update_*` (mutate + save).

## Error handling and naming

- Return `tauri::Result<T>` and use `?` in fallible functions; avoid panics (`let Some(...) else { return Ok(()) }`).
- Menu item ids are constants in a `pub mod id { ... }`.
- Named constants for magic numbers (zoom clamp range, window size tiers).
