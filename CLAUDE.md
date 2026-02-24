# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

Snapr is a Windows window-snapping utility built with Tauri (Rust backend + React frontend). It installs a global low-level keyboard hook to intercept hotkeys and snap windows to screen positions (halves, quarters, maximize, center). Users configure shortcuts through a GUI that minimizes to the system tray.

## Repository Structure

Cargo workspace with two members:

- **`libs/snapr/`** — Core Rust library: keyboard hook (`events/`), window positioning via Win32 APIs (`monitor/`), command definitions and storage (`commands/`), JSON config persistence (`configuration/`). Windows-only (`windows-sys` crate).
- **`tauri-app/src-tauri/`** — Tauri backend: initializes snapr, manages `AppState` with `Arc<CommandStorage>`, exposes `save_config`/`load_config` commands to the frontend, sets up system tray.
- **`tauri-app/src/`** — React frontend: shortcut configuration UI using TanStack Router, Radix UI, TailwindCSS v4.

## Build & Development Commands

Package manager is **Bun**. All frontend commands run from `tauri-app/`.

```bash
# Development (runs both Vite dev server and Tauri)
cd tauri-app && bun run tauri dev

# Production build
cd tauri-app && bun run tauri build

# Rust-only build (workspace root)
cargo build

# Run snapr library standalone (no GUI)
cargo run -p snapr

# Frontend only (Vite dev server on port 1420)
cd tauri-app && bun run dev

# Lint/format frontend (Biome)
cd tauri-app && bunx biome check .
cd tauri-app && bunx biome check --write .
```

No test suite exists currently.

## Architecture Details

**Keyboard hook flow:** `events::start_keyboard_listener` spawns a thread that installs a Win32 `WH_KEYBOARD_LL` hook. The hook callback uses thread-local storage to access `CommandStorage`, matches pressed key+modifiers against registered commands, and calls `Command::exec()` which calculates target bounds and calls `Monitor::set_position()`.

**Modifier bitmask encoding:** Ctrl=1, Shift=2, Win=4, Alt=8. Stored in `KeyBinding.modifiers` as a `u32`.

**Command storage key format:** Commands are serialized/indexed as `"key;modifiers"` strings (e.g., `"37;4"` for Win+Left).

**Config persistence:** JSON file in the OS app data directory (`UserConfiguration` struct). Loaded at startup; if missing, defaults are written. Frontend changes invoke Tauri commands that update both the in-memory `CommandStorage` (via `RwLock`) and the JSON file.

**Window positioning:** Accounts for invisible shadow borders (7px constant `SHADOW_BORDERS_SIZE`). Detects fullscreen windows and skips snapping. Handles maximized windows by calling `ShowWindow(SW_SHOWNORMAL)` before repositioning.

## Code Conventions

- **Rust:** Edition 2024 (snapr lib), Edition 2021 (tauri-app). Platform-specific code gated with `#[cfg(windows)]`.
- **TypeScript/React:** Strict mode, Biome for formatting (tabs, double quotes), path alias `@/*` → `./src/*`.
- **Concurrency:** `Arc<CommandStorage>` shared between Tauri main thread and keyboard hook thread. `RwLock<HashMap>` for interior mutability.
