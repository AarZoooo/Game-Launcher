<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import GameMenu from "$lib/components/game/GameMenu.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";
import type { GameMenuContext } from "$lib/types/Menu";
import { resolveAccentPresentation } from "$lib/utils/accent";

const dispatch = createEventDispatcher<{
	action: { id: string; game: Game };
}>();

export let game: Game;
export let compact = false;
export let context: GameMenuContext = "library";

let cardElement: HTMLElement;

$: accentPresentation = resolveAccentPresentation(game);

$: if (cardElement) {
	cardElement.style.setProperty("--card-accent-rgb", accentPresentation.rgb);
}

function openGame() {
	goto(`/game/${game.id}`);
}
</script>

<article bind:this={cardElement} class:compact class="card">
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
        <span class="favorite" aria-label={pageLabels.common.favorite}>★</span>
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
    gap: var(--space-3);
    min-width: 0;
    padding: var(--space-3);
    background:
      linear-gradient(180deg, rgb(var(--card-accent-rgb) / 0.08), transparent 38%),
      var(--surface-card);
    border: 1px solid var(--surface-border-soft);
    border-radius: var(--radius-lg);
    box-shadow: var(--shadow-inset);
    transition:
      transform var(--motion-base) ease,
      opacity var(--motion-base) ease,
      box-shadow var(--motion-base) ease,
      border-color var(--motion-base) ease;
  }

  .card:hover {
    transform: translateY(-0.24rem);
    box-shadow: 0 1rem 2rem rgb(var(--card-accent-rgb) / 0.18);
    border-color: rgb(var(--card-accent-rgb) / 0.22);
  }

  .card:hover,
  .card:focus-within {
    z-index: 24;
  }

  .menu-shell {
    position: absolute;
    top: var(--space-3);
    right: var(--space-3);
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
    height: clamp(15rem, 30vw, 17.5rem);
    object-fit: cover;
    background-color: var(--color-background-4);
    border-radius: calc(var(--radius-lg) - var(--space-1));
    border: 1px solid var(--surface-border-soft);
    box-shadow:
      inset 0 -1.2rem 2rem rgb(var(--card-accent-rgb) / 0.2),
      var(--shadow-sm);
  }

  .info {
    min-height: 3.9rem;
    padding: 0 var(--space-1);
  }

  .title-row {
    display: flex;
    justify-content: space-between;
    gap: var(--space-3);
    align-items: start;
  }

  .copy {
    min-width: 0;
  }

  h3 {
    margin: 0;
    font-size: 0.95rem;
    font-weight: 650;
    color: var(--text-primary);
    line-height: 1.2;
    display: -webkit-box;
    overflow: hidden;
    line-clamp: 2;
    -webkit-line-clamp: 2;
    -webkit-box-orient: vertical;
  }

  p {
    margin: var(--space-2) 0 0;
    color: var(--text-secondary);
    font-size: 0.76rem;
    white-space: nowrap;
    overflow: hidden;
    text-overflow: ellipsis;
  }

  .favorite {
    color: rgb(var(--card-accent-rgb));
    font-size: 0.9rem;
    line-height: 1;
  }

  .compact .info {
    min-height: 3.55rem;
  }
</style>
