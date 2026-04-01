import { pageLabels } from "$lib/data/labels";

export function getSyncNowMessage(title: string) {
	return pageLabels.messages.syncPlaceholder(title);
}

export function getSyncShortcutMessage(title: string) {
	return pageLabels.messages.shortcutPlaceholder(title);
}
