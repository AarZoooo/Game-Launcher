<script lang="ts">
import { onMount } from "svelte";
import { browser } from "$app/environment";
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
    <section class="content-section">
      <div class="section-header compact">
        <div class="section-title">
          <span class="ai-sparkle" aria-hidden="true">
            <svg viewBox="0 0 44 32" focusable="false">
              <path d="M14 1c1.1 5.7 2.55 8.93 4.95 11.33C21.35 14.73 24.58 16.18 30.28 17.28c-5.7 1.1-8.93 2.55-11.33 4.95C16.55 24.63 15.1 27.86 14 33.56c-1.1-5.7-2.55-8.93-4.95-11.33-2.4-2.4-5.63-3.85-11.33-4.95 5.7-1.1 8.93-2.55 11.33-4.95C11.45 9.93 12.9 6.7 14 1Z" transform="translate(2 -1.5) scale(0.9 0.82)" />
              <path d="M31.3 13.1c.62 3.18 1.45 4.95 2.83 6.33 1.38 1.38 3.15 2.21 6.33 2.83-3.18.62-4.95 1.45-6.33 2.83-1.38 1.38-2.21 3.15-2.83 6.33-.62-3.18-1.45-4.95-2.83-6.33-1.38-1.38-3.15-2.21-6.33-2.83 3.18-.62 4.95-1.45 6.33-2.83 1.38-1.38 2.21-3.15 2.83-6.33Z" transform="translate(0 -1)" />
            </svg>
          </span>
          <h2>{pageLabels.explore.suggestedGames}</h2>
        </div>
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

    <section class="content-section">
      <div class="section-title">
        <span class="ai-sparkle" aria-hidden="true">
          <svg viewBox="0 0 44 32" focusable="false">
            <path d="M14 1c1.1 5.7 2.55 8.93 4.95 11.33C21.35 14.73 24.58 16.18 30.28 17.28c-5.7 1.1-8.93 2.55-11.33 4.95C16.55 24.63 15.1 27.86 14 33.56c-1.1-5.7-2.55-8.93-4.95-11.33-2.4-2.4-5.63-3.85-11.33-4.95 5.7-1.1 8.93-2.55 11.33-4.95C11.45 9.93 12.9 6.7 14 1Z" transform="translate(2 -1.5) scale(0.9 0.82)" />
            <path d="M31.3 13.1c.62 3.18 1.45 4.95 2.83 6.33 1.38 1.38 3.15 2.21 6.33 2.83-3.18.62-4.95 1.45-6.33 2.83-1.38 1.38-2.21 3.15-2.83 6.33-.62-3.18-1.45-4.95-2.83-6.33-1.38-1.38-3.15-2.21-6.33-2.83 3.18-.62 4.95-1.45 6.33-2.83 1.38-1.38 2.21-3.15 2.83-6.33Z" transform="translate(0 -1)" />
          </svg>
        </span>
        <h3>{pageLabels.explore.suggestedGames}</h3>
      </div>
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

  .content-section {
    display: flex;
    flex-direction: column;
    gap: clamp(0.85rem, 1.5vw, 1.2rem);
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: baseline;
  }

  .section-title {
    display: inline-flex;
    align-items: center;
    gap: 0.5rem;
    min-width: 0;
  }

  .compact {
    justify-content: flex-start;
    gap: 0.55rem;
  }

  h2,
  h3 {
    margin: 0;
    font-size: var(--font-size-body);
  }

  .ai-sparkle {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    width: 1.45rem;
    height: 1.05rem;
    color: color-mix(in srgb, var(--text-primary) 84%, var(--interactive-primary-bg));
    filter: drop-shadow(0 0 0.35rem rgb(255 255 255 / 0.14));
    flex: 0 0 auto;
  }

  .ai-sparkle svg {
    width: 100%;
    height: 100%;
    fill: currentColor;
  }

  span,
  p {
    color: var(--text-muted);
    font-size: var(--font-size-control-sm);
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
    border-radius: var(--radius-panel);
    background: var(--surface-glass);
    box-shadow: var(--shadow-accent-outline-soft);
    backdrop-filter: blur(var(--blur-md));
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
    border-radius: var(--radius-control-md);
    background: var(--interactive-primary-bg);
    color: var(--interactive-primary-text);
    font-weight: 700;
  }

  .offline {
    min-height: 62vh;
    display: grid;
    place-items: center;
  }
</style>
