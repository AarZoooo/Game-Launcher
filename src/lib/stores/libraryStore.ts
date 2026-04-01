import { derived, get, writable } from "svelte/store";

import { appImages } from "$lib/assets";
import { pageLabels } from "$lib/data/labels";
import {
	getGames as loadStoredGames,
	type StoredGame,
	saveGames as saveStoredGames,
} from "$lib/services/tauriService";
import { uiStore } from "$lib/stores/uiStore";
import type {
	AccentTone,
	Game,
	GameMetric,
	GameStatus,
	ImportedGameResult,
	PlatformType,
	ResumeState,
} from "$lib/types/Game";
import { getAccentHexByTone } from "$lib/utils/accent";

export type {
	AccentTone,
	Game,
	GameMetric,
	GameStatus,
	ImportedGameResult,
	PlatformType,
	ResumeState,
} from "$lib/types/Game";

function slugify(value: string) {
	return value
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");
}

function parseHours(value: string) {
	const match = value.match(/\d+/);
	return match ? parseInt(match[0], 10) : 0;
}

function wait(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

const defaultGameCover = appImages.placeholders.gameCover;
const defaultGameBanner = appImages.banners.default;

function buildImportedGame(input: ImportedGameResult): Game {
	const platformLabel =
		input.platform === "steam"
			? pageLabels.platforms.steam
			: input.platform === "epic"
				? pageLabels.platforms.epic
				: pageLabels.platforms.local;
	const accent =
		input.platform === "steam"
			? "gold"
			: input.platform === "epic"
				? "silver"
				: "olive";
	const accentHex = getAccentHexByTone(accent);

	return {
		id: input.id || slugify(input.title),
		title: input.title,
		hours: "0h",
		platform: platformLabel,
		platformType: input.platform,
		genres: "Uncategorized",
		rating: "0.0",
		coop: "Unknown",
		completion: "Unknown",
		cover: defaultGameCover,
		accent,
		accentHex,
		accentColor: accentHex,
		path: input.path,
		inLibrary: true,
		status: "none",
		favorite: false,
		cloudSyncEnabled: false,
		tags: [],
		genreLabel: `${platformLabel} • New import`,
		resumeState: "resume",
		storageDescription: "",
		storageGenres: ["Uncategorized"],
		storageLastPlayedRaw: null,
	};
}

const seedGames: Game[] = [
	{
		id: "sekiro",
		title: "Sekiro: Shadows Die Twice",
		hours: "9h",
		platform: "Steam",
		genres: "Action • Adventure • RPG",
		rating: "9.1",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		hero: defaultGameBanner,
		accent: "gold",
		featured: true,
		inLibrary: true,
		path: "C:\\Games\\Sekiro\\sekiro.exe",
		lastPlayed: "1hr 26m",
		totalPlaytime: "102hr 59m",
		syncStatus: "Up to date",
		blurb: "A harsh, elegant action adventure with demanding swordplay.",
		genreLabel: "Action • 9.1",
		similarIds: ["ghost-tsushima", "nioh2", "witcher3", "hades", "stardew"],
		metrics: [
			{ label: "Last Play", value: "1hr 26m" },
			{ label: "Total Play", value: "102hr 59m" },
			{ label: "Genres", value: "Action • Adventure • RPG" },
			{ label: "Path", value: "C:\\Games\\Sekiro\\sekiro.exe" },
			{ label: "Cloud Sync", value: "Up to date" },
		],
	},
	{
		id: "cs2",
		title: "Counter Strike 2",
		hours: "1109h",
		platform: "Steam",
		genres: "Action",
		rating: "8.5",
		coop: "Yes",
		completion: "Multiplayer",
		cover: defaultGameCover,
		accent: "silver",
		inLibrary: true,
		genreLabel: "Action • 8.5",
	},
	{
		id: "satisfactory",
		title: "Satisfactory",
		hours: "215h",
		platform: "Local",
		genres: "Automation • Building",
		rating: "8.7",
		coop: "Yes",
		completion: "Sandbox",
		cover: defaultGameCover,
		accent: "silver",
		inLibrary: true,
		genreLabel: "Sandbox • 8.7",
	},
	{
		id: "hades",
		title: "Hades",
		hours: "12h",
		platform: "Steam",
		genres: "Action • Roguelite",
		rating: "9.2",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "gold",
		inLibrary: true,
		genreLabel: "Action • 9.2",
	},
	{
		id: "gta-vi",
		title: "Grand Theft Auto VI",
		hours: "1h",
		platform: "Steam",
		genres: "Open World • Crime",
		rating: "9.0",
		coop: "No",
		completion: "Story",
		cover: defaultGameCover,
		accent: "silver",
		inLibrary: true,
		genreLabel: "Open World • 9.0",
	},
	{
		id: "rust",
		title: "Rust",
		hours: "2h",
		platform: "Steam",
		genres: "Survival • Shooter",
		rating: "7.8",
		coop: "Yes",
		completion: "Persistent",
		cover: defaultGameCover,
		accent: "silver",
		inLibrary: true,
		genreLabel: "Survival • 7.8",
	},
	{
		id: "stardew",
		title: "Stardew Valley",
		hours: "256h",
		platform: "Local",
		genres: "RPG • Survival • Simulation",
		rating: "8.2",
		coop: "Yes",
		completion: "Single player",
		cover: defaultGameCover,
		hero: defaultGameBanner,
		accent: "gold",
		inLibrary: true,
		genreLabel: "RPG • 8.2",
		path: "C:\\Windows\\System32\\notepad.exe",
		similarIds: [
			"coral-island",
			"sun-haven",
			"sandrock",
			"roots-of-pacha",
			"terraria",
		],
	},
	{
		id: "ghost-yotei",
		title: "Ghost of Yotei",
		hours: "0h",
		platform: "Wishlist",
		genres: "Action • Adventure • RPG",
		rating: "9.1",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		hero: defaultGameBanner,
		accent: "olive",
		inRecommendations: true,
		genreLabel: "Adventure • 9.1",
		similarIds: ["ghost-tsushima", "sekiro", "nioh2", "witcher3", "horizon-fw"],
	},
	{
		id: "elden-ring",
		title: "Elden Ring",
		hours: "0h",
		platform: "Suggested",
		genres: "Action • Fantasy",
		rating: "9.0",
		coop: "Yes",
		completion: "Open world",
		cover: defaultGameCover,
		accent: "gold",
		inRecommendations: true,
		genreLabel: "Action • 9.0",
	},
	{
		id: "rdr2",
		title: "Red Dead Redemption 2",
		hours: "0h",
		platform: "Suggested",
		genres: "RPG • Western",
		rating: "8.8",
		coop: "Yes",
		completion: "Open world",
		cover: defaultGameCover,
		accent: "gold",
		inRecommendations: true,
		genreLabel: "RPG • 8.8",
	},
	{
		id: "hollow-knight",
		title: "Hollow Knight",
		hours: "0h",
		platform: "Suggested",
		genres: "Action • Metroidvania",
		rating: "9.2",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Action • 9.2",
	},
	{
		id: "cyberpunk-2077",
		title: "Cyberpunk 2077",
		hours: "0h",
		platform: "Suggested",
		genres: "Action • RPG • Open World",
		rating: "7.9",
		coop: "Yes",
		completion: "Single player",
		cover: defaultGameCover,
		hero: defaultGameBanner,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Open World • 7.9",
		similarIds: [
			"deus-ex",
			"the-ascent",
			"ghostrunner-2",
			"observer",
			"cloudpunk",
		],
	},
	{
		id: "uncharted-4",
		title: "Uncharted 4",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "9.1",
		coop: "No",
		completion: "Story",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Adventure • 9.1",
	},
	{
		id: "valorant",
		title: "Valorant",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.0",
		coop: "Yes",
		completion: "Competitive",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Action • 8.0",
	},
	{
		id: "battlefield-2042",
		title: "Battlefield 2042",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.2",
		coop: "Yes",
		completion: "Competitive",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Action • 8.2",
	},
	{
		id: "ori",
		title: "Ori and the Blind Forest",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "9.2",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Adventure • 9.2",
	},
	{
		id: "forza-horizon-6",
		title: "Forza Horizon 6",
		hours: "0h",
		platform: "Suggested",
		genres: "Open World",
		rating: "7.9",
		coop: "Yes",
		completion: "Racing",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Open World • 7.9",
	},
	{
		id: "modern-warfare",
		title: "Call of Duty: Modern Warfare",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.9",
		coop: "Yes",
		completion: "Shooter",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Action • 8.9",
	},
	{
		id: "it-takes-two",
		title: "It Takes Two",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "8.2",
		coop: "Yes",
		completion: "Co-op",
		cover: defaultGameCover,
		accent: "gold",
		inRecommendations: true,
		genreLabel: "Adventure • 8.2",
	},
	{
		id: "sea-of-thieves",
		title: "Sea of Thieves",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "7.8",
		coop: "Yes",
		completion: "Multiplayer",
		cover: defaultGameCover,
		accent: "gold",
		inRecommendations: true,
		genreLabel: "Adventure • 7.8",
	},
	{
		id: "a-way-out",
		title: "A Way Out",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.8",
		coop: "Yes",
		completion: "Co-op story",
		cover: defaultGameCover,
		accent: "gold",
		inRecommendations: true,
		genreLabel: "Action • 8.8",
	},
	{
		id: "no-mans-sky",
		title: "No Man's Sky",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "7.5",
		coop: "Yes",
		completion: "Exploration",
		cover: defaultGameCover,
		accent: "silver",
		inRecommendations: true,
		genreLabel: "Adventure • 7.5",
	},
	{
		id: "ghost-tsushima",
		title: "Ghost of Tsushima: Director’s Cut",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "9.2",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "olive",
		genreLabel: "Adventure • 9.2",
	},
	{
		id: "nioh2",
		title: "Nioh 2",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.7",
		coop: "Yes",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "olive",
		genreLabel: "Action • 8.7",
	},
	{
		id: "witcher3",
		title: "The Witcher 3: Wild Hunt",
		hours: "0h",
		platform: "Suggested",
		genres: "RPG",
		rating: "8.9",
		coop: "No",
		completion: "Story",
		cover: defaultGameCover,
		accent: "silver",
		genreLabel: "RPG • 8.9",
	},
	{
		id: "horizon-fw",
		title: "Horizon Forbidden West",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "8.8",
		coop: "No",
		completion: "Story",
		cover: defaultGameCover,
		accent: "gold",
		genreLabel: "Adventure • 8.8",
	},
	{
		id: "coral-island",
		title: "Coral Island",
		hours: "0h",
		platform: "Suggested",
		genres: "Simulation",
		rating: "8.5",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "gold",
		genreLabel: "Simulation • 8.5",
	},
	{
		id: "sun-haven",
		title: "Sun Haven",
		hours: "0h",
		platform: "Suggested",
		genres: "RPG",
		rating: "8.0",
		coop: "Yes",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "gold",
		genreLabel: "RPG • 8.0",
	},
	{
		id: "sandrock",
		title: "My Time at Sandrock",
		hours: "0h",
		platform: "Suggested",
		genres: "Simulation",
		rating: "8.0",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "gold",
		genreLabel: "Simulation • 8.0",
	},
	{
		id: "roots-of-pacha",
		title: "Roots of Pacha",
		hours: "0h",
		platform: "Suggested",
		genres: "Simulation",
		rating: "8.5",
		coop: "Yes",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "gold",
		genreLabel: "Simulation • 8.5",
	},
	{
		id: "terraria",
		title: "Terraria",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "9.0",
		coop: "Yes",
		completion: "Sandbox",
		cover: defaultGameCover,
		accent: "gold",
		genreLabel: "Action • 9.0",
	},
	{
		id: "deus-ex",
		title: "Deus Ex: Mankind Divided",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.5",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		genreLabel: "Action • 8.5",
	},
	{
		id: "the-ascent",
		title: "The Ascent",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.0",
		coop: "Yes",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		genreLabel: "Action • 8.0",
	},
	{
		id: "ghostrunner-2",
		title: "Ghostrunner 2",
		hours: "0h",
		platform: "Suggested",
		genres: "Action",
		rating: "8.0",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		genreLabel: "Action • 8.0",
	},
	{
		id: "observer",
		title: "Observer: System Redux",
		hours: "0h",
		platform: "Suggested",
		genres: "Horror",
		rating: "7.8",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		genreLabel: "Horror • 7.8",
	},
	{
		id: "cloudpunk",
		title: "Cloudpunk",
		hours: "0h",
		platform: "Suggested",
		genres: "Adventure",
		rating: "7.5",
		coop: "No",
		completion: "Single player",
		cover: defaultGameCover,
		accent: "silver",
		genreLabel: "Adventure • 7.5",
	},
];

function inferPlatformType(platform: string): PlatformType {
	const normalized = platform.toLowerCase();

	if (normalized.includes("steam")) return "steam";
	if (normalized.includes("epic")) return "epic";
	if (normalized.includes("wishlist")) return "wishlist";
	if (normalized.includes("suggested")) return "suggested";
	return "local";
}

function normalizeGame(game: Game, index: number): Game {
	const defaultStatus: GameStatus =
		parseHours(game.hours) > 50
			? "played"
			: parseHours(game.hours) > 0
				? "playing"
				: "want";

	return {
		...game,
		platformType: game.platformType || inferPlatformType(game.platform),
		accentHex: game.accentHex || getAccentHexByTone(game.accent),
		accentColor:
			game.accentColor || game.accentHex || getAccentHexByTone(game.accent),
		status: game.status || defaultStatus,
		favorite: game.favorite ?? index < 4,
		cloudSyncEnabled: game.cloudSyncEnabled ?? Boolean(game.syncStatus),
		tags: game.tags || [],
		resumeState:
			game.resumeState || (parseHours(game.hours) > 0 ? "resume" : "restart"),
		storageDescription: game.storageDescription ?? game.blurb ?? "",
		storageGenres: game.storageGenres ?? splitGenres(game.genres),
		storageLastPlayedRaw: game.storageLastPlayedRaw ?? null,
	};
}

export const allGames = seedGames.map(normalizeGame);
const fallbackGames = allGames.map((game) => ({
	...game,
	inLibrary: false,
}));

export const featuredGameId = "sekiro";
export const homeRecentIds = [
	"sekiro",
	"cs2",
	"satisfactory",
	"hades",
	"gta-vi",
];
export const homeExploreIds = [
	"elden-ring",
	"rdr2",
	"hollow-knight",
	"cyberpunk-2077",
	"uncharted-4",
];
export const libraryOrderIds = [
	"sekiro",
	"cs2",
	"satisfactory",
	"hades",
	"gta-vi",
	"rust",
	"stardew",
];
export const explorePrimaryIds = [
	"elden-ring",
	"rdr2",
	"hollow-knight",
	"cyberpunk-2077",
	"uncharted-4",
	"valorant",
	"battlefield-2042",
	"ori",
	"forza-horizon-6",
	"modern-warfare",
];
export const exploreSecondaryIds = [
	"it-takes-two",
	"sea-of-thieves",
	"a-way-out",
	"elden-ring",
	"no-mans-sky",
];

function splitGenres(value: string) {
	return value
		.split(/[^a-zA-Z0-9+]+/)
		.map((item) => item.trim())
		.filter(Boolean);
}

function formatPlatformLabel(platform: string) {
	return platform.charAt(0).toUpperCase() + platform.slice(1);
}

function formatPlaytime(minutes?: number) {
	if (!minutes) return "0m";

	const hours = Math.floor(minutes / 60);
	const remainingMinutes = minutes % 60;

	if (!hours) return `${remainingMinutes}m`;
	if (!remainingMinutes) return `${hours}h`;
	return `${hours}h ${remainingMinutes}m`;
}

function formatHours(minutes?: number) {
	if (!minutes) return "0h";
	return `${Math.max(1, Math.round(minutes / 60))}h`;
}

function formatLastPlayed(value: string | null) {
	if (!value) return undefined;

	const parsed = new Date(value);

	if (Number.isNaN(parsed.getTime())) {
		return value;
	}

	return parsed.toLocaleDateString("en-US", {
		month: "short",
		day: "numeric",
		year: "numeric",
	});
}

function mapStoredStatus(status: string): GameStatus {
	if (status === "playing") return "playing";
	if (status === "completed") return "played";
	if (status === "backlog") return "want";
	return "none";
}

function mapUiStatus(status: GameStatus | undefined): string {
	if (status === "playing") return "playing";
	if (status === "played") return "completed";
	if (status === "want") return "backlog";
	return "installed";
}

function buildAccent(platformType: PlatformType): AccentTone {
	if (platformType === "steam") return "gold";
	if (platformType === "epic") return "silver";
	return "olive";
}

function toMetrics(game: Game): GameMetric[] {
	return [
		{ label: "Last Play", value: game.lastPlayed || "Never" },
		{ label: "Total Play", value: game.totalPlaytime || game.hours },
		{ label: "Genres", value: game.genres },
		{ label: "Path", value: game.path || "Unavailable" },
		{ label: "Storage", value: pageLabels.messages.localLibrary },
	];
}

function mapStoredGame(game: StoredGame): Game {
	const platformType = inferPlatformType(game.platform);
	const accent = buildAccent(platformType);
	const genres = game.genres.length ? game.genres.join(" / ") : "Uncategorized";

	return normalizeGame(
		{
			id: game.id,
			title: game.title,
			hours: formatHours(game.totalPlaytime),
			platform: formatPlatformLabel(game.platform),
			platformType,
			genres,
			rating: "0.0",
			coop: "Unknown",
			completion: "Unknown",
			cover: game.coverArt || defaultGameCover,
			hero: game.coverArt || defaultGameBanner,
			accent,
			path: game.exePath,
			lastPlayed: formatLastPlayed(game.lastPlayed),
			totalPlaytime: formatPlaytime(game.totalPlaytime),
			syncStatus: pageLabels.messages.storedLocally,
			blurb: game.description,
			genreLabel: genres,
			inLibrary: true,
			featured: false,
			status: mapStoredStatus(game.status),
			favorite: false,
			cloudSyncEnabled: false,
			tags: [],
			metrics: [],
			storageDescription: game.description,
			storageGenres: game.genres,
			storageLastPlayedRaw: game.lastPlayed,
		},
		0,
	);
}

function toStoredGame(game: Game): StoredGame {
	return {
		id: game.id,
		title: game.title,
		exePath: game.path || "",
		coverArt: game.cover,
		platform:
			game.platformType &&
			game.platformType !== "suggested" &&
			game.platformType !== "wishlist"
				? game.platformType
				: game.platform.toLowerCase(),
		totalPlaytime: parseHours(game.hours) * 60,
		lastPlayed: game.storageLastPlayedRaw ?? null,
		status: mapUiStatus(game.status),
		genres: game.storageGenres?.length
			? game.storageGenres
			: splitGenres(game.genres),
		description: game.storageDescription ?? game.blurb ?? "",
	};
}

function mergeStoredLibrary(storedGames: StoredGame[]) {
	const libraryGames = storedGames.map(mapStoredGame).map((game) => ({
		...game,
		metrics: toMetrics(game),
	}));
	const libraryIds = new Set(libraryGames.map((game) => game.id.toLowerCase()));

	return [
		...libraryGames,
		...fallbackGames.filter((game) => !libraryIds.has(game.id.toLowerCase())),
	];
}

function createGameStore() {
	const { subscribe, update, set } = writable<Game[]>(fallbackGames);

	async function persistLibrary(items: Game[]) {
		const storedGames = items
			.filter((game) => game.inLibrary !== false)
			.map(toStoredGame);

		await saveStoredGames(storedGames);
	}

	// TODO: Favorites, cloud sync, launch options, tags, save paths, ratings, co-op,
	// and recommendation metadata still live only in the frontend until backend support exists.
	function withMetrics(items: Game[]) {
		return items.map((game) => ({
			...game,
			metrics: game.metrics?.length ? game.metrics : toMetrics(game),
		}));
	}

	async function updateAndPersist(
		transform: (items: Game[]) => Game[],
		message = "Updating library...",
	) {
		let nextItems = get(games);

		update((items) => {
			nextItems = withMetrics(transform(items));
			return nextItems;
		});

		uiStore.setLibraryBusy(true, message);
		const startedAt = Date.now();

		try {
			await persistLibrary(nextItems);
			const elapsed = Date.now() - startedAt;
			if (elapsed < 450) {
				await wait(450 - elapsed);
			}
		} catch (error) {
			console.error("Failed to persist library changes:", error);
			throw error;
		} finally {
			uiStore.setLibraryBusy(false);
		}
	}

	return {
		subscribe,
		reset: () => set(fallbackGames),
		async loadFromBackend() {
			const storedGames = await loadStoredGames();
			set(mergeStoredLibrary(storedGames));
		},
		toggleFavorite: (id: string) =>
			update((items) =>
				items.map((game) =>
					game.id === id ? { ...game, favorite: !game.favorite } : game,
				),
			),
		setStatus: (id: string, status: GameStatus) =>
			updateAndPersist(
				(items) =>
					items.map((game) =>
						game.id === id
							? { ...game, status, hiddenFromContinue: false }
							: game,
					),
				"Saving game status...",
			),
		toggleCloudSync: (id: string) =>
			update((items) =>
				items.map((game) =>
					game.id === id
						? {
								...game,
								cloudSyncEnabled: !game.cloudSyncEnabled,
								syncStatus: !game.cloudSyncEnabled
									? "Sync enabled"
									: "Sync paused",
							}
						: game,
				),
			),
		setLaunchOptions: (id: string, launchOptions: string) =>
			update((items) =>
				items.map((game) =>
					game.id === id ? { ...game, launchOptions } : game,
				),
			),
		updateDetails: (
			id: string,
			payload: Partial<
				Pick<Game, "title" | "path" | "tags" | "cover" | "savePath">
			>,
		) =>
			updateAndPersist(
				(items) =>
					items.map((game) =>
						game.id === id
							? {
									...game,
									...payload,
									storageDescription:
										game.storageDescription ?? game.blurb ?? "",
									storageGenres: game.storageGenres ?? splitGenres(game.genres),
								}
							: game,
					),
				"Saving game details...",
			),
		removeFromLibrary: (id: string) =>
			updateAndPersist(
				(items) => items.filter((game) => game.id !== id),
				"Removing game...",
			),
		hideFromContinuePlaying: (id: string) =>
			update((items) =>
				items.map((game) =>
					game.id === id ? { ...game, hiddenFromContinue: true } : game,
				),
			),
		addImportedGames: (imports: ImportedGameResult[]) =>
			updateAndPersist((items) => {
				const seen = new Set(
					items.flatMap((game) => [
						game.id.toLowerCase(),
						(game.path || "").toLowerCase(),
					]),
				);

				const additions = imports
					.filter((item) => {
						const byId = item.id.toLowerCase();
						const byPath = item.path.toLowerCase();
						return !seen.has(byId) && !seen.has(byPath);
					})
					.map(buildImportedGame);

				return [...items, ...additions];
			}, "Adding games..."),
		addManualGame: (title: string, path: string) =>
			updateAndPersist((items) => {
				const candidate: ImportedGameResult = {
					id: `local-${slugify(title)}`,
					title,
					path,
					platform: "local",
				};

				const exists = items.some(
					(game) =>
						game.id.toLowerCase() === candidate.id.toLowerCase() ||
						(game.path &&
							game.path.toLowerCase() === candidate.path.toLowerCase()),
				);

				return exists ? items : [...items, buildImportedGame(candidate)];
			}, "Adding game..."),
	};
}

export const games = createGameStore();
export const installedGames = derived(games, ($games) =>
	$games.filter((game) => game.inLibrary === true),
);
export const catalogGames = derived(games, ($games) =>
	$games.filter((game) => game.inLibrary !== true),
);
export const favoriteGames = derived(games, ($games) =>
	$games.filter((game) => game.favorite),
);
export const continuePlayingGames = derived(games, ($games) =>
	$games
		.filter((game) => !game.hiddenFromContinue && parseHours(game.hours) > 0)
		.slice(0, 5),
);
export const playingGames = derived(games, ($games) =>
	$games.filter((game) => game.status === "playing"),
);

export function getAllGames() {
	return get(games);
}

export function getInstalledGames() {
	return get(games).filter((game) => game.inLibrary === true);
}

export function getCatalogGames() {
	return get(games).filter((game) => game.inLibrary !== true);
}

export function getGameById(id: string) {
	return get(games).find((game) => game.id === id);
}

export function getGamesByIds(ids: string[]) {
	const items = get(games);
	return ids
		.map((id) => items.find((game) => game.id === id))
		.filter((game): game is Game => Boolean(game));
}

export function hasDuplicateGame(candidate: ImportedGameResult) {
	const items = get(games);
	return items.some(
		(game) =>
			game.id.toLowerCase() === candidate.id.toLowerCase() ||
			(game.path && game.path.toLowerCase() === candidate.path.toLowerCase()),
	);
}
