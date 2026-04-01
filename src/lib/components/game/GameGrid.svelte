<script lang="ts">
import { createEventDispatcher } from "svelte";
import GameCard from "$lib/components/game/GameCard.svelte";
import type { Game } from "$lib/types/Game";
import type { GameMenuContext } from "$lib/types/Menu";

const dispatch = createEventDispatcher<{
	action: { id: string; game: Game };
}>();

export let games: Game[] = [];
export let horizontal = false;
export let compact = false;
export let context: GameMenuContext = "library";
</script>

<div class:horizontal class="grid">
  {#each games as game}
    <div class="item">
      <GameCard {game} {compact} {context} on:action={(event) => dispatch('action', event.detail)} />
    </div>
  {/each}
</div>

<style>
  .item {
    position: relative;
    z-index: 0;
  }

  .item:hover,
  .item:focus-within {
    z-index: 18;
  }

  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12.25rem, 1fr));
    gap: var(--space-6);
    align-items: start;
  }

  .horizontal {
    display: flex;
    gap: var(--space-6);
    overflow-x: auto;
    scrollbar-width: none;
    padding: var(--space-1) 0 var(--space-2);
  }

  .horizontal::-webkit-scrollbar {
    display: none;
  }

  .horizontal .item {
    flex: 0 0 12rem;
    min-width: 12rem;
  }

  @media (max-width: 720px) {
    .grid {
      grid-template-columns: repeat(auto-fill, minmax(10.4rem, 1fr));
      gap: var(--space-4);
    }

    .horizontal {
      gap: var(--space-4);
    }

    .horizontal .item {
      flex-basis: 10.8rem;
      min-width: 10.8rem;
    }
  }
</style>
