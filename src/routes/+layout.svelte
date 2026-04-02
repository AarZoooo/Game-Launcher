<script lang="ts">
import { onMount } from "svelte";
import "$lib/styles/index.css";
import AppShell from "$lib/components/layout/AppShell.svelte";
import { appBrand } from "$lib/data/navigation";
import { listenForGameProcessEvents } from "$lib/services/tauriService";
import { games } from "$lib/stores/libraryStore";
import { uiStore } from "$lib/stores/uiStore";

onMount(() => {
	// TODO: Keep temporary frontend-only features until matching backend commands exist.
	void games.loadFromBackend().catch((error) => {
		console.error("Failed to load stored library:", error);
	});

	let disposed = false;
	let stopListening = () => {};

	void listenForGameProcessEvents((event) => {
		if (disposed) return;

		if (event.state === "started") {
			uiStore.setGameRunning(true, event.gameId);
			return;
		}

		if (event.state === "exited") {
			uiStore.finishGame(event.gameId);
			void games.loadFromBackend().catch((error) => {
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

			stopListening = unlisten;
		})
		.catch((error) => {
			console.error("Failed to listen for game process events:", error);
		});

	return () => {
		disposed = true;
		stopListening();
	};
});
</script>

<svelte:head>
  <title>{appBrand.title}</title>
</svelte:head>

<AppShell>
  <slot />
</AppShell>
