import { pageLabels } from "$lib/data/labels";
import {
	getSyncNowMessage,
	getSyncShortcutMessage,
} from "$lib/services/syncService";
import {
	launchGame,
	openGameFolder,
	openSaveFolder,
} from "$lib/services/tauriService";
import { games } from "$lib/stores/libraryStore";
import type { Game, GameStatus } from "$lib/types/Game";
import type { GameMenuActionId } from "$lib/types/Menu";

function statusFromAction(actionId: GameMenuActionId): GameStatus | null {
	if (actionId === "status-want") return "want";
	if (actionId === "status-playing") return "playing";
	if (actionId === "status-played") return "played";
	return null;
}

function promptForValue(label: string, initial = "") {
	return window.prompt(label, initial);
}

export async function playGame(game: Game) {
	if (!game.path) {
		throw new Error("No executable path was provided for this game.");
	}

	return launchGame(game.path, game.id);
}

export async function performGameAction(
	actionId: GameMenuActionId,
	game: Game,
) {
	const nextStatus = statusFromAction(actionId);

	if (nextStatus) {
		return games.setStatus(game.id, nextStatus);
	}

	if (actionId === "play" || actionId === "resume" || actionId === "restart") {
		return playGame(game);
	}

	if (actionId === "toggle-favorite") {
		games.toggleFavorite(game.id);
		return;
	}

	if (actionId === "open-folder") {
		return openGameFolder(game.path);
	}

	if (actionId === "open-save-folder") {
		return openSaveFolder(game.savePath);
	}

	if (actionId === "toggle-cloud-sync") {
		games.toggleCloudSync(game.id);
		return;
	}

	if (actionId === "remove-library") {
		return games.removeFromLibrary(game.id);
	}

	if (actionId === "hide-continue") {
		games.hideFromContinuePlaying(game.id);
		return;
	}

	if (actionId === "edit-details") {
		const title = promptForValue(pageLabels.fields.gameTitlePrompt, game.title);
		if (title) {
			return games.updateDetails(game.id, { title });
		}
		return;
	}

	if (actionId === "change-cover") {
		const cover = promptForValue(pageLabels.fields.coverPrompt, game.cover);
		if (cover) {
			return games.updateDetails(game.id, { cover });
		}
		return;
	}

	if (actionId === "launch-options") {
		const value = promptForValue(
			pageLabels.fields.launchArgumentsPrompt,
			game.launchOptions || "",
		);
		if (value !== null) {
			games.setLaunchOptions(game.id, value);
		}
		return;
	}

	if (actionId === "create-shortcut") {
		window.alert(getSyncShortcutMessage(game.title));
		return;
	}

	if (actionId === "sync-now") {
		window.alert(getSyncNowMessage(game.title));
		return;
	}

	if (actionId === "view-playtime") {
		window.alert(
			pageLabels.messages.playtimeDetails(
				game.title,
				game.totalPlaytime || game.hours,
			),
		);
	}
}
