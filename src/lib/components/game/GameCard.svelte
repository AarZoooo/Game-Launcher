<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import { page } from "$app/stores";
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
  </button>

  <div class="game-card-info">
    <div class="game-card-title-row">
      <div class="game-card-copy">
        <h3 class="game-card-title">{game.title}</h3>
        <p class="game-card-meta">
          {compact ? game.genreLabel || game.genres : `${game.hours} • ${game.platform}`}
        </p>
      </div>
    </div>
  </div>
</article>
