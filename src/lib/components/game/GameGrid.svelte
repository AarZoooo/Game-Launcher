<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import GameCard from '$lib/components/game/GameCard.svelte';
  import type { Game } from '$lib/stores/libraryStore';

  type GameMenuContext = 'library' | 'explore' | 'home';

  const dispatch = createEventDispatcher<{
    action: { id: string; game: Game };
  }>();

  export let games: Game[] = [];
  export let horizontal = false;
  export let compact = false;
  export let context: GameMenuContext = 'library';
</script>

<div class:horizontal class="grid">
  {#each games as game}
    <div class="item">
      <GameCard {game} {compact} {context} on:action={(event) => dispatch('action', event.detail)} />
    </div>
  {/each}
</div>

<style>
  .grid {
    display: grid;
    grid-template-columns: repeat(auto-fill, minmax(12.25rem, 1fr));
    gap: 1.45rem;
    align-items: start;
  }

  .horizontal {
    display: flex;
    gap: 1.45rem;
    overflow-x: auto;
    scrollbar-width: none;
    padding: 0.15rem 0 0.35rem;
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
      gap: 1rem;
    }

    .horizontal {
      gap: 1rem;
    }

    .horizontal .item {
      flex-basis: 10.8rem;
      min-width: 10.8rem;
    }
  }
</style>
