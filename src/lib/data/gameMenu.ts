import { pageLabels } from "$lib/data/labels";
import type {
	GameMenuContext,
	MenuAction,
	MenuActionFactory,
	MenuActionState,
} from "$lib/types/Menu";

const labels = pageLabels.actions;

const playAction: MenuActionFactory = ({ isActiveGame }) => ({
	id: "play",
	label: isActiveGame ? labels.playing : labels.play,
	disabled: isActiveGame,
});

const toggleFavoriteAction: MenuActionFactory = ({ game }) => ({
	id: "toggle-favorite",
	label: game.favorite ? labels.removeFavorite : labels.addFavorite,
});

const resumeAction: MenuActionFactory = ({ game, isActiveGame }) => ({
	id: game.resumeState === "restart" ? "restart" : "resume",
	label: game.resumeState === "restart" ? labels.restart : labels.resume,
	disabled: isActiveGame,
});

const cloudSyncAction: MenuActionFactory = ({ game }) => ({
	id: "toggle-cloud-sync",
	label: game.cloudSyncEnabled
		? labels.disableCloudSync
		: labels.enableCloudSync,
});

const menuConfig: Record<GameMenuContext, MenuActionFactory[][]> = {
	explore: [
		[
			() => ({ id: "status-want", label: labels.wantToPlay }),
			() => ({ id: "status-playing", label: labels.playing }),
			() => ({ id: "status-played", label: labels.played }),
		],
	],
	home: [
		[playAction, toggleFavoriteAction, resumeAction],
		[() => ({ id: "open-folder", label: labels.openFolder })],
		[() => ({ id: "view-playtime", label: labels.viewPlaytime })],
		[
			() => ({
				id: "hide-continue",
				label: labels.removeFromContinue,
				tone: "danger",
			}),
		],
	],
	library: [
		[playAction, toggleFavoriteAction],
		[
			() => ({ id: "open-folder", label: labels.openGameFolder }),
			() => ({ id: "open-save-folder", label: labels.openSaveFolder }),
		],
		[() => ({ id: "edit-details", label: labels.editDetails })],
		[() => ({ id: "sync-now", label: labels.syncNow }), cloudSyncAction],
		[
			() => ({ id: "launch-options", label: labels.launchOptions }),
			() => ({ id: "create-shortcut", label: labels.createShortcut }),
		],
		[
			() => ({
				id: "remove-library",
				label: labels.removeFromLibrary,
				tone: "danger",
			}),
		],
	],
};

export function getGameMenuGroups(
	context: GameMenuContext,
	state: MenuActionState,
): MenuAction[][] {
	return menuConfig[context].map((group) =>
		group.map((resolver) => resolver(state)),
	);
}
