<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import { appIcons } from "$lib/assets";
import Icon from "$lib/components/common/Icon.svelte";
import GamePlayButton from "$lib/components/game/GamePlayButton.svelte";
import { pageLabels } from "$lib/data/labels";
import { games } from "$lib/stores/libraryStore";
import { effectiveUIMode } from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";
import { resolveAccentPresentation } from "$lib/utils/accent";
import type { GameMediaType } from "$lib/utils/getGameMedia";
import { getGameImage } from "$lib/utils/getGameMedia";

const dispatch = createEventDispatcher<{ launcherror: string }>();

export let game: Game;
export let imageType: GameMediaType = "horizontal";
export let eyebrow: string | undefined = undefined;
export let showBackButton = false;
export let backHref = "/";
export let showFavorite = false;
export let showPlayButton = true;

let heroElement: HTMLElement;

$: accentPresentation = resolveAccentPresentation(game);
$: heroMetrics = [
	{
		label: "Played Today",
		value:
			game.metrics?.find((metric) => metric.label === "Played Today")?.value ||
			"0m",
	},
	{ label: "Total Play", value: game.totalPlaytime || game.hours },
	{ label: pageLabels.game.genres, value: game.genres },
	{ label: pageLabels.game.rating, value: `${game.rating}/10` },
	{ label: pageLabels.game.coopSupport, value: game.coop },
	{ label: pageLabels.game.completionTime, value: game.completion },
];

$: if (heroElement) {
	heroElement.style.setProperty("--hero-accent-rgb", accentPresentation.rgb);
}

function toggleFavorite() {
	void games.toggleFavorite(game.id);
}

let launchError = "";
</script>

<section bind:this={heroElement} class="hero" class:performance={$effectiveUIMode === 'gaming'}>
  <img class="hero-media" src={getGameImage(game, imageType)} alt="" loading="lazy" />

  {#if showBackButton}
    <div class="hero-top">
      <button class="back glass-surface" aria-label={pageLabels.common.goBack} on:click={() => goto(backHref)}>
        <Icon src={appIcons.ui.back} size="1.1rem" />
      </button>
    </div>
  {/if}

  <div class="content">
    {#if eyebrow}
      <p class="eyebrow">{eyebrow}</p>
    {/if}
    <h1>{game.title}</h1>

    <div class="actions">
      {#if showPlayButton}
        <GamePlayButton
          {game}
          compact
          on:launcherror={(event) => {
            launchError = event.detail;
            dispatch("launcherror", event.detail);
          }}
        />
      {/if}

      {#if showFavorite}
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
      {/if}

      <div class="meta">
        {#each heroMetrics as metric}
          <div class="metric">
            <span>{metric.label}</span>
            <p>{metric.value}</p>
          </div>
        {/each}
      </div>
    </div>

    {#if showPlayButton && launchError}
      <p class="error">{launchError}</p>
    {/if}
  </div>
</section>

<slot />

<style>
  .hero {
    position: relative;
    min-height: 25rem;
    border-radius: 0;
    overflow: hidden;
    box-shadow: var(--shadow-inset);
    transition: transform var(--motion-base) ease, box-shadow var(--motion-base) ease;
    margin-block-start: calc(var(--page-padding-y) * -1);
    margin-inline: calc(var(--page-padding-x) * -1);
  }

  .hero-media {
    position: absolute;
    inset: 0;
    width: 100%;
    height: 100%;
    object-fit: cover;
    mask-image: linear-gradient(180deg, black 50%, transparent 100%);
    -webkit-mask-image: linear-gradient(180deg, black 50%, transparent 100%);
  }

  .hero.performance {
    transition: none;
  }

  .hero-top {
    position: relative;
    z-index: 2;
    display: flex;
    justify-content: space-between;
    padding: var(--space-4);
  }

  .back {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: var(--control-height-sm);
    height: var(--control-height-sm);
    padding: 0;
    cursor: pointer;
    border-radius: var(--radius-control-sm);
  }

  .content {
    position: absolute;
    inset: 0;
    z-index: 1;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: var(--space-4);
    padding: var(--space-9) var(--page-padding-x) var(--space-6);
  }

  .eyebrow {
    margin: 0;
    color: var(--text-secondary);
    font-size: var(--font-size-control-sm);
    font-weight: 600;
  }

  h1 {
    margin: 0;
    font: 700 var(--font-size-display) / 1.15 var(--font-display);
    color: var(--text-primary);
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 0;
  }

  .hero-favorite {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    flex-shrink: 0;
    width: var(--control-height-sm);
    height: var(--control-height-sm);
    margin-inline: var(--space-2);
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
    background: rgb(var(--hero-accent-rgb));
    border-color: rgb(var(--hero-accent-rgb));
    color: var(--button-accent-text, #fff);
    box-shadow: var(--shadow-sm);
  }

  .hero-favorite span {
    font-size: var(--font-size-subheading);
    line-height: 1;
  }

  .meta {
    display: flex;
    flex: 1 1 0;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-6);
    padding-inline: var(--space-6);
    min-width: 0;
    overflow-x: auto;
    overflow-y: hidden;
    scrollbar-width: none;
    mask-image: linear-gradient(
      to right,
      transparent 0%,
      black var(--space-6),
      black calc(100% - var(--space-6)),
      transparent 100%
    );
    -webkit-mask-image: linear-gradient(
      to right,
      transparent 0%,
      black var(--space-6),
      black calc(100% - var(--space-6)),
      transparent 100%
    );
  }

  .meta::-webkit-scrollbar {
    display: none;
  }

  .metric {
    min-width: 0;
    flex: 0 0 auto;
  }

  .metric span {
    display: block;
    margin-bottom: var(--space-1);
    color: var(--text-muted);
    font-size: var(--font-size-caption);
  }

  .metric p {
    margin: 0;
    color: var(--text-primary);
    font-size: var(--font-size-caption);
    font-weight: 600;
  }

  .error {
    margin: 0;
    color: var(--color-danger-1);
    font-size: var(--font-size-control-sm);
  }

  @media (max-width: 900px) {
    .hero {
      min-height: 21rem;
    }

    .content {
      padding: var(--space-6) var(--page-padding-x);
    }

    .meta {
      flex: 1 1 0;
    }
  }

  @media (min-width: 921px) {
    .hero {
      margin-block-start: calc(var(--page-padding-y) * -1);
      margin-inline-start: calc((var(--shell-sidebar-width, 0rem) + var(--page-padding-x)) * -1);
      margin-inline-end: calc(var(--page-padding-x) * -1);
      border-radius: 0 0 var(--radius-banner) 0;
      min-height: clamp(26rem, 42vw, 32rem);
    }


    .content {
      padding:
        calc(var(--page-padding-y) + var(--space-10))
        calc(var(--page-padding-x))
        var(--space-6)
        calc(var(--shell-sidebar-width, 0rem) + var(--page-padding-x));
    }

    .hero-top {
      padding:
        calc(var(--page-padding-y) + var(--space-5))
        calc(var(--page-padding-x) + var(--space-4))
        0
        calc(var(--shell-sidebar-width, 0rem) + var(--page-padding-x) + var(--space-4));
    }

    h1 {
      font-size: var(--font-size-display-lg);
    }

    .actions {
      align-items: center;
      gap: var(--space-5);
    }

    .meta {
      gap: var(--space-5);
    }
  }
</style>
