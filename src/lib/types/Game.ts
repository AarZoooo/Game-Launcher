export type AccentTone = "gold" | "olive" | "silver";

export type PlatformType =
	| "steam"
	| "epic"
	| "local"
	| "suggested"
	| "wishlist";

export type GameStatus = "want" | "playing" | "played" | "none";

export type ResumeState = "resume" | "restart";

export interface GameMetric {
	label: string;
	value: string;
}

export interface ImportedGameResult {
	id: string;
	title: string;
	path: string;
	platform: "steam" | "epic" | "local";
}

export interface Game {
	id: string;
	title: string;
	hours: string;
	platform: string;
	platformType?: PlatformType;
	genres: string;
	rating: string;
	coop: string;
	completion: string;
	cover: string;
	hero?: string;
	accent: AccentTone;
	accentHex?: string;
	accentColor?: string;
	path?: string;
	savePath?: string;
	lastPlayed?: string;
	totalPlaytime?: string;
	syncStatus?: string;
	blurb?: string;
	genreLabel?: string;
	inLibrary?: boolean;
	featured?: boolean;
	inRecommendations?: boolean;
	similarIds?: string[];
	metrics?: GameMetric[];
	status?: GameStatus;
	favorite?: boolean;
	cloudSyncEnabled?: boolean;
	launchOptions?: string;
	tags?: string[];
	hiddenFromContinue?: boolean;
	resumeState?: ResumeState;
	storageDescription?: string;
	storageGenres?: string[];
	storageLastPlayedRaw?: string | null;
}
