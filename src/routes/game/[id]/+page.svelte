<script lang="ts">
import { page } from "$app/stores";
import GameDetails from "$lib/components/game/GameDetails.svelte";
import { pageLabels } from "$lib/data/labels";
import { getGameById, getGamesByIds } from "$lib/stores/libraryStore";

$: gameId = $page.params.id || "ghost-yotei";
$: game = getGameById(gameId) || getGameById("ghost-yotei");
$: similarGames = getGamesByIds(game?.similarIds || []);
$: requestedBackHref = $page.url.searchParams.get("from") || "/";
$: backHref = requestedBackHref.startsWith("/") ? requestedBackHref : "/";
</script>

{#if game}
  <GameDetails {game} {similarGames} {backHref} />
{:else}
  <p>{pageLabels.game.notFound}</p>
{/if}
