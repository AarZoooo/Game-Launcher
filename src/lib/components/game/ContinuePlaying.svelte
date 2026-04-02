<script lang="ts">
import GamePlayButton from "$lib/components/game/GamePlayButton.svelte";
import { pageLabels } from "$lib/data/labels";
import { games } from "$lib/stores/libraryStore";
import { effectiveUIMode } from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";
import { resolveAccentPresentation } from "$lib/utils/accent";
import { getGameImage } from "$lib/utils/getGameMedia";

export let game: Game;

let heroElement: HTMLElement;

$: accentPresentation = resolveAccentPresentation(game);

$: if (heroElement) {
	heroElement.style.setProperty("--hero-accent-rgb", accentPresentation.rgb);
}

function toggleFavorite() {
	void games.toggleFavorite(game.id);
}
</script>

<section bind:this={heroElement} class="hero" class:performance={$effectiveUIMode === 'gaming'}>
  <img class="hero-media" src={getGameImage(game, 'horizontal')} alt="" loading="lazy" />
  <div class="veil"></div>
  <button
    type="button"
    class:active={game.favorite}
    class="hero-favorite"
    aria-label={game.favorite ? pageLabels.actions.removeFavorite : pageLabels.actions.addFavorite}
    aria-pressed={game.favorite}
    on:click={toggleFavorite}
  >
    <span aria-hidden="true">{game.favorite ? "★" : "☆"}</span>
  </button>
  <div class="content">
    <p class="eyebrow">{pageLabels.continuePlaying.eyebrow}</p>
    <h1>{game.title}</h1>

    <div class="actions">
      <GamePlayButton {game} compact />

      <div class="meta">
        {#each (game.metrics || []).filter((metric) => metric.label !== 'Cloud Sync') as metric}
          <div class="metric">
            <span>{metric.label}</span>
            <p>{metric.value}</p>
          </div>
        {/each}
      </div>
    </div>
  </div>
</section>

<style>
  .hero {
    position: relative;
    min-height: 25rem;
    border-radius: var(--radius-banner);
    overflow: hidden;
    box-shadow: var(--shadow-inset);
    transition: transform var(--motion-base) ease, box-shadow var(--motion-base) ease;
  }

  .hero-media {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .veil {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(90deg, var(--surface-glass-strong) 0%, var(--surface-glass) 45%, transparent 100%),
      radial-gradient(circle at 50% 0%, rgb(var(--hero-accent-rgb) / 0.18) 0%, transparent 48%);
    backdrop-filter: blur(calc(var(--ui-blur) * 0.08));
  }

  .hero.performance,
  .hero.performance .veil {
    transition: none;
    backdrop-filter: none;
  }

  .content {
    position: absolute;
    inset: 0;
    z-index: 1;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: var(--space-4);
    padding: var(--space-9) var(--space-8) var(--space-7);
  }

  .hero-favorite {
    position: absolute;
    right: var(--space-6);
    top: var(--space-6);
    z-index: var(--z-menu);
    display: inline-grid;
    place-items: center;
    width: 2.25rem;
    height: 2.25rem;
    border: 1px solid rgb(255 255 255 / 0.12);
    border-radius: var(--radius-pill-ui);
    background: var(--surface-glass);
    color: var(--text-secondary);
    box-shadow: var(--shadow-outline);
    cursor: pointer;
    transition:
      transform var(--motion-fast) ease,
      color var(--motion-fast) ease,
      background-color var(--motion-fast) ease,
      border-color var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease;
  }

  .hero-favorite:hover {
    transform: translateY(-1px) scale(1.03);
    color: var(--text-primary);
    background: var(--surface-hover);
  }

  .hero-favorite.active {
    color: rgb(var(--hero-accent-rgb));
    border-color: rgb(var(--hero-accent-rgb) / 0.28);
    box-shadow:
      var(--shadow-outline),
      0 0 1rem rgb(var(--hero-accent-rgb) / 0.18);
  }

  .hero-favorite span {
    font-size: 1rem;
    line-height: 1;
  }

  .eyebrow {
    margin: 0;
    color: var(--text-secondary);
    font-size: 0.78rem;
    font-weight: 600;
  }

  h1 {
    margin: 0;
    max-width: 24rem;
    font: 700 clamp(1.9rem, 3vw, 2.45rem) / 1.05 var(--font-display);
    color: var(--text-primary);
  }

  .actions {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--space-3);
  }

  .meta {
    display: flex;
    flex: 1 1 24rem;
    flex-wrap: wrap;
    align-items: center;
    gap: var(--space-3);
    min-width: 0;
  }

  .metric {
    min-width: 0;
    flex: 1 1 9rem;
  }

  .metric span {
    display: block;
    margin-bottom: var(--space-1);
    color: var(--text-muted);
    font-size: 0.72rem;
  }

  .metric p {
    margin: 0;
    color: var(--text-primary);
    font-size: 0.74rem;
    font-weight: 600;
  }

  @media (max-width: 900px) {
    .hero {
      min-height: 21rem;
    }

    .content {
      padding: var(--space-6);
    }

    .meta {
      flex-basis: 100%;
    }
  }
</style>

