# Game Launcher (Tauri + Svelte)

## Overview

This project is a high-level plan and implementation scaffold for a Windows game launcher.
The app is meant to serve as a unified hub for a user’s entire game library, regardless of source, with:

- local game management and launching
- metadata editing and custom cover art
- playtime tracking and running-state detection
- Steam and Epic game library import
- cloud save sync through user-owned storage providers
- OAuth sign-in for Google, Steam, and Epic
- AI-powered tracker updates and game recommendations

The core launcher and tracker features are designed to stay free. A paid services tier is planned later for hosted backend features.

## Goals

- Provide a clean, unified interface for launching and managing games from any source
- Give users full control over their library entries, including cover art and metadata
- Support cloud save sync via user-owned storage accounts at no cost to the user
- Integrate with Steam and Epic Games for library import and optional social sign-in
- Help users track their game backlog with an intelligent, automatically updated tracker
- Offer AI-driven game recommendations when users are unsure what to play next
- Keep the application lightweight with low memory and CPU footprint suitable for background use while gaming

## Software guide scope

This repository currently holds the client-side application and front-end scaffold.
It includes:

- Tauri desktop shell configuration and host settings
- Svelte frontend source, routes, and UI starter
- Local launch/build workflow for the desktop app
- Basic Tauri + SvelteKit setup

Planned system scope includes separate backend responsibilities as well:

- local app: game scanning, process detection, executable launching, save file path resolution, playtime tracking
- backend: auth, AI inference, cloud save coordination, platform integration, and user data storage

**Note:** the backend service described in the roadmap is not included in this repository yet.

## Tech stack

| Layer | Technology | Notes |
|---|---|---|
| Client shell | Tauri (Rust) | Native Windows app host, handles system calls and filesystem access. |
| Client UI | Svelte | Frontend framework inside Tauri webview. |
| Backend | FastAPI (Python) | REST API for auth, AI, cloud save coordination, platform integration. |
| AI layer | OpenAI API | Called exclusively from the backend. |
| Auth | TBD (authlib or Auth0) | OAuth2 flows for Google, Steam, Epic. |
| Cloud saves | Google Drive, Dropbox, OneDrive | Uses the user’s own storage account. |

## Feature breakdown

### Core Launcher

All core launcher features are free and available to all users.

- Launcher cards for games in a browsable grid/list view
- Full CRUD support for game entries
- Add games manually or via auto-detection
- Edit metadata: title, executable path, description, tags
- Remove entries without uninstalling the game
- Custom cover art per entry (upload locally or fetch from the web)
- Support games from any source, including DRM-free or manually added titles
- Launch games directly from launcher cards
- Detect running game processes and update status in real time
- Track last played date and cumulative playtime per game

### Cloud Saves

Cloud save syncing is designed to use the user’s own storage account. The backend coordinates sync but does not store user save data.

- Supported providers: Google Drive, Dropbox, OneDrive
- Automatic or manual save sync per game entry
- Works for games from any source
- Save data stays in the user’s own storage account
- Conflict resolution prompt when local and cloud versions differ
- Users authenticate with chosen storage provider via OAuth

### Platform Integration

- Connect Steam and Epic Games accounts
- Import installed games from connected Steam/Epic accounts
- Optionally launch platform-linked games through native clients
- Use Google, Steam, or Epic accounts as sign-in providers
- Visually distinguish platform-linked entries from manual entries

### Game Tracker

Embedded backlog tracking system with automatic updates from launcher usage.

- Three core tracker lists: Played, Currently Playing, Want to Play
- Lists update based on launcher activity
- Starting a game moves it to Currently Playing
- Extended inactivity triggers a status review prompt
- Manual overrides are always available
- Per-game metadata: user rating, notes, date started, date completed
- Filter and sort by status, genre, playtime, rating, and more

### AI Layer

The AI assistant runs on the backend and supports tracker updating plus recommendations.

#### Tracker updater

- Analyses playtime and session data to infer status changes
- Suggests moving games between tracker lists based on recent activity
- Flags long-abandoned games and prompts the user to update status
- Suggestions are non-destructive; user approval is required

#### Recommendation engine

- Builds a taste profile from library, playtime, genres, ratings
- Suggests the best games to play next from the Want to Play list
- Can surface new titles outside the library based on user profile
- Recommendations refresh periodically or on demand
- Users can dismiss or pin suggestions to influence future results

## Monetisation

The app is free to download and use. A paid services tier is planned later for hosted backend features.

| Feature | Free | Paid (Services Tier) |
|---|---|---|
| Game library management | Yes | Yes |
| Custom cover art | Yes | Yes |
| Support for pirated game entries | Yes | Yes |
| Game Tracker | Yes | Yes |
| AI tracker updater | Yes | Yes |
| AI game recommendations | Yes | Yes |
| Cloud save sync (user-owned storage) | Yes | Yes |
| Steam and Epic library import | Yes | Yes |
| Platform-hosted services | No | Yes (scope TBD) |
| Pricing | Free | TBD |

## Development roadmap

The roadmap is split into four phases.

### Phase 1 — Core Launcher

#### M1.1 Project scaffold

- Tauri + Svelte project initialized
- FastAPI backend project initialized
- Basic local dev environment running
- Client and backend can communicate via a test API call
- App window opens and fetches a response from the backend

