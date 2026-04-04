# Development Tracker

## Phase 1 — Core Launcher

- [x] **M1.1 Project Scaffold** — Tauri v2 + SvelteKit initialized, dev workflow working
- [x] **M1.2 Game Library UI** — Card grid with cover art, platform badges, playtime display
- [x] **M1.3 Game Entry CRUD** — Add (scan/manual/platform), edit title, delete from library, SQLite persistence
- [x] **M1.4 Game Launching** — Launch exe via Tauri, process detection with launcher/updater awareness
- [x] **M1.5 Playtime Tracking** — Session recording (start/end/duration), total + daily playtime, last played date
- [x] **M1.6 Game Auto-Detection** — Directory scanning with depth limits, exe scoring, duplicate detection, platform identification
- [x] **M1.7 Cover Art** — IGDB fetch with Twitch OAuth (2x covers, 1080p artworks), local file fallback, placeholder generation
  - [ ] Custom cover art upload from UI (file picker not implemented)

## Phase 2 — Game Tracker

- [x] **M2.1 Tracker Data Model** — Status field (`want`/`playing`/`played`/`none`), favorite flag, persisted in DB
- [x] **M2.2 Tracker UI** — No dedicated tracker page needed; the games page with filters serves this purpose
- [ ] **M2.3 Automatic Status Updates** — No auto-move on launch, no inactivity detection
- [x] **M2.4 Filter & Sort** — Sort by title/playtime/rating, filter by genre/status/favorites. Filters persist within app session only (reset on app close, by design).

## Phase 3 — Cloud Saves & Auth

- [ ] **M3.1 User Auth** — OAuth2 (Google, Steam, Epic)
- [ ] **M3.2 Storage Provider Linking** — Google Drive, Dropbox, OneDrive
- [ ] **M3.3 Save File Management** — Per-game save paths, manual sync
- [ ] **M3.4 Auto Sync & Conflict Resolution**
- [ ] **M3.5 Platform Library Import** — Steam/Epic account-based import

## Phase 4 — AI Layer

- [ ] **M4.1 OpenAI Integration**
- [ ] **M4.2 Taste Profile**
- [ ] **M4.3 AI Tracker Updater**
- [ ] **M4.4 Recommendation Engine**
- [ ] **M4.5 Recommendation Refresh**

---

## Extra Work (not in original roadmap)

- [x] **IGDB API Integration** — Full provider with Twitch OAuth, title matching, caching, multi-image support. Covers at `t_cover_big_2x`, artworks/screenshots at `t_1080p`. Artworks preferred over screenshots.
- [x] **Process Tracking Sophistication** — Short session detection, reappearance windows, launcher activity differentiation
- [x] **Resource Monitor** — Dev-only CPU/memory tracking for launcher process group
- [x] **Biome Linter** — Code formatting/linting with husky pre-commit hooks
- [x] **UI Polish Pass** — Play button restyle (no icon, larger padding), HeroBanner shared component, game card redesign (CSS mask image fade, bottom text overlay, circular fav/menu buttons, slow zoom hover), sidebar cleanup (sync moved to profile, no accent tint, left-aligned nav), stats dashboard reorganization (total hours hero, subtle heatmap, merged streaks)
- [x] **Design System Centralization** — Typography tokens (display through icon-xs), spacing half-steps (0.5, 1.5), motion-slow (300ms), control-height-xs, glass-surface utility, fade-to-bg utility. All hardcoded font-sizes replaced across codebase.
- [x] **Reusable Components** — Tooltip (glass-surface, 4 positions), HeroBanner (shared between home + game page, CSS mask fade)
- [x] **Dev Seed Data** — 20 curated real games in `src/lib/data/seedGames.ts` with varied statuses/playtimes. IGDB covers resolved in parallel on startup via `resolveIgdbCovers()`.
- [x] **IGDB Image Quality** — Upgraded to 2x covers (528px) and 1080p artworks/screenshots. Frontend prefers artworks over screenshots for horizontal images.

---

## Known Issues / Tech Debt

- [ ] **PowerShell for file dialogs** — `manual_add.rs` shells out to PowerShell for file picker; must use Rust-native solution (e.g., `rfd` crate)
- [ ] **No tests** — Zero test files (frontend or backend), no test runner configured
- [ ] **Windows-only** — Hardcoded Windows paths, `.exe` assumptions, no OS detection
- [ ] **No cross-platform process tracking** — `sysinfo` is cross-platform but path logic is Windows-specific
- [ ] **CSP disabled** — `tauri.conf.json` has `"csp": null`
- [ ] **Blocking HTTP in async context** — `reqwest::blocking` used in some Tauri commands instead of async reqwest. Backend `read_games` triggers `queue_media_resolution` which makes blocking HTTP calls and can slow app startup.
- [ ] **Error handling** — `Result<T, String>` everywhere; should use typed errors
- [ ] **CI/CD** — Consider GitHub Actions (free for public repos, 2000 min/month free for private). Pre-commit hooks already cover Biome linting; could add `cargo check` and `svelte-check` to hooks
- [ ] **Toast/notification system** — Build toast component + store, rework sync button to async with toast feedback, remove blocking full-screen overlay for sync
- [ ] **Game card menu portal** — Render dropdown at body level so it's never clipped by card/grid bounds
- [ ] **Local media cache layer** — Download IGDB images to local disk cache, extract dominant colors from landscape covers (for accent theming), serve cached assets to frontend. Needed for: offline mode, faster startup, bandwidth savings, accurate accent colors. Currently images load from CDN every time and accent colors fall back to hardcoded tones (gold/silver) instead of actual cover colors.
- [ ] **Accent color extraction** — Depends on media cache layer. Currently `accent` field in seed data is hardcoded (gold/silver). Should extract dominant color from landscape cover image and use for sidebar active state, hero tinting, etc.

---

## In Progress

_Nothing currently in progress._
