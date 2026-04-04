<script lang="ts">
import { createEventDispatcher } from "svelte";
import GameCard from "$lib/components/game/GameCard.svelte";
import type { Game } from "$lib/types/Game";
import type { GameMenuContext, MenuPlacement } from "$lib/types/Menu";

const dispatch = createEventDispatcher<{
	action: { id: string; game: Game };
}>();

export let games: Game[] = [];
export let horizontal = false;
export let compact = false;
export let context: GameMenuContext = "library";
export let menuPlacement: MenuPlacement = "above-right";
</script>

<div class:horizontal class:compact class="grid">
  {#each games as game}
    <div class="item">
      <GameCard
        {game}
        {compact}
        {context}
        {menuPlacement}
        on:action={(event) => dispatch('action', event.detail)}
      />
    </div>
  {/each}
</div>

<style>
  .item {
    position: relative;
    z-index: 0;
    min-width: 0;
    min-height: 0;
    display: flex;
  }

  .item:hover,
  .item:focus-within {
    z-index: 18;
  }

  .grid {
    --card-width: var(--card-width-md);
    --card-height: var(--card-height-md);
    --card-title-height: var(--card-title-height-md);
    --card-info-height: var(--card-info-height-md);
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(var(--card-width), 1fr));
    gap: var(--space-9) var(--space-6);
    align-items: start;
  }

  .grid.compact {
    --card-width: var(--card-width-sm);
    --card-height: var(--card-height-sm);
    --card-title-height: var(--card-title-height-sm);
    --card-info-height: var(--card-info-height-sm);
  }

  .horizontal {
    --menu-headroom: 13.5rem;
    display: flex;
    gap: var(--space-6);
    overflow-x: auto;
    scrollbar-width: none;
    padding: var(--menu-headroom) 0 var(--space-2);
    margin-top: calc(var(--menu-headroom) * -1);
  }

  .horizontal::-webkit-scrollbar {
    display: none;
  }

  .horizontal .item {
    flex: 0 0 var(--card-width);
    min-width: var(--card-width);
  }

  .item :global(.game-card) {
    flex: 1 0 auto;
  }

  @media (max-width: 720px) {
    .grid {
      --card-width: var(--card-width-sm);
      --card-height: var(--card-height-sm);
      --card-title-height: var(--card-title-height-sm);
      --card-info-height: var(--card-info-height-sm);
      grid-template-columns: repeat(auto-fill, minmax(var(--card-width), 1fr));
      gap: var(--space-4);
    }

    .horizontal {
      gap: var(--space-4);
    }
  }
</style>
