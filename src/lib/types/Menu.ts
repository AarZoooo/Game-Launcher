import type { Game } from "$lib/types/Game";

export type GameMenuContext = "library" | "explore" | "home";
export type MenuPlacement = "below-right" | "side-right" | "above-right";
export type MenuActionTone = "default" | "danger";

export type GameMenuActionId =
	| "play"
	| "resume"
	| "restart"
	| "toggle-favorite"
	| "open-folder"
	| "open-save-folder"
	| "edit-details"
	| "sync-now"
	| "toggle-cloud-sync"
	| "launch-options"
	| "create-shortcut"
	| "remove-library"
	| "status-want"
	| "status-playing"
	| "status-played"
	| "hide-continue"
	| "view-playtime";

export interface MenuAction {
	id: GameMenuActionId;
	label: string;
	tone?: MenuActionTone;
	disabled?: boolean;
}

export interface MenuActionState {
	game: Game;
	isActiveGame: boolean;
}

export type MenuActionFactory = (state: MenuActionState) => MenuAction;
