<script lang="ts">
  import { onMount } from "svelte";

  import type { Game } from "$lib/models/game";
  import { gameStore } from "$lib/stores/gameStore";
  import { FALLBACK_COVER_IMAGE } from "$lib/utils/constants";
  import { launchGame } from "$lib/services/tauriService";

  let launchMessage = $state("");

  onMount(() => {
    void gameStore.fetchGames();
  });

  async function handleLaunch(game: Game) {
    try {
      launchMessage = await launchGame(game.exePath);
    } catch (error) {
      launchMessage = error instanceof Error ? error.message : String(error);
    }
  }
</script>

<svelte:head>
  <title>Game Launcher</title>
</svelte:head>

<main class="page">
  <section class="hero">
    <p class="eyebrow">Desktop Game Launcher</p>
    <h1>Local game library</h1>
    <p class="intro">
      Games are loaded from <code>games.json</code> through the Svelte store, then launched through a Tauri
      command.
    </p>
    <div class="meta">
      <span>{$gameStore.loading ? "Loading games..." : `${$gameStore.games.length} games loaded`}</span>
      {#if $gameStore.selectedGame}
        <span>Selected: {$gameStore.selectedGame.title}</span>
      {/if}
    </div>
    {#if launchMessage}
      <p class="message">{launchMessage}</p>
    {/if}
  </section>

  <section class="content">
    <div class="panel">
      <h2>Library</h2>

      {#if $gameStore.games.length === 0 && !$gameStore.loading}
        <p class="empty">No games found yet.</p>
      {/if}

      <div class="grid">
        {#each $gameStore.games as game}
          <article class:selected={$gameStore.selectedGame?.id === game.id} class="card">
            <button class="select-button" type="button" on:click={() => gameStore.selectGame(game.id)}>
              <img src={game.coverArt || FALLBACK_COVER_IMAGE} alt={game.title} />
              <div class="card-copy">
                <h3>{game.title}</h3>
                <p>{game.platform} • {game.status}</p>
                <p>{game.genres.join(", ")}</p>
              </div>
            </button>
          </article>
        {/each}
      </div>
    </div>

    <div class="panel detail">
      <h2>Selected game</h2>

      {#if $gameStore.selectedGame}
        <h3>{$gameStore.selectedGame.title}</h3>
        <p>{$gameStore.selectedGame.description}</p>
        <dl>
          <div>
            <dt>Executable</dt>
            <dd>{$gameStore.selectedGame.exePath}</dd>
          </div>
          <div>
            <dt>Total playtime</dt>
            <dd>{$gameStore.selectedGame.totalPlaytime} minutes</dd>
          </div>
          <div>
            <dt>Last played</dt>
            <dd>{$gameStore.selectedGame.lastPlayed ?? "Never"}</dd>
          </div>
        </dl>
        <button
          class="launch"
          type="button"
          disabled={!$gameStore.selectedGame}
          on:click={() => {
            if ($gameStore.selectedGame) {
              void handleLaunch($gameStore.selectedGame);
            }
          }}
        >
          Launch Game
        </button>
      {:else}
        <p class="empty">Select a game to see its details.</p>
      {/if}
    </div>
  </section>
</main>

<style>
  :global(body) {
    margin: 0;
    font-family: "Segoe UI", Tahoma, Geneva, Verdana, sans-serif;
    background:
      radial-gradient(circle at top, rgba(41, 98, 255, 0.18), transparent 35%),
      linear-gradient(180deg, #08111f 0%, #101a2c 100%);
    color: #edf2ff;
  }

  .page {
    max-width: 1120px;
    margin: 0 auto;
    padding: 48px 24px 64px;
  }

  .hero {
    margin-bottom: 32px;
  }

  .eyebrow {
    margin: 0 0 8px;
    text-transform: uppercase;
    letter-spacing: 0.18em;
    color: #9db7ff;
    font-size: 0.78rem;
  }

  h1,
  h2,
  h3,
  p {
    margin-top: 0;
  }

  h1 {
    font-size: clamp(2.4rem, 6vw, 4rem);
    line-height: 1;
    margin-bottom: 12px;
  }

  .intro,
  .meta,
  .message,
  .empty,
  .card-copy p,
  dd {
    color: #c6d2f4;
  }

  .meta {
    display: flex;
    gap: 16px;
    flex-wrap: wrap;
  }

  .message {
    margin-top: 16px;
    padding: 12px 14px;
    border: 1px solid rgba(157, 183, 255, 0.3);
    border-radius: 14px;
    background: rgba(10, 23, 46, 0.72);
  }

  .content {
    display: grid;
    gap: 24px;
    grid-template-columns: 1.8fr 1fr;
  }

  .panel {
    padding: 22px;
    border-radius: 24px;
    background: rgba(8, 17, 31, 0.78);
    border: 1px solid rgba(157, 183, 255, 0.12);
    backdrop-filter: blur(18px);
  }

  .grid {
    display: grid;
    gap: 16px;
    grid-template-columns: repeat(auto-fit, minmax(220px, 1fr));
  }

  .card {
    border-radius: 18px;
    overflow: hidden;
    border: 1px solid rgba(157, 183, 255, 0.12);
    background: rgba(17, 28, 48, 0.92);
  }

  .card.selected {
    border-color: rgba(157, 183, 255, 0.75);
    box-shadow: 0 0 0 1px rgba(157, 183, 255, 0.3);
  }

  .select-button,
  .launch {
    cursor: pointer;
    border: 0;
    color: inherit;
  }

  .select-button {
    width: 100%;
    padding: 0;
    background: transparent;
    text-align: left;
  }

  img {
    width: 100%;
    aspect-ratio: 16 / 9;
    object-fit: cover;
    display: block;
    background: #13213b;
  }

  .card-copy {
    padding: 16px;
  }

  .detail dl {
    margin: 0 0 24px;
  }

  .detail div {
    margin-bottom: 16px;
  }

  dt {
    font-size: 0.82rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
    color: #9db7ff;
    margin-bottom: 4px;
  }

  dd {
    margin: 0;
    word-break: break-word;
  }

  .launch {
    padding: 14px 18px;
    border-radius: 14px;
    background: linear-gradient(135deg, #79a7ff 0%, #4b7dff 100%);
    color: #08111f;
    font-weight: 700;
  }

  .launch:disabled {
    cursor: not-allowed;
    opacity: 0.6;
  }

  code {
    font-family: Consolas, "Courier New", monospace;
  }

  @media (max-width: 900px) {
    .content {
      grid-template-columns: 1fr;
    }
  }
</style>
