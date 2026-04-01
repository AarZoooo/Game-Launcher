import epicIcon from "$lib/assets/icons/platforms/epic.svg";
import steamIcon from "$lib/assets/icons/platforms/steam.svg";
import backIcon from "$lib/assets/icons/ui/back.svg";
import moreIcon from "$lib/assets/icons/ui/more.svg";
import playIcon from "$lib/assets/icons/ui/play.svg";
import emptyLibraryIllustration from "$lib/assets/illustrations/empty-library.svg";
import errorStateIllustration from "$lib/assets/illustrations/error-state.svg";
import noResultsIllustration from "$lib/assets/illustrations/no-results.svg";
import defaultBannerImage from "$lib/assets/images/banners/default-banner.svg";
import defaultCoverImage from "$lib/assets/images/placeholders/game-cover.svg";
import type { Game } from "$lib/types/Game";

export const appIcons = {
	ui: {
		back: backIcon,
		more: moreIcon,
		play: playIcon,
	},
	platforms: {
		epic: epicIcon,
		steam: steamIcon,
	},
} as const;

export const appImages = {
	placeholders: {
		gameCover: defaultCoverImage,
	},
	banners: {
		default: defaultBannerImage,
	},
} as const;

export const appIllustrations = {
	emptyLibrary: emptyLibraryIllustration,
	noResults: noResultsIllustration,
	errorState: errorStateIllustration,
} as const;

const legacyMockAliases: Record<string, string> = {
	"/mock/elden-card.png": defaultCoverImage,
	"/mock/sekiro-card.png": defaultCoverImage,
	"/mock/home-hero.png": defaultBannerImage,
	"/mock/cs2-card.png": defaultCoverImage,
	"/mock/satisfactory-card.png": defaultCoverImage,
	"/mock/hades-card.png": defaultCoverImage,
	"/mock/uncharted-card.png": defaultCoverImage,
	"/mock/rust-card.png": defaultCoverImage,
	"/mock/stardew-card.png": defaultCoverImage,
	"/mock/stardew-hero.png": defaultBannerImage,
	"/mock/ghost-tsushima-card.png": defaultCoverImage,
	"/mock/ghost-hero.png": defaultBannerImage,
	"/mock/rdr2-card.png": defaultCoverImage,
	"/mock/hollow-knight-card.png": defaultCoverImage,
	"/mock/cyberpunk-card.png": defaultCoverImage,
	"/mock/cyberpunk-hero.png": defaultBannerImage,
	"/mock/valorant-card.png": defaultCoverImage,
	"/mock/battlefield-card.png": defaultCoverImage,
	"/mock/ori-card.png": defaultCoverImage,
	"/mock/forza-card.png": defaultCoverImage,
	"/mock/cod-card.png": defaultCoverImage,
	"/mock/it-takes-two-card.png": defaultCoverImage,
	"/mock/sea-of-thieves-card.png": defaultCoverImage,
	"/mock/a-way-out-card.png": defaultCoverImage,
	"/mock/nomanssky-card.png": defaultCoverImage,
	"/mock/nioh2-card.png": defaultCoverImage,
	"/mock/witcher3-card.png": defaultCoverImage,
	"/mock/roots-pacha-card.png": defaultCoverImage,
	"/mock/coral-island-card.png": defaultCoverImage,
	"/mock/sun-haven-card.png": defaultCoverImage,
	"/mock/sandrock-card.png": defaultCoverImage,
	"/mock/deus-ex-card.png": defaultCoverImage,
	"/mock/ascent-card.png": defaultCoverImage,
	"/mock/ghostrunner-card.png": defaultCoverImage,
	"/mock/observer-card.png": defaultCoverImage,
};

function normalizeLegacyAsset(path?: string) {
	if (!path) return undefined;
	return legacyMockAliases[path] || path;
}

export function getGameCover(game: Pick<Game, "cover"> | { cover?: string }) {
	return normalizeLegacyAsset(game.cover) || appImages.placeholders.gameCover;
}

export function getGameBanner(
	game: Pick<Game, "hero" | "cover"> | { hero?: string; cover?: string },
) {
	return (
		normalizeLegacyAsset(game.hero) ||
		normalizeLegacyAsset(game.cover) ||
		appImages.banners.default
	);
}

export function getEmptyStateIllustration(kind: keyof typeof appIllustrations) {
	return appIllustrations[kind];
}

export function getPlatformIcon(platform: keyof typeof appIcons.platforms) {
	return appIcons.platforms[platform];
}
