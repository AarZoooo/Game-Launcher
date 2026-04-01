import type { AccentTone, Game } from "$lib/types/Game";

const toneMap: Record<AccentTone, string> = {
	gold: "#b69b57",
	olive: "#8a9a54",
	silver: "#d6d7dc",
};

function normalizeHex(value: string) {
	const trimmed = value.trim();

	if (/^#[0-9a-f]{6}$/i.test(trimmed)) {
		return trimmed.toLowerCase();
	}

	if (/^#[0-9a-f]{3}$/i.test(trimmed)) {
		const [r, g, b] = trimmed.slice(1).split("");
		return `#${r}${r}${g}${g}${b}${b}`.toLowerCase();
	}

	return null;
}

function clampChannel(channel: number) {
	return Math.min(215, Math.max(36, Math.round(channel)));
}

export function hexToRgbTriplet(hex: string) {
	const normalized = normalizeHex(hex) || toneMap.gold;
	const red = parseInt(normalized.slice(1, 3), 16);
	const green = parseInt(normalized.slice(3, 5), 16);
	const blue = parseInt(normalized.slice(5, 7), 16);

	return `${clampChannel(red)} ${clampChannel(green)} ${clampChannel(blue)}`;
}

export function resolveAccentHex(
	source?: Pick<Game, "accentColor" | "accentHex" | "accent">,
) {
	return (
		(source?.accentColor && normalizeHex(source.accentColor)) ||
		(source?.accentHex && normalizeHex(source.accentHex)) ||
		toneMap[source?.accent || "gold"]
	);
}

function contrastColor(rgb: string) {
	const [red, green, blue] = rgb.split(" ").map(Number);
	const luminance = (0.2126 * red + 0.7152 * green + 0.0722 * blue) / 255;

	return luminance > 0.62 ? "#14161b" : "#f6f7fb";
}

export function resolveAccentPresentation(
	source?: Pick<Game, "accentColor" | "accentHex" | "accent">,
) {
	const hex = resolveAccentHex(source);
	const rgb = hexToRgbTriplet(hex);

	return {
		hex,
		rgb,
		text: contrastColor(rgb),
	};
}
