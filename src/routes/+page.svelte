<script lang="ts">
import EmptyState from "$lib/components/common/EmptyState.svelte";
import ContinuePlaying from "$lib/components/game/ContinuePlaying.svelte";
import GameCardSkeleton from "$lib/components/game/GameCardSkeleton.svelte";
import GameGrid from "$lib/components/game/GameGrid.svelte";
import StatsDashboard from "$lib/components/stats/StatsDashboard.svelte";
import { pageLabels } from "$lib/data/labels";
import { performGameAction } from "$lib/services/gameService";
import {
	continuePlayingGames,
	games,
	homeExploreIds,
	libraryHydrated,
} from "$lib/stores/libraryStore";
import { activeGameId } from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";
import type { GameMenuActionId } from "$lib/types/Menu";

let featuredGame: Game | undefined;
let suggestedGames: Game[] = [];
const recentSkeletons = Array.from({ length: 5 }, (_, index) => index);

function handleAction(event: CustomEvent<{ id: string; game: Game }>) {
	return performGameAction(
		event.detail.id as GameMenuActionId,
		event.detail.game,
	);
}

$: featuredGame =
	($activeGameId
		? $games.find((game) => game.id === $activeGameId)
		: undefined) || $continuePlayingGames[0];
$: suggestedGames = homeExploreIds
	.map((id) => $games.find((game) => game.id === id))
	.filter((game): game is Game => Boolean(game));
</script>

<div class="home">
	<ContinuePlaying
		game={featuredGame}
		loading={!$libraryHydrated}
		on:action={handleAction}
	/>

	<section>
		<div class="section-header">
			<h2>{pageLabels.home.recentlyPlayed}</h2>
			<span>{pageLabels.home.seeMore}</span>
		</div>

		{#if !$libraryHydrated}
			<div class="skeleton-row" aria-hidden="true">
				{#each recentSkeletons as skeleton (skeleton)}
					<GameCardSkeleton compact />
				{/each}
			</div>
		{:else if $continuePlayingGames.length}
			<GameGrid
				games={$continuePlayingGames}
				horizontal
				compact
				context="home"
				menuPlacement="above-right"
				on:action={handleAction}
			/>
		{:else}
			<EmptyState
				kind="emptyLibrary"
				variant="full-width"
				title={pageLabels.home.recentlyPlayedEmptyTitle}
				subtitle={pageLabels.home.recentlyPlayedEmptySubtitle}
			/>
		{/if}
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
    margin-bottom: var(--space-4);
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

	.skeleton-row {
		display: flex;
		gap: var(--space-6);
		min-height: calc(var(--card-height-sm) + var(--space-4));
		overflow-x: auto;
		scrollbar-width: none;
	}

	.skeleton-row::-webkit-scrollbar {
		display: none;
	}
</style>
