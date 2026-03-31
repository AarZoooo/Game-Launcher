<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { goto } from '$app/navigation';
  import type { Game } from '$lib/stores/libraryStore';
  import GameMenu from '$lib/components/game/GameMenu.svelte';

  type GameMenuContext = 'library' | 'explore' | 'home';

  const dispatch = createEventDispatcher<{
    action: { id: string; game: Game };
  }>();

  export let game: Game;
  export let compact = false;
  export let context: GameMenuContext = 'library';

  function openGame() {
    goto(`/game/${game.id}`);
  }
</script>

<article class:compact class={`card ${game.accent}`} style={`--accent:${game.accentHex || '#b69b57'}`}>
  <button type="button" class="poster" aria-label={`Open ${game.title}`} on:click={openGame}>
  <div class="image" style={`background-image: url('${game.cover}')`}></div>
  </button>

  <div class="info">
    <div class="title-row">
      <div class="copy">
        <h3>{game.title}</h3>
        <p>{compact ? game.genreLabel || game.genres : `${game.hours} • ${game.platform}`}</p>
      </div>

      {#if game.favorite}
        <span class="favorite" aria-label="Favorite">★</span>
      {/if}
    </div>
  </div>

  <GameMenu {game} {context} on:action={(event) => dispatch('action', event.detail)} />
</article>

<style>
  .card {
    position: relative;
    width: 100%;
    background: rgba(255, 255, 255, 0.02);
    border-radius: 0.1rem;
    transition:
      transform 180ms ease,
      opacity 180ms ease;
  }

  .card:hover {
    transform: translateY(-0.24rem);
  }

  .poster {
    display: block;
    width: 100%;
    border: 0;
    padding: 0;
    background: transparent;
    cursor: pointer;
    text-align: left;
  }

  .image {
    aspect-ratio: 0.63;
    width: 100%;
    background-color: #33343a;
    background-position: center;
    background-repeat: no-repeat;
    background-size: cover;
    border: 1px solid rgba(255, 255, 255, 0.05);
    box-shadow: inset 0 -1rem 2rem color-mix(in srgb, var(--accent) 18%, transparent);
  }

  .info {
    min-height: 3.55rem;
    padding: 0.85rem 0.15rem 0 0.1rem;
  }

  .title-row {
    display: flex;
    justify-content: space-between;
    gap: 0.7rem;
    align-items: start;
  }

  .copy {
    min-width: 0;
  }

  h3 {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 650;
    color: #efedf3;
    line-height: 1.2;
  }

  p {
    margin: 0.42rem 0 0;
    color: rgba(223, 220, 230, 0.56);
    font-size: 0.76rem;
  }

  .favorite {
    color: var(--accent);
    font-size: 0.9rem;
    line-height: 1;
  }

  .compact .info {
    padding-top: 0.7rem;
  }
</style>
