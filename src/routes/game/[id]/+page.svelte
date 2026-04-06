<script lang="ts">
import { page } from "$app/stores";
import GameDetails from "$lib/components/game/GameDetails.svelte";
import { pageLabels } from "$lib/data/labels";
import { games } from "$lib/stores/libraryStore";
import type { Game } from "$lib/types/Game";

$: gameId = $page.params.id || "ghost-yotei";
$: game =
	$games.find((entry) => entry.id === gameId) ||
	$games.find((entry) => entry.id === "ghost-yotei");
$: similarGames = (game?.similarIds || [])
	.map((id) => $games.find((entry) => entry.id === id))
	.filter((entry): entry is Game => Boolean(entry));
$: requestedBackHref = $page.url.searchParams.get("from") || "/";
$: backHref = requestedBackHref.startsWith("/") ? requestedBackHref : "/";
</script>

{#if game}
  <GameDetails {game} {similarGames} {backHref} />
{:else}
  <p>{pageLabels.game.notFound}</p>
{/if}
