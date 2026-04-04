<script lang="ts">
import ContinuePlaying from "$lib/components/game/ContinuePlaying.svelte";
import GameGrid from "$lib/components/game/GameGrid.svelte";
import StatsDashboard from "$lib/components/stats/StatsDashboard.svelte";
import { pageLabels } from "$lib/data/labels";
import { performGameAction } from "$lib/services/gameService";
import {
	continuePlayingGames,
	getAllGames,
	getGameById,
	getGamesByIds,
	homeExploreIds,
} from "$lib/stores/libraryStore";
import type { Game } from "$lib/types/Game";
import type { GameMenuActionId } from "$lib/types/Menu";

let featuredGame = getGameById("sekiro");
const suggestedGames = getGamesByIds(homeExploreIds);

function handleAction(event: CustomEvent<{ id: string; game: Game }>) {
	return performGameAction(
		event.detail.id as GameMenuActionId,
		event.detail.game,
	);
}

$: featuredGame =
	$continuePlayingGames[0] ||
	getAllGames().find((game) => game.inLibrary !== false) ||
	getGameById(homeExploreIds[0]) ||
	getGameById("sekiro");
</script>

{#if featuredGame}
  <div class="home">
    <ContinuePlaying game={featuredGame} on:action={handleAction} />

    <section>
      <div class="section-header">
        <h2>{pageLabels.home.recentlyPlayed}</h2>
        <span>{pageLabels.home.seeMore}</span>
      </div>
      <GameGrid
        games={$continuePlayingGames}
        horizontal
        compact
        context="home"
        menuPlacement="above-right"
        on:action={handleAction}
      />
    </section>

    <section>
      <div class="section-header">
        <h2>{pageLabels.home.exploreNew}</h2>
        <span>{pageLabels.home.seeMore}</span>
      </div>
      <GameGrid
        games={suggestedGames}
        horizontal
        compact
        context="explore"
        menuPlacement="above-right"
        on:action={handleAction}
      />
    </section>

    <StatsDashboard />
  </div>
{/if}

<style>
  .home {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 1rem;
  }

  h2 {
    margin: 0;
    font-size: var(--font-size-body);
  }

  span {
    color: var(--text-muted);
    font-size: var(--font-size-caption);
    cursor: pointer;
  }
</style>
