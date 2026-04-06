import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";
import { openPath } from "@tauri-apps/plugin-opener";
import { uiStore } from "$lib/stores/uiStore";
import type { GameSession, ImportedGameResult } from "$lib/types/Game";

export interface StoredGame {
	id: string;
	title: string;
	exePath: string;
	coverVertical?: string;
	coverHorizontal?: string;
	banner?: string;
	icon?: string;
	accentColor?: string;
	coverArt?: string;
	platform: string;
	totalPlaytime: number;
	lastPlayed: string | null;
	status: string;
	genres: string[];
	description: string;
	rating: string;
	coop: string;
	completion: string;
	sessions?: GameSession[];
}

export interface TodayPlaytimeEntry {
	gameId: string;
	minutesPlayedToday: number;
}

export interface IgdbSearchResult {
	id: number;
	name: string;
	slug?: string;
	genres: string[];
	coverUrl?: string;
	artworkUrls: string[];
	screenshotUrls: string[];
	summary?: string;
}

export interface LocalScanResult extends ImportedGameResult {}

export interface ManualGameCandidate {
	id: string;
	title: string;
	path: string;
	platform: "local";
}

export interface GameProcessEvent {
	gameId: string | null;
	exePath: string;
	state: "started" | "exited" | "error";
	message?: string;
}

export interface GameMediaResolutionEvent {
	gameId: string;
	exePath: string;
	state: "started" | "finished";
}

export async function getGames(): Promise<StoredGame[]> {
	return invoke<StoredGame[]>("read_games");
}

export async function refreshInstalledGameMedia(): Promise<StoredGame[]> {
	return invoke<StoredGame[]>("refresh_installed_game_media");
}

export async function saveGames(games: StoredGame[]): Promise<string> {
	return invoke<string>("write_games", { games });
}

export async function getTodayPlaytime(): Promise<TodayPlaytimeEntry[]> {
	return invoke<TodayPlaytimeEntry[]>("get_today_playtime");
}

export async function searchIgdbGame(
	title: string,
): Promise<IgdbSearchResult[]> {
	return invoke<IgdbSearchResult[]>("search_igdb_game", { title });
}

export async function scanLocalGames(): Promise<LocalScanResult[]> {
	return invoke<LocalScanResult[]>("scan_local_games");
}

export async function pickGameExecutable(): Promise<ManualGameCandidate | null> {
	return invoke<ManualGameCandidate | null>("pick_game_executable");
}

export async function launchGame(exePath: string, gameId?: string) {
	if (!exePath) {
		throw new Error("No executable path was provided for this game.");
	}

	const optimisticStart = Boolean(gameId);

	try {
		if (optimisticStart && gameId) {
			uiStore.startGame(gameId);
		} else {
			uiStore.setGameRunning(true);
		}

		await invoke("launch_game", { exePath, gameId: gameId || null });
	} catch (err) {
		const error = err instanceof Error ? err : new Error(String(err));

		if (!optimisticStart) {
			uiStore.setGameRunning(false);
			console.error("Failed to launch game:", error);
			throw error;
		}

		// Some backend flows can launch successfully and then still reject the invoke
		// while process tracking settles. Keep the optimistic running state and let
		// subsequent process events reconcile the UI back to idle.
		console.warn(
			"Launch invoke returned an error after starting the game; preserving running state until process events update it.",
			error,
		);
	}
}

export async function listenForGameProcessEvents(
	callback: (event: GameProcessEvent) => void,
) {
	return listen<GameProcessEvent>("game-process-state", (event) => {
		callback(event.payload);
	});
}

export async function listenForGameMediaUpdates(
	callback: (game: StoredGame) => void,
) {
	return listen<StoredGame>("game-media-updated", (event) => {
		callback(event.payload);
	});
}

export async function listenForGameMediaResolutionEvents(
	callback: (event: GameMediaResolutionEvent) => void,
) {
	return listen<GameMediaResolutionEvent>("game-media-resolution", (event) => {
		callback(event.payload);
	});
}

export function setGameRunning(isRunning: boolean, gameId?: string | null) {
	uiStore.setGameRunning(isRunning, gameId ?? null);
}

function getFolderPath(path: string) {
	const normalized = path.replace(/\//g, "\\");
	const index = normalized.lastIndexOf("\\");
	return index > 0 ? normalized.slice(0, index) : normalized;
}

export async function openGameFolder(path?: string) {
	if (!path) {
		throw new Error("No game path is available.");
	}

	await openPath(getFolderPath(path));
}

export async function openSaveFolder(path?: string) {
	if (!path) {
		throw new Error("No save path is configured for this game.");
	}

	await openPath(path);
}
