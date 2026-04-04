<script lang="ts">
import { onMount } from "svelte";
import "$lib/styles/index.css";
import AppShell from "$lib/components/layout/AppShell.svelte";
import { appBrand } from "$lib/data/navigation";
import {
	listenForGameMediaResolutionEvents,
	listenForGameMediaUpdates,
	listenForGameProcessEvents,
} from "$lib/services/tauriService";
import { games } from "$lib/stores/libraryStore";
import { uiStore } from "$lib/stores/uiStore";

onMount(() => {
	let disposed = false;
	let initialLoadStarted = false;
	let stopProcessListening = () => {};
	let stopMediaListening = () => {};
	let stopMediaResolutionListening = () => {};

	function startInitialLoad() {
		if (disposed || initialLoadStarted) {
			return;
		}

		initialLoadStarted = true;
		void games
			.loadFromBackend()
			.then(() => games.resolveIgdbCovers())
			.catch((error) => {
				console.error("Failed to load stored library:", error);
			});
	}

	void listenForGameMediaUpdates((game) => {
		if (disposed) return;
		games.applyBackendGameUpdate(game);
	})
		.then((unlisten) => {
			if (disposed) {
				unlisten();
				return;
			}

			stopMediaListening = unlisten;
			startInitialLoad();
		})
		.catch((error) => {
			console.error("Failed to listen for game media updates:", error);
			startInitialLoad();
		});

	void listenForGameMediaResolutionEvents((event) => {
		if (disposed) return;
		games.setMediaLoading(
			{ gameId: event.gameId, exePath: event.exePath },
			event.state === "started",
		);
	})
		.then((unlisten) => {
			if (disposed) {
				unlisten();
				return;
			}

			stopMediaResolutionListening = unlisten;
		})
		.catch((error) => {
			console.error(
				"Failed to listen for game media resolution events:",
				error,
			);
		});

	void listenForGameProcessEvents((event) => {
		if (disposed) return;

		if (event.state === "started") {
			uiStore.setGameRunning(true, event.gameId);
			return;
		}

		if (event.state === "exited") {
			uiStore.finishGame(event.gameId);
			void games.refreshAfterGameExit(event.gameId).catch((error) => {
				console.error("Failed to refresh library after game exit:", error);
			});
			return;
		}

		console.error("Game process event error:", event.message || event.exePath);
	})
		.then((unlisten) => {
			if (disposed) {
				unlisten();
				return;
			}

			stopProcessListening = unlisten;
		})
		.catch((error) => {
			console.error("Failed to listen for game process events:", error);
		});

	return () => {
		disposed = true;
		stopProcessListening();
		stopMediaListening();
		stopMediaResolutionListening();
	};
});
</script>

<svelte:head>
  <title>{appBrand.title}</title>
</svelte:head>

<AppShell>
  <slot />
</AppShell>
