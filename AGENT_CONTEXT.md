# Agent Context — Game Launcher

> This is the canonical context file for AI agents working on this project.
> All tool-specific files (CLAUDE.md, AGENTS.md, .cursorrules) point here.

## Project Overview

Desktop game library manager built with Tauri v2 (Rust) + SvelteKit (TypeScript) + SQLite. Aggregates games from Steam, Epic, GOG, and local installs into a unified launcher with playtime tracking and metadata from IGDB.

**Current state:** Phase 1 (Core Launcher) is ~95% complete, Phase 2 (Game Tracker) ~75%. Phases 3-4 (Cloud Saves, AI) not started. A major UI polish pass has been done. See `DEV.md` for full progress.

## Tech Stack

- **Backend:** Tauri v2, Rust (2021 edition), rusqlite (bundled SQLite), sysinfo, reqwest, image
- **Frontend:** SvelteKit, Svelte 5, TypeScript, Vite
- **Metadata:** IGDB API via Twitch OAuth (`dotenvy` for env loading). Covers at `t_cover_big_2x` (528px), artworks/screenshots at `t_1080p` (1920px). Artworks preferred over screenshots for horizontal images.
- **Linting:** Biome (formatting + linting), husky pre-commit hooks

## Architecture

### Backend (`src-tauri/src/`)

| Module | Purpose |
|--------|---------|
| `commands/` | Tauri command handlers — the API surface for the frontend |
| `db/` | SQLite schema, migrations, and query functions |
| `launch/` | Game process spawning |
| `tracking/` | Process monitoring, playtime session recording |
| `media/` | Cover art resolution (IGDB provider, scoped local install-file fallback, placeholders) |
| `perf/` | Dev-only resource monitor |
| `models/` | Shared Rust data structures |

Entry point: `main.rs` → `lib.rs` (registers all commands and sets up app state).

### Frontend (`src/`)

| Directory | Purpose |
|-----------|---------|
| `routes/` | SvelteKit pages (home, games, explore, settings, game/[id]) |
| `lib/components/common/` | Reusable: Button, Icon, Loader, Tooltip, EmptyState |
| `lib/components/game/` | HeroBanner (shared hero), GameCard, GameGrid, GameMenu, GamePlayButton, FilterPanel, ContinuePlaying, GameDetails |
| `lib/components/layout/` | AppShell (grid shell + sidebar), Sidebar |
| `lib/components/stats/` | StatsDashboard (playtime stats, heatmap, genre chart) |
| `lib/data/` | Seed games, UI labels, navigation config |
| `lib/stores/` | Svelte stores — single source of truth for UI state |
| `lib/services/tauriService.ts` | Wrapper around `@tauri-apps/api` invoke calls |
| `lib/styles/` | Design system (tokens, themes, component styles, utilities) |
| `lib/types/` | TypeScript interfaces and type definitions |
| `lib/utils/` | Helpers (accent color, media resolution, formatting) |

### Key Components

- **HeroBanner** (`components/game/HeroBanner.svelte`) — Shared horizontal cover component used by both home page (ContinuePlaying) and game detail page (GameDetails). Props control: back button, favourite button, eyebrow text, play button visibility. Uses CSS `mask-image` for bottom fade.
- **GameCard** (`components/game/GameCard.svelte`) — Card with portrait image (CSS mask fade at 50%), text overlay at bottom, circular favourite + menu buttons. Hover: slow scale zoom.
- **Sidebar** (`components/layout/Sidebar.svelte`) — Sticky sidebar with brand, left-aligned nav, profile section with sync button + tooltip. No accent tint.
- **Tooltip** (`components/common/Tooltip.svelte`) — Reusable tooltip with glass-surface style, 4 positions (top/bottom/left/right).

### Communication Pattern

- Frontend calls backend via `invoke()` (Tauri commands)
- Backend emits events to frontend via `app.emit()` for async updates (media resolution, process state)
- Stores subscribe to Tauri events in `+layout.svelte` on app init
- On startup: `loadFromBackend()` → `resolveIgdbCovers()` (parallel, fills seed games with IGDB covers)

### Dev Seed Data

- `src/lib/data/seedGames.ts` — ~20 curated real games with varied statuses, playtimes, favourites
- No hardcoded image URLs — IGDB resolver fetches covers at runtime via `resolveIgdbCovers()`
- Seed games are fallback data when no backend DB games exist

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
- **Tokens:** `tokens/spacing.css` (space-0.5 through space-14), `tokens/typography.css` (display through caption-sm + icon-xs), `tokens/colors.css`, `tokens/radius.css`, `tokens/shadows.css`, `tokens/blur.css`
- **Motion:** `--motion-fast` (140ms), `--motion-base` (180ms), `--motion-slow` (300ms) — defined per theme (dark, light, dynamic)
- **Utility classes:** `glass-surface` (blur + glass bg + border), `fade-to-bg` (bottom fade gradient) — reuse instead of inlining
- **Control heights:** `--control-height-md` (2.5rem), `--control-height-sm` (2.2rem), `--control-height-xs` (1.15rem)
- **Sections are invisible containers** — no background, border, shadow, or padding. Visual hierarchy comes from headings and spacing, not card-styled wrappers.
- **Game cards use CSS mask-image** — image fades to transparent at 50% height. No color overlay needed.
- **Hero banners use CSS mask-image** — same approach as cards, works regardless of background.
- **Images from IGDB:** Covers use `t_cover_big_2x`, artworks/screenshots use `t_1080p`. Artworks preferred over screenshots for horizontal images.

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

6. **Platform awareness.** Code is currently Windows-centric. When adding OS-specific logic, use `cfg(target_os)` in Rust or runtime detection in TypeScript. Don't hardcode Windows paths in new code.

7. **Design system first.** Never hardcode font sizes, spacing, colors, or timing values. Always use or create tokens. Check `src/lib/styles/tokens/` before adding new values.

8. **Check DEV.md** for current progress, known issues, and what's been implemented.

9. **Keep documentation current.** When completing work, update:
   - `DEV.md` — check off completed items, add new tasks discovered during work
   - `AGENT_CONTEXT.md` — update architecture/conventions sections if patterns change
   - Code comments for non-obvious logic

10. **Commit granularly.** One logical change per commit, one-liner messages, no co-authored-by sections.

Additional note: Home page empty/loading states now use a shared `libraryHydrated` frontend store signal so skeletons only show before backend hydration, and loaded-empty sections render reusable `EmptyState` variants instead of collapsing.
Additional note: The shared frontend `Modal` component now handles focus trapping, ESC dismissal, backdrop dismissal, and focus restoration, so new dialogs should reuse it instead of reimplementing modal behavior.
