<script lang="ts">
  import ContinuePlaying from '$lib/components/game/ContinuePlaying.svelte';
  import GameGrid from '$lib/components/game/GameGrid.svelte';
  import StatsDashboard from '$lib/components/stats/StatsDashboard.svelte';
  import { launchGame, openGameFolder } from '$lib/services/tauriService';
  import {
    continuePlayingGames,
    games,
    getAllGames,
    getGameById,
    getGamesByIds,
    homeExploreIds,
    type Game
  } from '$lib/stores/libraryStore';

  let featuredGame = getGameById('sekiro');
  const suggestedGames = getGamesByIds(homeExploreIds);

  function handleAction(event: CustomEvent<{ id: string; game: Game }>) {
    const { id, game } = event.detail;

    if (id === 'status-want') return games.setStatus(game.id, 'want');
    if (id === 'status-playing') return games.setStatus(game.id, 'playing');
    if (id === 'status-played') return games.setStatus(game.id, 'played');
    if ((id === 'play' || id === 'resume' || id === 'restart') && game.path) {
      return launchGame(game.path, game.id);
    }
    if (id === 'toggle-favorite') return games.toggleFavorite(game.id);
    if (id === 'open-folder') return openGameFolder(game.path);
    if (id === 'hide-continue') return games.hideFromContinuePlaying(game.id);
    if (id === 'view-playtime') {
      window.alert(`${game.title}: ${game.totalPlaytime || game.hours} total playtime.`);
    }
  }

  $: featuredGame =
    $continuePlayingGames[0] ||
    getAllGames().find((game) => game.inLibrary !== false) ||
    getGameById(homeExploreIds[0]) ||
    getGameById('sekiro');
</script>

{#if featuredGame}
  <div class="home">
    <ContinuePlaying game={featuredGame} on:action={handleAction} />

    <section>
      <div class="section-header">
        <h2>Recently Played</h2>
        <span>See more</span>
      </div>
      <GameGrid games={$continuePlayingGames} horizontal compact context="home" on:action={handleAction} />
    </section>

    <section>
      <div class="section-header">
        <h2>Explore New</h2>
        <span>See more</span>
      </div>
      <GameGrid games={suggestedGames} horizontal compact context="explore" on:action={handleAction} />
    </section>

    <StatsDashboard />
  </div>
{/if}

<style>
  .home {
    display: flex;
    flex-direction: column;
    gap: 2rem;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 1rem;
  }

  h2 {
    margin: 0;
    font-size: 0.98rem;
  }

  span {
    color: rgba(226, 223, 231, 0.42);
    font-size: 0.74rem;
    cursor: pointer;
  }
</style>
