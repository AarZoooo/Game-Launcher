# Game Launcher

A desktop game library manager that unifies your games from Steam, Epic, GOG, and local installs into a single launcher with playtime tracking, metadata management, and game backlog tracking.

<!-- TODO: Add screenshot -->

## Features

- **Unified Library** — Scan and import games from Steam, Epic, GOG, or add manually
- **Game Launching** — Launch games directly with real-time process detection
- **Playtime Tracking** — Automatic session recording with per-game and daily playtime stats
- **Cover Art & Metadata** — Auto-fetch from IGDB (high-res 2x covers, 1080p artworks), local image fallback
- **Game Tracker** — Categorize games as Want to Play, Currently Playing, or Completed
- **Filter & Sort** — Browse by genre, status, playtime, rating, and more

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | [Tauri v2](https://v2.tauri.app/) (Rust) |
| Frontend | [SvelteKit](https://svelte.dev/) + Svelte 5 + TypeScript |
| Database | SQLite (via rusqlite) |
| Metadata API | [IGDB](https://www.igdb.com/) (Twitch OAuth) |
| Linting | [Biome](https://biomejs.dev/) + husky pre-commit hooks |

## Prerequisites

- Node.js 18+ and npm
- Rust toolchain ([rustup](https://rustup.rs/))
- Tauri v2 system dependencies ([platform-specific guide](https://v2.tauri.app/start/prerequisites/))

## Setup

```bash
# Install dependencies
npm install

# Start development (launches Tauri + Vite dev server)
npm run tauri dev
```

### IGDB API (optional)

To enable cover art and metadata fetching, create a `.env` file in `src-tauri/`:

```
TWITCH_CLIENT_ID=your_client_id
TWITCH_CLIENT_SECRET=your_client_secret
```

Get credentials from the [Twitch Developer Console](https://dev.twitch.tv/console).

### Dev Seed Data

The app ships with ~20 curated seed games (`src/lib/data/seedGames.ts`) for development testing. On startup, the app automatically resolves IGDB covers for seed games in parallel if Twitch credentials are configured.

## Project Structure

```
src/                        # SvelteKit frontend
├── routes/                 # Pages (home, games, explore, settings, game/[id])
├── lib/
│   ├── components/         # Svelte components
│   │   ├── common/         # Reusable: Button, Icon, Loader, Tooltip, EmptyState
│   │   ├── game/           # Game-specific: HeroBanner, GameCard, GameGrid, GameMenu, FilterPanel
│   │   ├── layout/         # AppShell, Sidebar
│   │   └── stats/          # StatsDashboard
│   ├── data/               # Seed data, labels, navigation config
│   ├── stores/             # Reactive state (libraryStore, uiStore, gameTrackerStore)
│   ├── services/           # Tauri command bridge (tauriService.ts)
│   ├── styles/             # Design system
│   │   ├── tokens/         # spacing, typography, colors, radius, shadows, blur, zIndex
│   │   ├── themes/         # dark, light, dynamic
│   │   └── components/     # buttons, cards, dropdown, modal
│   ├── types/              # TypeScript type definitions
│   └── utils/              # Helpers (accent, getGameMedia, playActivity)
src-tauri/                  # Rust backend
├── src/
│   ├── commands/           # Tauri command handlers (storage, igdb, tracking, perf, manual_add)
│   ├── db/                 # SQLite schema and queries (games, sessions)
│   ├── launch/             # Game process spawning
│   ├── tracking/           # Process monitoring, playtime session recording
│   ├── media/              # Cover art resolution (IGDB provider, local files, placeholders)
│   ├── perf/               # Resource monitoring (dev)
│   └── models/             # Shared Rust data structures
```

## Development Commands

| Command | Description |
|---------|-------------|
| `npm run tauri dev` | Launch app in development mode |
| `npm run tauri build` | Build packaged desktop app |
| `npm run dev` | Start Vite dev server only |
| `npm run build` | Build frontend for production |
| `npm run check` | Run Svelte type checking |

## Documentation

- `DEV.md` — Development tracker with phase progress, tech debt, and pending work
- `AGENT_CONTEXT.md` — AI agent context file with architecture, conventions, and rules

## License

MIT
