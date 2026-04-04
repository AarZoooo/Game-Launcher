# Agent Context — Game Launcher

> This is the canonical context file for AI agents working on this project.
> All tool-specific files (CLAUDE.md, AGENTS.md, .cursorrules) point here.

## Project Overview

Desktop game library manager built with Tauri v2 (Rust) + SvelteKit (TypeScript) + SQLite. Aggregates games from Steam, Epic, GOG, and local installs into a unified launcher with playtime tracking and metadata from IGDB.

## Tech Stack

- **Backend:** Tauri v2, Rust (2021 edition), rusqlite (bundled SQLite), sysinfo, reqwest, image
- **Frontend:** SvelteKit, Svelte 5, TypeScript, Vite
- **Metadata:** IGDB API via Twitch OAuth (`dotenvy` for env loading)
- **Linting:** Biome (formatting + linting), husky pre-commit hooks

## Architecture

### Backend (`src-tauri/src/`)

| Module | Purpose |
|--------|---------|
| `commands/` | Tauri command handlers — the API surface for the frontend |
| `db/` | SQLite schema, migrations, and query functions |
| `launch/` | Game process spawning |
| `tracking/` | Process monitoring, playtime session recording |
| `media/` | Cover art resolution (IGDB provider, local files, placeholders) |
| `perf/` | Dev-only resource monitor |
| `models/` | Shared Rust data structures |

Entry point: `main.rs` → `lib.rs` (registers all commands and sets up app state).

### Frontend (`src/`)

| Directory | Purpose |
|-----------|---------|
| `routes/` | SvelteKit pages (games, explore, settings) |
| `lib/components/` | UI components (game cards, panels, modals, side panels) |
| `lib/stores/` | Svelte stores — single source of truth for UI state |
| `lib/services/tauriService.ts` | Wrapper around `@tauri-apps/api` invoke calls |
| `lib/types/` | TypeScript interfaces and type definitions |
| `lib/utilities/` | Formatting, color extraction, helpers |

### Communication Pattern

- Frontend calls backend via `invoke()` (Tauri commands)
- Backend emits events to frontend via `app.emit()` for async updates (media resolution, process state)
- Stores subscribe to Tauri events in `+layout.svelte` on app init

## Conventions

### Rust

- All Tauri commands return `Result<T, String>`
- Database access through `db/` module functions, not inline SQL in commands
- Game identity is by `id` (string) or normalized `exe_path`
- State shared via `tauri::State<Mutex<T>>` for DB connection and tracker

### Frontend

- Svelte 5 runes (`$state`, `$derived`, `$effect`) — not legacy `$:` reactivity
- Stores are the single source of truth; components read from stores
- `tauriService.ts` is the only file that calls `invoke()`
- CSS is component-scoped (Svelte `<style>` blocks)

### Design System (`src/lib/styles/`)

- **Never hardcode values.** All sizes, colors, spacing, typography, motion, and radii must use centralized tokens.
- **Tokens:** `tokens/spacing.css` (space-0.5 to space-14), `tokens/typography.css` (display to caption-sm), `tokens/colors.css`, `tokens/radius.css`, `tokens/shadows.css`, `tokens/blur.css`
- **Motion:** `--motion-fast` (140ms), `--motion-base` (180ms), `--motion-slow` (300ms) — defined per theme
- **Utility classes:** `glass-surface` (blur + glass bg), `fade-to-bg` (bottom fade gradient) — reuse these instead of inlining
- **Control heights:** `--control-height-md` (2.5rem), `--control-height-sm` (2.2rem), `--control-height-xs` (1.15rem)
- **Sections are invisible containers** — no background, border, shadow, or padding. Visual hierarchy comes from headings and spacing, not card-styled wrappers.
- **Game cards use image fade mask** — the image fades to transparent at 50% via CSS mask, with text overlay at the bottom.

### Database

- SQLite file at Tauri app data dir
- Schema created on first run in `db/schema.rs`
- Tables: `games`, `play_sessions`

## Development Commands

```bash
npm run tauri dev       # Full app (Rust + frontend)
npm run dev             # Frontend only (Vite)
npm run build           # Build frontend
npm run tauri build     # Package desktop app
npm run check           # Svelte type checking
```

## Rules for AI Agents

1. **No shell scripts for OS interactions.** Never use PowerShell, bash, or cmd for things like file dialogs, system queries, or OS interactions. Use Rust-native solutions or cross-platform crates (e.g., `rfd` for file dialogs).

2. **Respect the module structure.** New Tauri commands go in `commands/`, DB queries in `db/`, etc. Don't put business logic in command handlers.

3. **Use Svelte 5 patterns.** Use `$state`, `$derived`, `$effect` — not Svelte 4's `$:` reactive statements or `writable()`/`readable()` stores.

4. **Frontend-backend bridge.** All Tauri `invoke()` calls go through `tauriService.ts`. Components should never call `invoke()` directly.

5. **Use proper abstractions.** The project scope is large with many phases ahead. Keep code modular, reusable, and expandable. Use traits for platform-specific implementations, separate concerns into modules, and design interfaces that can grow.

6. **Platform awareness.** Code is currently Windows-only. When adding OS-specific logic, use `cfg(target_os)` in Rust or runtime detection in TypeScript. Don't hardcode Windows paths in new code.

7. **Check DEV.md** for current progress, known issues, and what's been implemented.

8. **Keep documentation current.** When completing work, update:
   - `DEV.md` — check off completed items, add new tasks discovered during work
   - `AGENT_CONTEXT.md` — update architecture/conventions sections if patterns change
   - Code comments for non-obvious logic
