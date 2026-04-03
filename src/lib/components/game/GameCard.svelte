<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import { page } from "$app/stores";
import Loader from "$lib/components/common/Loader.svelte";
import GameMenu from "$lib/components/game/GameMenu.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";
import type { GameMenuContext, MenuPlacement } from "$lib/types/Menu";
import { resolveAccentPresentation } from "$lib/utils/accent";
import { getGameImage } from "$lib/utils/getGameMedia";

const dispatch = createEventDispatcher<{
	action: { id: string; game: Game };
}>();

export let game: Game;
export let compact = false;
export let context: GameMenuContext = "library";
export let menuPlacement: MenuPlacement = "above-right";

let cardElement: HTMLElement;

$: accentPresentation = resolveAccentPresentation(game);
$: compactMeta = game.genreLabel || game.genres;

$: if (cardElement) {
	cardElement.style.setProperty("--card-accent-rgb", accentPresentation.rgb);
}

function openGame() {
	const from = `${$page.url.pathname}${$page.url.search}${$page.url.hash}`;
	goto(`/game/${game.id}?from=${encodeURIComponent(from)}`);
}
</script>

<article bind:this={cardElement} class:compact class="game-card">
  <div class="game-card-menu-shell">
    {#if game.favorite}
      <span class="game-card-favorite" aria-label={pageLabels.common.favorite}>★</span>
    {/if}

    <GameMenu
      {game}
      {context}
      placement={menuPlacement}
      on:action={(event) => dispatch('action', event.detail)}
    />
  </div>

  <button
    type="button"
    class="game-card-poster"
    aria-label={pageLabels.actions.openGame(game.title)}
    on:click={openGame}
  >
    <img
      class="game-card-image"
      src={getGameImage(game, 'vertical')}
      alt=""
      loading="lazy"
    />

    {#if game.mediaLoading}
      <div class="game-card-loader">
        <Loader loading inline size="sm" message="" />
      </div>
    {/if}
  </button>

  <div class="game-card-info">
    <div class="game-card-title-row">
      <div class="game-card-copy">
        <h3 class="game-card-title">{game.title}</h3>
        <p class="game-card-meta">
          {compact ? compactMeta : `${game.hours} • ${game.platform}`}
        </p>
      </div>
    </div>
  </div>
</article>

<style>
  .game-card-poster {
    position: relative;
    overflow: hidden;
  }

  .game-card-loader {
    position: absolute;
    inset: 0;
    display: grid;
    place-items: center;
    background:
      linear-gradient(180deg, rgb(7 10 14 / 0.06), rgb(7 10 14 / 0.44)),
      linear-gradient(90deg, rgb(var(--card-accent-rgb) / 0.09), transparent 62%);
    pointer-events: none;
  }

  .game-card-loader :global(.loader-shell) {
    padding: 0.6rem;
    border-radius: 999px;
    background: rgb(9 13 17 / 0.58);
    backdrop-filter: blur(4px);
  }
</style>
