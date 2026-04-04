# Game Launcher

A desktop game library manager that unifies your games from Steam, Epic, GOG, and local installs into a single launcher with playtime tracking, metadata management, and game backlog tracking.

<!-- TODO: Add screenshot -->

## Features

- **Unified Library** — Scan and import games from Steam, Epic, GOG, or add manually
- **Game Launching** — Launch games directly with real-time process detection
- **Playtime Tracking** — Automatic session recording with per-game and daily playtime stats
- **Cover Art & Metadata** — Auto-fetch from IGDB, or use local images as fallback
- **Game Tracker** — Categorize games as Want to Play, Currently Playing, or Completed
- **Filter & Sort** — Browse by genre, status, playtime, rating, and more

## Tech Stack

| Layer | Technology |
|-------|------------|
| Desktop shell | [Tauri v2](https://v2.tauri.app/) (Rust) |
| Frontend | [SvelteKit](https://svelte.dev/) + TypeScript |
| Database | SQLite (via rusqlite) |
| Metadata API | [IGDB](https://www.igdb.com/) (Twitch OAuth) |

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

## Project Structure

```
src/                        # SvelteKit frontend
├── routes/                 # Pages (games, explore, settings)
├── lib/
│   ├── components/         # Svelte components (game cards, panels, modals)
│   ├── stores/             # Reactive state (library, tracker, settings)
│   ├── services/           # Tauri command bridge
│   ├── types/              # TypeScript type definitions
│   └── utilities/          # Helpers (formatting, colors, etc.)
src-tauri/                  # Rust backend
├── src/
│   ├── commands/           # Tauri command handlers
│   ├── db/                 # SQLite schema and queries
│   ├── launch/             # Game process launching
│   ├── tracking/           # Process tracking & playtime
│   ├── media/              # Cover art resolution (IGDB, local)
│   ├── perf/               # Resource monitoring (dev)
│   └── models/             # Shared data models
```

## Development Commands

| Command | Description |
|---------|-------------|
| `npm run tauri dev` | Launch app in development mode |
| `npm run tauri build` | Build packaged desktop app |
| `npm run dev` | Start Vite dev server only |
| `npm run build` | Build frontend for production |
| `npm run check` | Run Svelte type checking |

## License

MIT
