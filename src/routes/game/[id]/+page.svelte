<script lang="ts">
  import GameDetails from '$lib/components/game/GameDetails.svelte';
  import { getGameById, getGamesByIds } from '$lib/stores/libraryStore';
  import { page } from '$app/stores';

  $: gameId = $page.params.id || 'ghost-yotei';
  $: game = getGameById(gameId) || getGameById('ghost-yotei');
  $: similarGames = getGamesByIds(game?.similarIds || []);
</script>

{#if game}
  <GameDetails {game} {similarGames} />
{:else}
  <p>Game not found.</p>
{/if}
