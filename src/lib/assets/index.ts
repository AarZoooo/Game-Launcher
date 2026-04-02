import epicIcon from "$lib/assets/icons/platforms/epic.svg";
import steamIcon from "$lib/assets/icons/platforms/steam.svg";
import backIcon from "$lib/assets/icons/ui/back.svg";
import moreIcon from "$lib/assets/icons/ui/more.svg";
import playIcon from "$lib/assets/icons/ui/play.svg";
import refreshIcon from "$lib/assets/icons/ui/refresh.svg";
import emptyLibraryIllustration from "$lib/assets/illustrations/empty-library.svg";
import errorStateIllustration from "$lib/assets/illustrations/error-state.svg";
import noResultsIllustration from "$lib/assets/illustrations/no-results.svg";

export const appIcons = {
	ui: {
		back: backIcon,
		more: moreIcon,
		play: playIcon,
		refresh: refreshIcon,
	},
	platforms: {
		epic: epicIcon,
		steam: steamIcon,
	},
} as const;

export const appIllustrations = {
	emptyLibrary: emptyLibraryIllustration,
	noResults: noResultsIllustration,
	errorState: errorStateIllustration,
} as const;

export function getEmptyStateIllustration(kind: keyof typeof appIllustrations) {
	return appIllustrations[kind];
}

export function getPlatformIcon(platform: keyof typeof appIcons.platforms) {
	return appIcons.platforms[platform];
}
