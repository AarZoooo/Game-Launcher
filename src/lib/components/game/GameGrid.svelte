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
    grid-template-columns: repeat(auto-fill, minmax(12rem, 1fr));
    gap: 1.7rem;
  }

  .horizontal {
    display: flex;
    gap: 1.85rem;
    overflow-x: auto;
    scrollbar-width: none;
    padding-bottom: 0.25rem;
  }

  .horizontal::-webkit-scrollbar {
    display: none;
  }

  .horizontal .item {
    min-width: 11.6rem;
  }
</style>
