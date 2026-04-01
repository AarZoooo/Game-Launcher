import type { AccentTone } from "$lib/types/Game";

export interface NavItem {
	label: string;
	path: string;
}

export interface FooterColumn {
	title: string;
	links: string[];
}

export const appBrand = {
	name: "Scoped",
	version: "v 0.1.1",
	title: "Scoped Launcher",
};

export const sidebarProfile = {
	initial: "N",
	name: "NezukoChan",
	status: "Online",
};

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

export const routeAccents: Record<string, AccentTone> = {
	"/": "gold",
	"/games": "silver",
	"/explore": "silver",
	"/settings": "silver",
};
