import placeholderBanner from "$lib/assets/images/placeholders/placeholder-banner.svg";
import placeholderHorizontal from "$lib/assets/images/placeholders/placeholder-horizontal.svg";
import placeholderVertical from "$lib/assets/images/placeholders/placeholder-vertical.svg";
import type { Game } from "$lib/types/Game";

export type GameMediaType = "vertical" | "horizontal" | "banner";

const placeholderByType: Record<GameMediaType, string> = {
	vertical: placeholderVertical,
	horizontal: placeholderHorizontal,
	banner: placeholderBanner,
};

export function getGameImage(game: Game, type: GameMediaType) {
	const source =
		type === "vertical"
			? game.coverVertical
			: type === "horizontal"
				? game.coverHorizontal
				: game.banner;

	return source || placeholderByType[type];
}
