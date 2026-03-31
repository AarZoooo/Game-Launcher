<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { goto } from '$app/navigation';
  import Button from '$lib/components/common/Button.svelte';
  import GameMenu from '$lib/components/game/GameMenu.svelte';
  import SyncBadge from '$lib/components/sync/SyncBadge.svelte';
  import type { Game } from '$lib/stores/libraryStore';

  const dispatch = createEventDispatcher<{
    action: { id: string; game: Game };
  }>();

  export let game: Game;
</script>

<section class="hero" style={`--hero-image: url('${game.hero || game.cover}')`}>
  <div class="veil"></div>
  <div class="content">
    <p class="eyebrow">Continue playing</p>
    <h1>{game.title}</h1>

    <div class="actions">
      <Button accent={game.accent} compact iconFirst on:click={() => goto(`/game/${game.id}`)}>
        <span>▶</span>
        <span>Play</span>
      </Button>
      <GameMenu {game} context="home" on:action={(event) => dispatch('action', event.detail)} />
    </div>

    <div class="meta">
      {#each game.metrics || [] as metric}
        <div class="metric">
          <span>{metric.label}</span>
          {#if metric.label === 'Cloud Sync'}
            <SyncBadge status={metric.value} />
          {:else}
            <p>{metric.value}</p>
          {/if}
        </div>
      {/each}
    </div>
  </div>
</section>

<style>
  .hero {
    position: relative;
    min-height: 25rem;
    border-radius: 1.4rem;
    background:
      linear-gradient(180deg, rgba(6, 7, 11, 0.04) 0%, rgba(25, 24, 31, 0.82) 75%),
      var(--hero-image) center top / cover no-repeat;
    overflow: hidden;
    box-shadow: inset 0 1px rgba(255, 255, 255, 0.08);
  }

  .veil {
    position: absolute;
    inset: 0;
    background:
      linear-gradient(90deg, rgba(11, 11, 14, 0.6) 0%, rgba(11, 11, 14, 0.22) 45%, rgba(11, 11, 14, 0.08) 100%),
      radial-gradient(circle at 50% 0%, rgba(255, 173, 98, 0.18) 0%, rgba(255, 173, 98, 0) 48%);
    backdrop-filter: blur(0.5px);
  }

  .content {
    position: relative;
    z-index: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: 1rem;
    padding: 2.2rem 1.9rem 1.7rem;
  }

  .eyebrow {
    margin: 0;
    color: rgba(232, 230, 236, 0.56);
    font-size: 0.78rem;
    font-weight: 600;
  }

  h1 {
    margin: 0;
    max-width: 24rem;
    font: 700 clamp(1.9rem, 3vw, 2.45rem) / 1.05 'Bahnschrift', 'Segoe UI Variable Text', sans-serif;
    color: #f5f3f8;
  }

  .actions {
    display: flex;
    align-items: center;
    gap: 0.65rem;
  }

  .meta {
    display: flex;
    gap: 1.4rem;
    flex-wrap: wrap;
  }

  .metric {
    min-width: 7.5rem;
  }

  .metric span {
    display: block;
    margin-bottom: 0.28rem;
    color: rgba(228, 225, 234, 0.48);
    font-size: 0.72rem;
  }

  .metric p {
    margin: 0;
    color: #f5f3f8;
    font-size: 0.74rem;
    font-weight: 600;
  }

  @media (max-width: 900px) {
    .hero {
      min-height: 21rem;
    }

    .content {
      padding: 1.4rem;
    }
  }
</style>
