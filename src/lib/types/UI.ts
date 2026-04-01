export type ThemeMode = "dark" | "light" | "dynamic";
export type UIModeVariant = "auto" | "normal" | "gaming";
export type ResolvedUIMode = "normal" | "gaming";
export type ScanPlatform = "steam" | "epic" | "local";

export interface ScanState {
	steam: boolean;
	epic: boolean;
	local: boolean;
}

export interface UIState {
	isGameRunning: boolean;
	activeGameId: string | null;
	activeMenuKey: string | null;
	performanceMode: boolean;
	scanning: ScanState;
	activeAccent: string;
	theme: ThemeMode;
	libraryBusy: boolean;
	libraryBusyMessage: string;
}

export interface SelectOption {
	label: string;
	value: string;
}

export interface GameFilterState {
	genre: string;
	coop: string;
	status: string;
	showFavorites: boolean;
}

export interface SettingField {
	label: string;
	type: "select" | "radio" | "text";
	value: string;
	options?: string[];
}

export interface SettingSection {
	title: string;
	fields: SettingField[];
}
