<script lang="ts">
  import { browser } from '$app/environment';
  import { onMount } from 'svelte';
  import Button from '$lib/components/common/Button.svelte';
  import GameGrid from '$lib/components/game/GameGrid.svelte';
  import {
    explorePrimaryIds,
    exploreSecondaryIds,
    games,
    getGamesByIds,
    type Game
  } from '$lib/stores/libraryStore';
  import { recommendationPrompt } from '$lib/utils/constants';

  const primary = getGamesByIds(explorePrimaryIds);
  const secondary = getGamesByIds(exploreSecondaryIds);
  let isOnline = true;
  let prompt = recommendationPrompt;

  function handleAction(event: CustomEvent<{ id: string; game: Game }>) {
    const { id, game } = event.detail;

    if (id === 'status-want') return games.setStatus(game.id, 'want');
    if (id === 'status-playing') return games.setStatus(game.id, 'playing');
    if (id === 'status-played') return games.setStatus(game.id, 'played');
  }

  onMount(() => {
    if (!browser) return;
    isOnline = navigator.onLine;

    const onOnline = () => (isOnline = true);
    const onOffline = () => (isOnline = false);

    window.addEventListener('online', onOnline);
    window.addEventListener('offline', onOffline);

    return () => {
      window.removeEventListener('online', onOnline);
      window.removeEventListener('offline', onOffline);
    };
  });
</script>

{#if isOnline}
  <div class="explore">
    <section>
      <div class="section-header compact">
        <h2>? Suggested Games</h2>
        <span>based on what you play</span>
      </div>
      <GameGrid games={primary} compact context="explore" on:action={handleAction} />
      <div class="actions"><Button quiet>Refresh</Button></div>
    </section>

    <section class="assistant">
      <div class="section-header compact">
        <h2>Recommendation Assistant</h2>
        <span>just tell us what you want</span>
      </div>

      <div class="prompt-box">
        <input bind:value={prompt} />
        <button>?</button>
      </div>

      <p>Suggestions will show up below</p>
    </section>

    <section>
      <h3>Suggested Games</h3>
      <GameGrid games={secondary} compact context="explore" on:action={handleAction} />
      <div class="actions"><Button quiet>Refresh</Button></div>
    </section>
  </div>
{:else}
  <section class="offline">
    <div class="icon">?</div>
    <h2>It seems that you're offline.</h2>
    <p>Come back when you have an active internet connection.</p>
  </section>
{/if}

<style>
  .explore {
    display: flex;
    flex-direction: column;
    gap: 2rem;
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
    color: rgba(226, 223, 231, 0.42);
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
    gap: 0.9rem;
  }

  .prompt-box {
    display: grid;
    grid-template-columns: 1fr auto;
    gap: 0.7rem;
    padding: 0.28rem;
    border: 1px solid rgba(207, 165, 130, 0.48);
    background: rgba(255, 255, 255, 0.03);
    box-shadow: inset 0 0 0 1px rgba(183, 155, 87, 0.12);
  }

  .prompt-box input {
    border: 0;
    background: transparent;
    color: #f4f2f7;
    padding: 0.85rem;
    font: inherit;
  }

  .prompt-box button {
    width: 2.4rem;
    border: 0;
    background: rgba(255, 255, 255, 0.9);
    color: #404149;
    font-weight: 700;
  }

  .offline {
    min-height: 62vh;
    display: grid;
    place-items: center;
    align-content: center;
    gap: 0.6rem;
    text-align: center;
    color: rgba(232, 229, 237, 0.48);
  }

  .icon {
    font-size: 4rem;
    opacity: 0.35;
  }
</style>
