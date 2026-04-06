import { derived, get, writable } from "svelte/store";

import { pageLabels } from "$lib/data/labels";
import { seedGames as rawSeedGames } from "$lib/data/seedGames";
import {
	getGames as loadStoredGames,
	getTodayPlaytime as loadTodayPlaytime,
	refreshInstalledGameMedia as refreshStoredInstalledGameMedia,
	type StoredGame,
	saveGames as saveStoredGames,
	searchIgdbGame,
	type TodayPlaytimeEntry,
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

function parsePlaytimeToMinutes(value?: string) {
	if (!value) return 0;

	const hourMatch = value.match(/(\d+)\s*h/i);
	const minuteMatch = value.match(/(\d+)\s*m/i);

	if (hourMatch || minuteMatch) {
		const hours = hourMatch ? parseInt(hourMatch[1], 10) : 0;
		const minutes = minuteMatch ? parseInt(minuteMatch[1], 10) : 0;
		return hours * 60 + minutes;
	}

	return parseHours(value) * 60;
}

function wait(ms: number) {
	return new Promise((resolve) => setTimeout(resolve, ms));
}

const seedGames = rawSeedGames as Game[];

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
		mediaLoading: false,
		hours: "0h",
		platform: platformLabel,
		platformType: input.platform,
		genres: "Uncategorized",
		rating: "0.0",
		coop: "Unknown",
		completion: "Unknown",
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
		storageTotalPlaytimeMinutes: 0,
		storageMinutesPlayedToday: 0,
		storageLastPlayedRaw: null,
	};
}

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
		mediaLoading: game.mediaLoading ?? false,
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
		storageTotalPlaytimeMinutes:
			game.storageTotalPlaytimeMinutes ??
			parsePlaytimeToMinutes(game.totalPlaytime || game.hours),
		storageMinutesPlayedToday: game.storageMinutesPlayedToday ?? 0,
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
	"counter-strike-2",
	"elden-ring",
	"satisfactory",
	"hades",
];
export const homeExploreIds = [
	"ghost-of-tsushima",
	"it-takes-two",
	"ori-and-the-blind-forest",
	"death-stranding",
	"hades-ii",
];
export const libraryOrderIds = [
	"sekiro",
	"counter-strike-2",
	"elden-ring",
	"satisfactory",
	"hades",
	"red-dead-redemption-2",
	"stardew-valley",
	"the-witcher-3",
	"god-of-war",
	"hollow-knight",
	"cyberpunk-2077",
	"rust",
	"celeste",
];
export const explorePrimaryIds = [
	"ghost-of-tsushima",
	"it-takes-two",
	"ori-and-the-blind-forest",
	"death-stranding",
	"hades-ii",
];
export const exploreSecondaryIds = ["baldurs-gate-3", "disco-elysium"];

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

	if (minutes < 1) {
		return "0m";
	}

	if (minutes < 60) {
		return `${minutes}m`;
	}

	const hours = Math.floor(minutes / 60);
	return `${hours}h`;
}

function formatLastPlayed(value: string | null) {
	if (!value) return "Never";

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
		{
			label: "Played Today",
			value: formatPlaytime(game.storageMinutesPlayedToday),
		},
		{ label: "Total Play", value: game.totalPlaytime || game.hours },
		{ label: "Genres", value: game.genres },
		{ label: "Path", value: game.path || "Unavailable" },
		{ label: "Storage", value: pageLabels.messages.localLibrary },
	];
}

function mapStoredGame(game: StoredGame, existing?: Game): Game {
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
			rating: existing?.rating || "0.0",
			coop: existing?.coop || "Unknown",
			completion: existing?.completion || "Unknown",
			coverVertical: game.coverVertical || game.coverArt,
			coverHorizontal: game.coverHorizontal,
			banner: game.banner,
			icon: game.icon,
			accent,
			accentColor: game.accentColor,
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
			storageTotalPlaytimeMinutes: game.totalPlaytime,
			storageMinutesPlayedToday: 0,
			storageLastPlayedRaw: game.lastPlayed,
		},
		0,
	);
}

