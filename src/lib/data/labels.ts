import type { SettingSection } from "$lib/types/UI";

export const pageLabels = {
	common: {
		confirm: "Confirm",
		cancel: "Cancel",
		goBack: "Go back",
		favorite: "Favorite",
		openMenuFor: (title: string) => `Open menu for ${title}`,
	},
	platforms: {
		steam: "Steam",
		epic: "Epic",
		local: "Local",
	},
	home: {
		recentlyPlayed: "Recently Played",
		exploreNew: "Explore New",
		seeMore: "See more",
	},
	games: {
		title: "All Games",
		sortBy: "Sort by:",
		filters: "Filters",
		importSteam: "Import Steam",
		importEpic: "Import Epic",
		autoSearch: "Auto search",
		searching: "Searching...",
		addManually: "Add manually",
		installedGames: "Installed Games",
		installedDescription: "Loaded from your local backend storage.",
		catalogue: "Catalogue",
		catalogueDescription:
			"Temporary catalog and recommendation data kept separate from your installed library.",
		installedEmpty: "No installed games match the current filters.",
		catalogEmpty: "No catalog games match the current filters.",
		scanFailed: "Scan failed. Please try again.",
		autoSearchFailed: "Auto search failed. Please try again.",
		manualAddFailed: "Manual add failed. Please try again.",
		shownSuffix: "shown",
		localAddedSingular: "Added 1 local game to your installed list.",
	},
	explore: {
		recommendedForYou: "Recommended for you",
		suggestedGames: "Suggested Games",
		basedOnWhatYouPlay: "based on what you play",
		recommendationAssistant: "Recommendation Assistant",
		assistantHint: "just tell us what you want",
		suggestionsPending: "Suggestions will show up below",
		go: "Go",
		refresh: "Refresh",
		offlineTitle: "It seems that you're offline.",
		offlineBody: "Come back when you have an active internet connection.",
	},
	game: {
		notFound: "Game not found.",
		similarTitles: "Similar Titles",
		genres: "Genres",
		rating: "Rating",
		coopSupport: "Co-Op Support",
		completionTime: "Completion Time",
	},
	stats: {
		yearlyActivity: "Yearly activity",
		playStreak: "Play Streak",
		totalHours: "Total Hours",
		longestStreak: "Longest streak",
		currentStreak: "Current streak",
		gamesPlayed: "Games Played",
		activeRightNow: "Active Right Now",
		favorites: "Favorites",
		mostPlayed: "Most Played",
		noDataYet: "No data yet",
	},
	filterPanel: {
		title: "Filters",
		close: "Close filters",
		showFavorites: "Show only favourites",
		status: "Status",
		genres: "Genres",
		coopSupport: "Co-op Support",
		apply: "Apply Filters",
	},
	importModal: {
		autoSearch: "Auto Search",
		importPrefix: "Import",
		searching:
			"Searching likely game folders and checking for known game files...",
		reviewResults:
			"Review the detected games below. Checked items will be added to your library.",
		noResults: "No games were detected on your connected drives.",
		selectAll: "Select all",
		cancel: "Cancel",
		addAll: "Add All",
		addSelected: "Add Selected",
		scanning: "Scanning...",
	},
	settings: {
		title: "Settings",
	},
	continuePlaying: {
		eyebrow: "Continue playing",
	},
	actions: {
		play: "Play",
		playing: "Playing",
		openGame: (title: string) => `Open ${title}`,
		favorite: "Favorite",
		addFavorite: "Add Favorite",
		removeFavorite: "Remove Favorite",
		openFolder: "Open Folder",
		openGameFolder: "Open Game Folder",
		openSaveFolder: "Open Save Folder",
		editDetails: "Edit Details",
		syncNow: "Sync Now",
		enableCloudSync: "Toggle Cloud Sync",
		disableCloudSync: "Disable Cloud Sync",
		launchOptions: "Launch Options",
		createShortcut: "Create Desktop Shortcut",
		removeFromLibrary: "Remove from Library",
		wantToPlay: "Want to Play",
		played: "Played",
		resume: "Resume",
		restart: "Restart",
		removeFromContinue: "Remove from Continue Playing",
		viewPlaytime: "View Playtime Details",
	},
	buttons: {
		play: "Play",
		playing: "Playing",
	},
	fields: {
		gameTitlePrompt: "Game title",
		launchArgumentsPrompt: "Launch arguments",
	},
	messages: {
		launchErrorPrefix: "Failed to launch game:",
		storedLocally: "Stored locally",
		localLibrary: "Local library",
		addedToLibrary: (title: string) =>
			`Added ${title} to your installed games list.`,
		addedLocalGames: (count: number) =>
			`Added ${count} local game${count === 1 ? "" : "s"} to your installed list.`,
		playtimeDetails: (title: string, playtime: string) =>
			`${title}: ${playtime} total playtime.`,
		shortcutPlaceholder: (title: string) =>
			`Desktop shortcut will be supported by the backend layer for ${title}.`,
		syncPlaceholder: (title: string) =>
			`Cloud sync for ${title} will be handled once backend sync is connected.`,
	},
};

export const recommendationPrompt =
	"I want to play an action adventure game with my friends";

export const genreShares = [
	{ label: "Action", value: "40%", percent: 40 },
	{ label: "Adventure", value: "27%", percent: 27 },
	{ label: "RPG", value: "20%", percent: 20 },
	{ label: "Sports", value: "13%", percent: 13 },
];

export const settingsSections: SettingSection[] = [
	{
		title: "General",
		fields: [
			{
				label: "Launch on System Startup",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{
				label: "When closed, app runs in background",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{
				label: "When minimizing, hide to Tray instead",
				type: "select",
				value: "No",
				options: ["Yes", "No"],
			},
			{
				label: "Automatically update the app",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{
				label: "UI Language",
				type: "select",
				value: "English",
				options: ["English", "Hindi"],
			},
			{
				label: "Minimize when game starts",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{ label: "Launch Arguments", type: "text", value: "" },
		],
	},
	{
		title: "Appearance",
		fields: [
			{
				label: "Theme",
				type: "select",
				value: "Dark",
				options: ["Dark", "Light", "Dynamic"],
			},
			{
				label: "Primary Color",
				type: "select",
				value: "Auto",
				options: ["Auto", "Gold", "Olive", "Silver"],
			},
			{
				label: "Blur Effect",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{
				label: "Animations",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{
				label: "Game UI view",
				type: "select",
				value: "Grid",
				options: ["Grid", "List"],
			},
		],
	},
	{
		title: "Scanning",
		fields: [
			{ label: "Directories to Scan", type: "radio", value: "C:/Games/" },
			{ label: "Directories to Exclude", type: "radio", value: "C:/Windows/" },
			{
				label: "Re-scan directories on every startup",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
			{
				label: "Scan for games in external launchers like Steam, Epic etc.",
				type: "select",
				value: "Yes",
				options: ["Yes", "No"],
			},
		],
	},
	{
		title: "Data",
		fields: [
			{
				label: "Cloud Sync platform",
				type: "select",
				value: "Off",
				options: ["Off", "Google Drive", "Dropbox", "OneDrive"],
			},
			{ label: "Import/Export Data", type: "radio", value: "Export / Import" },
			{ label: "Clear Data", type: "radio", value: "Clear" },
		],
	},
	{
		title: "Advanced",
		fields: [
			{
				label: "Enable debug logs",
				type: "select",
				value: "No",
				options: ["Yes", "No"],
			},
			{
				label: "Log file directory",
				type: "radio",
				value: "C:/Documents/Scoped/logs/",
			},
		],
	},
];
