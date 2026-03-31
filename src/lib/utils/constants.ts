import type { AccentTone } from "$lib/stores/libraryStore";

export interface NavItem {
	label: string;
	path: string;
}

export interface FooterColumn {
	title: string;
	links: string[];
}

export interface GenreShare {
	label: string;
	value: string;
	percent: number;
}

export interface SelectOption {
	label: string;
	value: string;
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

export const navItems: NavItem[] = [
	{ label: "Home", path: "/" },
	{ label: "All Games", path: "/games" },
	{ label: "Explore", path: "/explore" },
	{ label: "Settings", path: "/settings" },
];

export const footerColumns: FooterColumn[] = [
	{ title: "Credits", links: ["Terms of Service"] },
	{ title: "Source Code", links: ["Contribute"] },
	{ title: "Documentation", links: ["Use of AI"] },
	{ title: "Pricing", links: ["Talk to us"] },
];

export const sortOptions: SelectOption[] = [
	{ label: "Select", value: "default" },
	{ label: "Playtime", value: "hours" },
	{ label: "Name", value: "title" },
	{ label: "Rating", value: "rating" },
];

export const genreOptions: SelectOption[] = [
	{ label: "Select", value: "" },
	{ label: "Action", value: "action" },
	{ label: "Adventure", value: "adventure" },
	{ label: "RPG", value: "rpg" },
	{ label: "Simulation", value: "simulation" },
];

export const statusOptions: SelectOption[] = [
	{ label: "Select", value: "" },
	{ label: "Want to Play", value: "want" },
	{ label: "Playing", value: "playing" },
	{ label: "Played", value: "played" },
];

export const coopOptions: SelectOption[] = [
	{ label: "Select", value: "" },
	{ label: "Yes", value: "yes" },
	{ label: "No", value: "no" },
];

export const routeAccents: Record<string, AccentTone> = {
	"/": "gold",
	"/games": "silver",
	"/explore": "silver",
	"/settings": "silver",
};

export const genreShares: GenreShare[] = [
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
			{ label: "Import/Export Data", type: "radio", value: "Export • Import" },
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

export const recommendationPrompt =
	"I want to play an action adventure game with my friends";