function matchesStoredGameIdentity(current: Game, next: StoredGame) {
	return (
		current.id.toLowerCase() === next.id.toLowerCase() ||
		((current.path || "").toLowerCase() !== "" &&
			(current.path || "").toLowerCase() === next.exePath.toLowerCase())
	);
}

function mergeStoredGameUpdate(
	existing: Game | undefined,
	storedGame: StoredGame,
) {
	const mapped = mapStoredGame(storedGame, existing);

	if (!existing) {
		return {
			...mapped,
			metrics: toMetrics(mapped),
		};
	}

	const merged: Game = {
		...existing,
		...mapped,
		mediaLoading: false,
		favorite: existing.favorite,
		cloudSyncEnabled: existing.cloudSyncEnabled,
		launchOptions: existing.launchOptions,
		tags: existing.tags,
		hiddenFromContinue: existing.hiddenFromContinue,
		featured: existing.featured,
		inRecommendations: existing.inRecommendations,
		similarIds: existing.similarIds,
		savePath: existing.savePath,
	};

	return {
		...merged,
		metrics: toMetrics(merged),
	};
}

function toStoredGame(game: Game): StoredGame {
	return {
		id: game.id,
		title: game.title,
		exePath: game.path || "",
		coverVertical: game.coverVertical,
		coverHorizontal: game.coverHorizontal,
		banner: game.banner,
		icon: game.icon,
		accentColor: game.accentColor,
		coverArt: game.coverVertical || game.cover,
		platform:
			game.platformType &&
			game.platformType !== "suggested" &&
			game.platformType !== "wishlist"
				? game.platformType
				: game.platform.toLowerCase(),
		totalPlaytime:
			game.storageTotalPlaytimeMinutes ??
			parsePlaytimeToMinutes(game.totalPlaytime || game.hours),
		lastPlayed: game.storageLastPlayedRaw ?? null,
		status: mapUiStatus(game.status),
		genres: game.storageGenres?.length
			? game.storageGenres
			: splitGenres(game.genres),
		description: game.storageDescription ?? game.blurb ?? "",
	};
}

function mergeTodayPlaytime(
	items: Game[],
	todayPlaytimeEntries: TodayPlaytimeEntry[],
) {
	const todayMinutesByGameId = new Map(
		todayPlaytimeEntries.map((entry) => [
			entry.gameId.toLowerCase(),
			entry.minutesPlayedToday,
		]),
	);

	return items.map((game) => ({
		...game,
		storageMinutesPlayedToday:
			todayMinutesByGameId.get(game.id.toLowerCase()) || 0,
	}));
}

function mergeStoredLibrary(
	storedGames: StoredGame[],
	todayPlaytimeEntries: TodayPlaytimeEntry[] = [],
	existingItems: Game[] = fallbackGames,
) {
	const libraryGames = mergeTodayPlaytime(
		storedGames.map((storedGame) => {
			const existing = existingItems.find((game) =>
				matchesStoredGameIdentity(game, storedGame),
			);
			return mapStoredGame(storedGame, existing);
		}),
		todayPlaytimeEntries,
	).map((game) => ({
		...game,
		metrics: toMetrics(game),
	}));
	const libraryIds = new Set(libraryGames.map((game) => game.id.toLowerCase()));

	return [
		...libraryGames,
		...fallbackGames.filter((game) => !libraryIds.has(game.id.toLowerCase())),
	];
}

function buildTodayPlaytimeMap(todayPlaytimeEntries: TodayPlaytimeEntry[]) {
	return new Map(
		todayPlaytimeEntries.map((entry) => [
			entry.gameId.toLowerCase(),
			entry.minutesPlayedToday,
		]),
	);
}

