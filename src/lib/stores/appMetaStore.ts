import { derived, writable } from "svelte/store";

import { getAppVersion } from "$lib/services/tauriService";

type AppMetaState = {
	version: string | null;
	loaded: boolean;
};

const initialState: AppMetaState = {
	version: null,
	loaded: false,
};

let versionRequest: Promise<void> | null = null;

function createAppMetaStore() {
	const { subscribe, update } = writable<AppMetaState>(initialState);

	return {
		subscribe,
		async loadVersion() {
			if (versionRequest) {
				return versionRequest;
			}

			versionRequest = getAppVersion()
				.then((version) => {
					update((state) => ({
						...state,
						version: version?.trim() || null,
						loaded: true,
					}));
				})
				.catch((error) => {
					console.error("Failed to load app version:", error);
					update((state) => ({
						...state,
						loaded: true,
					}));
				});

			return versionRequest;
		},
	};
}

export const appMetaStore = createAppMetaStore();
export const appVersion = derived(
	appMetaStore,
	($appMetaStore) => $appMetaStore.version,
);
