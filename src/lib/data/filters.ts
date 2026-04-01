import type { GameFilterState, SelectOption } from "$lib/types/UI";

export const defaultGameFilters: GameFilterState = {
	genre: "",
	coop: "",
	status: "",
	showFavorites: false,
};

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