function applyStoredGameStats(
	items: Game[],
	storedGame: StoredGame,
	todayMinutes: number,
) {
	return items.map((game) => {
		if (!matchesStoredGameIdentity(game, storedGame)) {
			return game;
		}

		const merged = mergeStoredGameUpdate(game, storedGame);
		const withStats: Game = {
			...merged,
			hours: formatHours(storedGame.totalPlaytime),
			totalPlaytime: formatPlaytime(storedGame.totalPlaytime),
			lastPlayed: formatLastPlayed(storedGame.lastPlayed),
			storageTotalPlaytimeMinutes: storedGame.totalPlaytime,
			storageMinutesPlayedToday: todayMinutes,
			storageLastPlayedRaw: storedGame.lastPlayed,
		};

		return {
			...withStats,
			metrics: toMetrics(withStats),
		};
	});
}

function createGameStore() {
	const { subscribe, update, set } = writable<Game[]>(fallbackGames);

	async function readBackendSnapshot() {
		const currentItems = get(games);
		const [storedGames, todayPlaytimeEntries] = await Promise.all([
			loadStoredGames(),
			loadTodayPlaytime().catch((error) => {
				console.error("Failed to load today's playtime:", error);
				return [];
			}),
		]);

		return mergeStoredLibrary(
			storedGames,
			todayPlaytimeEntries,
			currentItems.length ? currentItems : fallbackGames,
		);
	}

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
			set(await readBackendSnapshot());
		},
		async resolveIgdbCovers() {
			const items = get(games);
			const needsCovers = items.filter(
				(game) => !game.coverVertical && !game.coverHorizontal && game.title,
			);

			if (!needsCovers.length) return;

			await Promise.allSettled(
				needsCovers.map(async (game) => {
					try {
						const results = await searchIgdbGame(game.title);
						if (!results.length) return;

						const best = results[0];
						const horizontalImage =
							best.artworkUrls?.[0] ||
							best.screenshotUrls?.[0] ||
							best.coverUrl;
						update((current) =>
							current.map((item) =>
								item.id === game.id
									? {
											...item,
											coverVertical: best.coverUrl || item.coverVertical,
											coverHorizontal: horizontalImage || item.coverHorizontal,
											banner: horizontalImage || item.banner,
											storageDescription:
												item.storageDescription || best.summary || "",
											storageGenres: best.genres.length
												? best.genres
												: item.storageGenres,
										}
									: item,
							),
						);
					} catch (error) {
						console.warn(
							`[igdb] Failed to resolve cover for "${game.title}":`,
							error,
						);
					}
				}),
			);
		},
		async refreshInstalledGameMedia() {
			uiStore.setLibraryBusy(true, "Refreshing installed game media...");
			const startedAt = Date.now();

			try {
				const [storedGames, todayPlaytimeEntries] = await Promise.all([
					refreshStoredInstalledGameMedia(),
					loadTodayPlaytime().catch((error) => {
						console.error("Failed to load today's playtime:", error);
						return [];
					}),
				]);

				set(
					mergeStoredLibrary(
						storedGames,
						todayPlaytimeEntries,
						get(games).length ? get(games) : fallbackGames,
					),
				);

				const elapsed = Date.now() - startedAt;
				if (elapsed < 450) {
					await wait(450 - elapsed);
				}
			} finally {
				uiStore.setLibraryBusy(false);
			}
		},
		async refreshAfterGameExit(gameId?: string | null) {
			if (!gameId) {
				set(await readBackendSnapshot());
				return;
			}

			const currentItems = get(games);
			const currentGame = currentItems.find((game) => game.id === gameId);
			const previousTotalMinutes =
				currentGame?.storageTotalPlaytimeMinutes ?? 0;
			const previousLastPlayed = currentGame?.storageLastPlayedRaw ?? null;
			const previousPlayedToday = currentGame?.storageMinutesPlayedToday ?? 0;
			const deadline = Date.now() + 35000;
			let attempt = 0;

			while (Date.now() < deadline) {
				const [storedGames, todayPlaytimeEntries] = await Promise.all([
					loadStoredGames(),
					loadTodayPlaytime().catch((error) => {
						console.error("Failed to load today's playtime:", error);
						return [];
					}),
				]);
				const matchingStoredGame = storedGames.find(
					(storedGame) => storedGame.id.toLowerCase() === gameId.toLowerCase(),
				);

				if (!matchingStoredGame) {
					set(
						mergeStoredLibrary(
							storedGames,
							todayPlaytimeEntries,
							get(games).length ? get(games) : fallbackGames,
						),
					);
					return;
				}

				const todayMinutes =
					buildTodayPlaytimeMap(todayPlaytimeEntries).get(
						gameId.toLowerCase(),
					) || 0;
				const targetGameChanged =
					matchingStoredGame.totalPlaytime !== previousTotalMinutes ||
					(matchingStoredGame.lastPlayed ?? null) !== previousLastPlayed ||
					todayMinutes !== previousPlayedToday;

				if (targetGameChanged) {
					update((items) =>
						applyStoredGameStats(items, matchingStoredGame, todayMinutes),
					);
					return;
				}

				attempt += 1;
				await wait(attempt < 8 ? 300 : 1000);
			}

			set(await readBackendSnapshot());
		},
		applyBackendGameUpdate(storedGame: StoredGame) {
			update((items) => {
				let didMatch = false;
				const nextItems = items.map((game) => {
					if (!matchesStoredGameIdentity(game, storedGame)) {
						return game;
					}

					didMatch = true;
					return mergeStoredGameUpdate(game, storedGame);
				});

				if (didMatch) {
					return nextItems;
				}

				const appendedGame = mergeStoredGameUpdate(undefined, storedGame);
				const dedupedItems = nextItems.filter(
					(game) => !matchesStoredGameIdentity(game, storedGame),
				);

				return [...dedupedItems, appendedGame];
			});
		},
		setMediaLoading(
			identity: { gameId?: string | null; exePath?: string | null },
			loading: boolean,
		) {
			const normalizedGameId = identity.gameId?.toLowerCase() || "";
			const normalizedExePath = identity.exePath?.toLowerCase() || "";

			update((items) =>
				items.map((game) => {
					const matchesId =
						normalizedGameId !== "" &&
						game.id.toLowerCase() === normalizedGameId;
					const matchesPath =
						normalizedExePath !== "" &&
						(game.path || "").toLowerCase() === normalizedExePath;

					if (!matchesId && !matchesPath) {
						return game;
					}

					return {
						...game,
						mediaLoading: loading,
					};
				}),
			);
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
			payload: Partial<Pick<Game, "title" | "path" | "tags" | "savePath">>,
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
export const continuePlayingGames = derived(games, ($games) => {
	const installedVisibleGames = $games.filter(
		(game) => game.inLibrary === true && !game.hiddenFromContinue,
	);

	const sortByRecentActivity = (left: Game, right: Game) => {
		const leftLastPlayed = left.storageLastPlayedRaw
			? Date.parse(left.storageLastPlayedRaw)
			: 0;
		const rightLastPlayed = right.storageLastPlayedRaw
			? Date.parse(right.storageLastPlayedRaw)
			: 0;
		const safeLeftLastPlayed = Number.isNaN(leftLastPlayed)
			? 0
			: leftLastPlayed;
		const safeRightLastPlayed = Number.isNaN(rightLastPlayed)
			? 0
			: rightLastPlayed;

		if (safeRightLastPlayed !== safeLeftLastPlayed) {
			return safeRightLastPlayed - safeLeftLastPlayed;
		}

		return (
			(right.storageTotalPlaytimeMinutes ?? 0) -
			(left.storageTotalPlaytimeMinutes ?? 0)
		);
	};

	const recentGames = installedVisibleGames.filter(
		(game) => (game.storageTotalPlaytimeMinutes ?? 0) > 0,
	);

	return recentGames.sort(sortByRecentActivity).slice(0, 5);
});
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
