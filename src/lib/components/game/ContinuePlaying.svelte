<script lang="ts">
import { createEventDispatcher } from "svelte";
import GameMenu from "$lib/components/game/GameMenu.svelte";
import GamePlayButton from "$lib/components/game/GamePlayButton.svelte";
import SyncBadge from "$lib/components/sync/SyncBadge.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";

const dispatch = createEventDispatcher<{
	action: { id: string; game: Game };
}>();

export let game: Game;
</script>

<section class="hero">
  <img class="hero-media" src={game.hero || game.cover} alt="" loading="lazy" />
  <div class="veil"></div>
  <div class="content">
    <p class="eyebrow">{pageLabels.continuePlaying.eyebrow}</p>
    <h1>{game.title}</h1>

    <div class="actions">
      <GamePlayButton {game} compact />
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
    border-radius: var(--radius-xl);
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
      linear-gradient(90deg, rgba(11, 11, 14, 0.6) 0%, rgba(11, 11, 14, 0.22) 45%, rgba(11, 11, 14, 0.08) 100%),
      radial-gradient(circle at 50% 0%, rgb(var(--accent-rgb) / 0.18) 0%, transparent 48%);
    backdrop-filter: blur(calc(var(--ui-blur) * 0.08));
  }

  .content {
    position: relative;
    z-index: 1;
    height: 100%;
    display: flex;
    flex-direction: column;
    justify-content: flex-end;
    gap: var(--space-4);
    padding: var(--space-9) var(--space-8) var(--space-7);
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
    gap: var(--space-3);
  }

  .meta {
    display: flex;
    gap: var(--space-6);
    flex-wrap: wrap;
  }

  .metric {
    min-width: 7.5rem;
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
  }
</style>

