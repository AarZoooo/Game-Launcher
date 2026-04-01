<script lang="ts">
import { onMount } from "svelte";
import { browser } from "$app/environment";
import { getGameBanner } from "$lib/assets";
import Button from "$lib/components/common/Button.svelte";
import EmptyState from "$lib/components/common/EmptyState.svelte";
import GameGrid from "$lib/components/game/GameGrid.svelte";
import { pageLabels, recommendationPrompt } from "$lib/data/labels";
import { performGameAction } from "$lib/services/gameService";
import {
	explorePrimaryIds,
	exploreSecondaryIds,
	getGamesByIds,
} from "$lib/stores/libraryStore";
import type { Game } from "$lib/types/Game";
import type { GameMenuActionId } from "$lib/types/Menu";

const primary = getGamesByIds(explorePrimaryIds);
const featuredRecommendation = primary[0];
const primaryGrid = primary.slice(1);
const secondary = getGamesByIds(exploreSecondaryIds);
let isOnline = true;
let prompt = recommendationPrompt;

function handleAction(event: CustomEvent<{ id: string; game: Game }>) {
	return performGameAction(
		event.detail.id as GameMenuActionId,
		event.detail.game,
	);
}

onMount(() => {
	if (!browser) return;
	isOnline = navigator.onLine;

	const onOnline = () => (isOnline = true);
	const onOffline = () => (isOnline = false);

	window.addEventListener("online", onOnline);
	window.addEventListener("offline", onOffline);

	return () => {
		window.removeEventListener("online", onOnline);
		window.removeEventListener("offline", onOffline);
	};
});
</script>

{#if isOnline}
  <div class="explore">
    {#if featuredRecommendation}
      <section class="featured-banner">
        <div class="banner-media">
          <img src={getGameBanner(featuredRecommendation)} alt="" loading="lazy" />
        </div>

        <div class="banner-copy">
          <p>{pageLabels.explore.recommendedForYou}</p>
          <h1>{featuredRecommendation.title}</h1>
          <span>{featuredRecommendation.genreLabel || featuredRecommendation.genres}</span>
        </div>
      </section>
    {/if}

    <section>
      <div class="section-header compact">
        <h2>{pageLabels.explore.suggestedGames}</h2>
        <span>{pageLabels.explore.basedOnWhatYouPlay}</span>
      </div>
      <GameGrid games={primaryGrid} compact context="explore" on:action={handleAction} />
      <div class="actions"><Button quiet>{pageLabels.explore.refresh}</Button></div>
    </section>

    <section class="assistant">
      <div class="section-header compact">
        <h2>{pageLabels.explore.recommendationAssistant}</h2>
        <span>{pageLabels.explore.assistantHint}</span>
      </div>

      <div class="prompt-box">
        <input class="field-control" bind:value={prompt} />
        <button aria-label="Generate recommendations">{pageLabels.explore.go}</button>
      </div>

      <p>{pageLabels.explore.suggestionsPending}</p>
    </section>

    <section>
      <h3>{pageLabels.explore.suggestedGames}</h3>
      <GameGrid games={secondary} compact context="explore" on:action={handleAction} />
      <div class="actions"><Button quiet>{pageLabels.explore.refresh}</Button></div>
    </section>
  </div>
{:else}
  <section class="offline">
    <EmptyState
      kind="errorState"
      title={pageLabels.explore.offlineTitle}
      message={pageLabels.explore.offlineBody}
    />
  </section>
{/if}

<style>
  .explore {
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .featured-banner {
    position: relative;
    overflow: hidden;
    border-radius: var(--radius-xl);
    border: 1px solid var(--surface-border);
    background: var(--surface-card);
    box-shadow: var(--shadow-inset);
  }

  .banner-media {
    position: relative;
    height: clamp(16.25rem, 32vw, 18.75rem);
  }

  .banner-media::after {
    content: '';
    position: absolute;
    inset: 0;
    background:
      linear-gradient(180deg, transparent 15%, var(--surface-glass-strong) 100%),
      linear-gradient(90deg, transparent 20%, var(--surface-glass) 100%);
  }

  .banner-media img {
    display: block;
    width: 100%;
    height: 100%;
    object-fit: cover;
    border-radius: inherit;
  }

  .banner-copy {
    position: absolute;
    inset: auto 0 0 0;
    z-index: 1;
    display: flex;
    flex-direction: column;
    gap: 0.45rem;
    padding: 1.4rem 1.5rem 1.45rem;
  }

  .banner-copy p,
  .banner-copy span,
  .banner-copy h1 {
    margin: 0;
  }

  .banner-copy p {
    color: var(--text-secondary);
    font-size: 0.76rem;
    letter-spacing: 0.08em;
    text-transform: uppercase;
  }

  .banner-copy h1 {
    font: 700 clamp(1.55rem, 2.6vw, 2.2rem) / 1.05 var(--font-display);
    color: var(--text-primary);
  }

  .banner-copy span {
    color: var(--text-secondary);
    font-size: 0.86rem;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
    margin-bottom: 1rem;
  }

  .compact {
    justify-content: flex-start;
    gap: 0.55rem;
  }

  h2,
  h3 {
    margin: 0;
    font-size: 1rem;
  }

  span,
  p {
    color: var(--text-muted);
    font-size: 0.75rem;
  }

  .actions {
    display: flex;
    justify-content: center;
    margin-top: 1.2rem;
  }

  .assistant {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .prompt-box {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: var(--space-3);
    padding: var(--space-1);
    border: 1px solid var(--surface-border);
    border-radius: var(--radius-lg);
    background: var(--surface-glass);
    box-shadow: inset 0 0 0 1px rgb(var(--accent-rgb) / 0.12);
    backdrop-filter: blur(10px);
  }

  .prompt-box input {
    border: 0;
    background: transparent;
    color: var(--text-primary);
    padding: 0 var(--space-3);
    font: inherit;
    box-shadow: none;
  }

  .prompt-box button {
    min-width: 3.1rem;
    border: 0;
    border-radius: var(--radius-md);
    background: var(--interactive-primary-bg);
    color: var(--interactive-primary-text);
    font-weight: 700;
  }

  .offline {
    min-height: 62vh;
    display: grid;
    place-items: center;
  }

  @media (max-width: 720px) {
    .banner-copy {
      padding: 1.1rem;
    }

    .banner-media {
      height: 16.4rem;
    }
  }
</style>
