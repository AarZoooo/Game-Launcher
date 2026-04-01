<script lang="ts">
import { goto } from "$app/navigation";
import { appIcons, getGameBanner } from "$lib/assets";
import Icon from "$lib/components/common/Icon.svelte";
import GameGrid from "$lib/components/game/GameGrid.svelte";
import GamePlayButton from "$lib/components/game/GamePlayButton.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";

export let game: Game;
export let similarGames: Game[] = [];
export let backHref = "/";

let launchError = "";
$: showPlayButton = game.inLibrary !== false;
</script>

<section class="details">
  <div class="hero">
    <img class="hero-media" src={getGameBanner(game)} alt="" loading="lazy" />
    <div class="hero-top">
      <button class="back" aria-label={pageLabels.common.goBack} on:click={() => goto(backHref)}>
        <Icon src={appIcons.ui.back} size="1rem" />
      </button>
      <div class="window-actions">
        <span></span><span></span><span></span>
      </div>
    </div>

    <div class="hero-copy">
      <h1>{game.title}</h1>
      {#if showPlayButton}
        <div class="actions">
          <GamePlayButton
            {game}
            compact
            on:launcherror={(event) => (launchError = event.detail)}
          />
        </div>
      {/if}
      {#if showPlayButton && launchError}
        <p class="error">{launchError}</p>
      {/if}

      <div class="meta-row">
        <div><span>{pageLabels.game.genres}</span><p>{game.genres}</p></div>
        <div><span>{pageLabels.game.rating}</span><p>{game.rating}/10</p></div>
        <div><span>{pageLabels.game.coopSupport}</span><p>{game.coop}</p></div>
        <div><span>{pageLabels.game.completionTime}</span><p>{game.completion}</p></div>
      </div>
    </div>
  </div>

  <section class="similar">
    <h2>{pageLabels.game.similarTitles}</h2>
    <GameGrid games={similarGames} horizontal compact menuPlacement="above-right" />
  </section>
</section>

<style>
  .details {
    display: flex;
    flex-direction: column;
    gap: 1.8rem;
  }

  .hero {
    position: relative;
    min-height: 25rem;
    border-radius: var(--radius-xl);
    overflow: hidden;
    box-shadow: var(--shadow-inset);
    transition: box-shadow var(--motion-base) ease;
  }

  .hero-media {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
  }

  .hero::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(180deg, transparent 8%, var(--surface-glass-strong) 100%);
  }

  .hero-top,
  .hero-copy {
    position: relative;
    z-index: 1;
  }

  .hero-top {
    display: flex;
    justify-content: space-between;
    padding: var(--space-4);
  }

  .back {
    width: 1.9rem;
    height: 1.9rem;
    border: 0;
    background: var(--surface-glass);
    color: var(--text-primary);
    cursor: pointer;
    border-radius: var(--radius-sm);
    backdrop-filter: blur(var(--blur-md));
  }

  .window-actions {
    display: flex;
    gap: var(--space-2);
  }

  .window-actions span {
    width: 0.65rem;
    height: 0.65rem;
    border-radius: var(--radius-pill);
    background: var(--text-secondary);
  }

  .hero-copy {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: var(--space-4);
    min-height: 22rem;
    padding: 0 var(--space-6) var(--space-7);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: var(--space-3);
  }

  h1,
  h2 {
    margin: 0;
  }

  h1 {
    font: 700 clamp(1.9rem, 3vw, 2.45rem) / 1.05 var(--font-display);
  }

  .error {
    margin: 0;
    color: var(--color-danger-1);
    font-size: 0.78rem;
  }

  .meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: var(--space-6);
  }

  .meta-row span {
    display: block;
    margin-bottom: var(--space-2);
    color: var(--text-muted);
    font-size: 0.74rem;
  }

  .meta-row p {
    margin: 0;
    color: var(--text-primary);
    font-size: 0.8rem;
    font-weight: 600;
  }

  .similar {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }
</style>
