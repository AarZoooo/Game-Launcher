<script lang="ts">
  import { goto } from '$app/navigation';
  import Button from '$lib/components/common/Button.svelte';
  import GameGrid from '$lib/components/game/GameGrid.svelte';
  import { launchGame } from '$lib/services/tauriService';
  import type { Game } from '$lib/stores/libraryStore';

  export let game: Game;
  export let similarGames: Game[] = [];

  let launchError = '';

  async function play() {
    launchError = '';

    if (!game.path) {
      goto('/games');
      return;
    }

    try {
      await launchGame(game.path);
    } catch (error) {
      launchError = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<section class={`details ${game.accent}`} style={`--hero-image: url('${game.hero || game.cover}')`}>
  <div class="hero">
    <div class="hero-top">
      <button class="back" on:click={() => goto('/')}>?</button>
      <div class="window-actions">
        <span></span><span></span><span></span>
      </div>
    </div>

    <div class="hero-copy">
      <h1>{game.title}</h1>
      <Button accent={game.accent} wide on:click={play}>Play</Button>
      {#if launchError}
        <p class="error">{launchError}</p>
      {/if}

      <div class="meta-row">
        <div><span>Genres</span><p>{game.genres}</p></div>
        <div><span>Rating</span><p>{game.rating}/10</p></div>
        <div><span>Co-Op Support</span><p>{game.coop}</p></div>
        <div><span>Completion Time</span><p>{game.completion}</p></div>
      </div>
    </div>
  </div>

  <section class="similar">
    <h2>Similar Titles</h2>
    <GameGrid games={similarGames} horizontal compact />
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
    border-radius: 1.2rem;
    overflow: hidden;
    background:
      linear-gradient(180deg, rgba(6, 6, 8, 0.06) 0%, rgba(48, 48, 55, 0.72) 72%),
      var(--hero-image) center / cover no-repeat;
    box-shadow: inset 0 1px rgba(255, 255, 255, 0.08);
  }

  .hero::after {
    content: '';
    position: absolute;
    inset: 0;
    background: linear-gradient(180deg, rgba(15, 16, 20, 0) 8%, rgba(58, 59, 65, 0.95) 100%);
  }

  .hero-top,
  .hero-copy {
    position: relative;
    z-index: 1;
  }

  .hero-top {
    display: flex;
    justify-content: space-between;
    padding: 1rem;
  }

  .back {
    width: 1.9rem;
    height: 1.9rem;
    border: 0;
    background: rgba(72, 72, 80, 0.72);
    color: #f7f5fa;
    cursor: pointer;
  }

  .window-actions {
    display: flex;
    gap: 0.55rem;
  }

  .window-actions span {
    width: 0.65rem;
    height: 0.65rem;
    border-radius: 999px;
    background: rgba(241, 239, 244, 0.55);
  }

  .hero-copy {
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 1rem;
    min-height: 22rem;
    padding: 0 1.4rem 1.6rem;
  }

  h1,
  h2 {
    margin: 0;
  }

  h1 {
    font: 700 clamp(1.9rem, 3vw, 2.45rem) / 1.05 'Bahnschrift', 'Segoe UI Variable Text', sans-serif;
  }

  .error {
    margin: 0;
    color: #ffc2b8;
    font-size: 0.78rem;
  }

  .meta-row {
    display: flex;
    flex-wrap: wrap;
    gap: 1.6rem;
  }

  .meta-row span {
    display: block;
    margin-bottom: 0.35rem;
    color: rgba(227, 223, 232, 0.52);
    font-size: 0.74rem;
  }

  .meta-row p {
    margin: 0;
    color: #f3f1f6;
    font-size: 0.8rem;
    font-weight: 600;
  }

  .similar {
    display: flex;
    flex-direction: column;
    gap: 1rem;
  }
</style>
