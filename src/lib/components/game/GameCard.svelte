<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import GameMenu from "$lib/components/game/GameMenu.svelte";
import type { Game } from "$lib/stores/libraryStore";

type GameMenuContext = "library" | "explore" | "home";

const dispatch = createEventDispatcher<{
	action: { id: string; game: Game };
}>();

export let game: Game;
export let compact = false;
export let context: GameMenuContext = "library";

function openGame() {
	goto(`/game/${game.id}`);
}
</script>

<article class:compact class={`card ${game.accent}`} style={`--accent:${game.accentHex || '#b69b57'}`}>
  <div class="menu-shell">
    <GameMenu
      {game}
      {context}
      placement="side-right"
      on:action={(event) => dispatch('action', event.detail)}
    />
  </div>

  <button type="button" class="poster" aria-label={`Open ${game.title}`} on:click={openGame}>
    <img class="image" src={game.cover} alt="" loading="lazy" />
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
</article>

<style>
  .card {
    position: relative;
    z-index: 0;
    width: 100%;
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
    min-width: 0;
    background: transparent;
    border-radius: 1rem;
    transition:
      transform var(--motion-base) ease,
      opacity var(--motion-base) ease;
  }

  .card:hover {
    transform: translateY(-0.24rem);
  }

  .card:hover,
  .card:focus-within {
    z-index: 24;
  }

  .menu-shell {
    position: absolute;
    right: 0.75rem;
    bottom: 0.9rem;
    z-index: 12;
  }

  .card:hover .menu-shell :global(.menu-trigger),
  .card:focus-within .menu-shell :global(.menu-trigger) {
    opacity: 0.92;
    transform: scale(1);
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
    display: block;
    width: 100%;
    aspect-ratio: 0.63;
    object-fit: cover;
    background-color: #33343a;
    border-radius: 1rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow:
      inset 0 -1.2rem 2rem color-mix(in srgb, var(--accent) 18%, transparent),
      0 0.9rem 2rem rgba(0, 0, 0, 0.16);
  }

  .info {
    min-height: 3.9rem;
    padding: 0 0.15rem 0 0.1rem;
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
    display: -webkit-box;
    overflow: hidden;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  p {
    margin: 0.42rem 0 0;
    color: rgba(223, 220, 230, 0.56);
    font-size: 0.76rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .favorite {
    color: var(--accent);
    font-size: 0.9rem;
    line-height: 1;
  }

  .compact .info {
    min-height: 3.55rem;
  }
</style>
