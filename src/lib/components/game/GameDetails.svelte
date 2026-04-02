<script lang="ts">
import { goto } from "$app/navigation";
import { appIcons } from "$lib/assets";
import Icon from "$lib/components/common/Icon.svelte";
import GameGrid from "$lib/components/game/GameGrid.svelte";
import GamePlayButton from "$lib/components/game/GamePlayButton.svelte";
import { pageLabels } from "$lib/data/labels";
import { effectiveUIMode } from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";
import { resolveAccentPresentation } from "$lib/utils/accent";
import { getGameImage } from "$lib/utils/getGameMedia";

export let game: Game;
export let similarGames: Game[] = [];
export let backHref = "/";

let launchError = "";
let heroElement: HTMLElement;
$: showPlayButton = game.inLibrary !== false;
$: accentPresentation = resolveAccentPresentation(game);
$: detailMetrics = [
	{ label: "Last Play", value: game.lastPlayed || "Never" },
	{ label: "Total Play", value: game.totalPlaytime || game.hours },
	{ label: pageLabels.game.genres, value: game.genres },
	{ label: pageLabels.game.rating, value: `${game.rating}/10` },
	{ label: pageLabels.game.coopSupport, value: game.coop },
	{ label: pageLabels.game.completionTime, value: game.completion },
];

$: if (heroElement) {
	heroElement.style.setProperty("--details-accent-rgb", accentPresentation.rgb);
}
</script>

<section class="details">
  <div bind:this={heroElement} class="hero" class:performance={$effectiveUIMode === 'gaming'}>
    <img class="hero-media" src={getGameImage(game, 'banner')} alt="" loading="lazy" />
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
      <div class="actions">
        {#if showPlayButton}
          <GamePlayButton
            {game}
            compact
            on:launcherror={(event) => (launchError = event.detail)}
          />
        {/if}

        <div class="meta-row">
          {#each detailMetrics as item}
            <div>
              <span>{item.label}</span>
              <p>{item.value}</p>
            </div>
          {/each}
        </div>
      </div>

      {#if showPlayButton && launchError}
        <p class="error">{launchError}</p>
      {/if}
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
    border-radius: var(--radius-banner);
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
    background:
      linear-gradient(180deg, transparent 8%, var(--surface-glass-strong) 100%),
      radial-gradient(circle at 80% 12%, rgb(var(--details-accent-rgb) / 0.18), transparent 34%);
  }

  .hero.performance {
    transition: none;
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
    border-radius: var(--radius-control-sm);
    backdrop-filter: blur(var(--blur-md));
  }

  .window-actions {
    display: flex;
    gap: var(--space-2);
  }

  .window-actions span {
    width: 0.65rem;
    height: 0.65rem;
    border-radius: var(--radius-round);
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
    flex-wrap: wrap;
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
    flex: 1 1 24rem;
    flex-wrap: wrap;
    gap: var(--space-3);
    min-width: 0;
  }

  .meta-row > div {
    min-width: 0;
    flex: 1 1 9rem;
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

  @media (max-width: 900px) {
    .meta-row {
      flex-basis: 100%;
    }
  }
</style>