#### M1.2 Game library UI

- Launcher card component built in Svelte
- Grid and list view layouts
- Static mock data rendered in the UI
- Basic navigation between views

#### M1.3 Game entry CRUD

- Add game form (title, executable path, tags)
- Edit and delete game entries
- Persistent local storage for entries
- UI reflects changes immediately

#### M1.4 Game launching

- Launch executable from launcher card via Tauri
- Detect running game process
- Show in-progress state on the card while the game is running

#### M1.5 Playtime tracking

- Record session start and end times on launch and close
- Accumulate total playtime per game
- Display last played date and total playtime on each card

#### M1.6 Game auto-detection

- Scan common install directories for known game executables
- Present detected games for user confirmation before adding
- Avoid duplicate entries for already-added games

#### M1.7 Cover art

- Upload custom cover art from local files
- Fetch cover art from a public source by game title
- Fallback placeholder for entries with no art

### Phase 2 — Game Tracker

#### M2.1 Tracker data model

- Define tracker status enum: Played, Currently Playing, Want to Play
- Extend game entry schema with tracker status, rating, notes, dates
- Migrate existing entries to new schema

#### M2.2 Tracker UI

- Dedicated tracker view with three list sections
- Move games between lists manually via drag or context menu
- Display rating, notes, and dates per entry

#### M2.3 Automatic status updates

- Auto-move a game to Currently Playing on first launch
- Detect inactivity threshold and surface a status review prompt
- User can confirm or dismiss suggested status changes

#### M2.4 Filter and sort

- Filter tracker lists by status, genre, rating
- Sort by playtime, last played, title, date added
- Persist filter and sort preferences across sessions

### Phase 3 — Cloud Saves and Accounts

#### M3.1 User auth

- OAuth2 sign-in via Google, Steam, Epic on the backend
- Auth token issued and stored securely on the client
- User profile screen showing connected accounts
- Sign-out flow

#### M3.2 Storage provider linking

- OAuth connection to Google Drive, Dropbox, OneDrive
- User can link one or more providers
- Backend verifies provider access and stores tokens securely

#### M3.3 Save file management

- User defines save file path per game entry
- Backend reads and writes to linked storage provider
- Manual sync trigger per game

#### M3.4 Automatic sync and conflict resolution

- Auto-sync save on game close
- Detect version conflicts between local and cloud saves
- Prompt user to choose which version to keep

#### M3.5 Platform library import

- Pull installed game list from connected Steam account
- Pull installed game list from connected Epic Games account
- Present imported entries for user confirmation before adding
- Distinguish platform-linked entries visually in the library

### Phase 4 — AI Layer

#### M4.1 OpenAI integration

- OpenAI API client set up on the backend
- Prompt templating system for structured requests
- Error handling and basic rate-limit management

#### M4.2 Taste profile

- Aggregate playtime, genres, ratings, and tracker data per user
- Build a structured taste profile from this data
- Store and refresh profile on schedule or on demand

#### M4.3 AI tracker updater

- AI analyses recent session data and suggests tracker status changes
- Suggestions are non-destructive prompts in the UI
- User can accept, dismiss, or ignore each suggestion

#### M4.4 Recommendation engine

- AI ranks Want to Play list by fit with taste profile
- AI can suggest new titles outside the library
- Recommendations shown in a dedicated UI section
- User can pin or dismiss recommendations

#### M4.5 Recommendation refresh

- Scheduled background refresh of recommendations
- On-demand refresh triggered from the UI
- Staleness indicator when recommendations have not been refreshed recently

## Open questions

- Application name and branding
- Auth implementation: Auth0 (managed) vs authlib (self-managed)
- Paid services tier scope and pricing
- OpenAI API usage strategy, quota, cost controls, and throttling for free users
- Offline behavior and which features remain functional without internet
- Steam/Epic integration depth: read-only library import vs full launch delegation

## Prerequisites

- Windows 10/11
- Node.js 18+ and npm
- Rust toolchain (`rustup`) for Tauri native builds
- Optional: Python for the backend

Recommended Rust install commands:

```powershell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup toolchain install stable
rustup target add x86_64-pc-windows-msvc
```

If the Tauri CLI is not installed locally yet, run:

```bash
npm install --save-dev @tauri-apps/cli
```

## Setup guide

1. Install project dependencies:

```bash
npm install
```

2. Confirm the frontend builds successfully:

```bash
npm run build
```

3. Start the Tauri development app:

```bash
npm run tauri dev
```

This launches the Svelte app inside the Tauri desktop shell.

## Development workflow

- `npm run dev`: start Vite development server
- `npm run tauri dev`: launch Tauri with the frontend dev server
- `npm run build`: build the frontend for production
- `npm run tauri build`: build the packaged desktop app
- `npm run check`: run Svelte type checking

## Notes for extending the app

- Add new UI pages under `src/routes/`
- Use Tauri APIs from `@tauri-apps/api` for native functionality
- Configure native behavior in `src-tauri/tauri.conf.json`
- Persist local data using file-based storage or a small local database

## Recommended IDE

- VS Code
- Svelte extension
- Tauri extension
- rust-analyzer

## Notes

This `README.md` now contains the complete high-level plan and roadmap from `docs/game-launcher-plan-v3.docx`.
Once you confirm this content is complete, the `docs/` file may be removed.

