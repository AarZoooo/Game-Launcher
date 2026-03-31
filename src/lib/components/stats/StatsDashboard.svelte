<script lang="ts">
  import { games } from '$lib/stores/libraryStore';
  import { genreShares, statMetrics, statsHeatmap } from '$lib/utils/constants';

  $: playedCount = $games.filter((game) => game.status === 'played').length;
  $: playingCount = $games.filter((game) => game.status === 'playing').length;
  $: favoriteCount = $games.filter((game) => game.favorite).length;
  $: dynamicMetrics = statMetrics.map((metric) => {
    if (metric.label === 'Games Played') return { ...metric, value: String(playedCount) };
    if (metric.label === 'Current Streak') return { ...metric, value: `${playingCount} active` };
    if (metric.label === 'Favorite Game') return { ...metric, value: `${favoriteCount} favorites` };
    return metric;
  });
</script>

<section class="stats">
  <h2>Your Game Stats</h2>

  <div class="heatmap">
    {#each statsHeatmap as value}
      <div class="cell" style={`opacity:${Math.max(0.18, value)}; transform: scaleY(${0.72 + value * 0.48})`}></div>
    {/each}
  </div>

  <div class="summary">
    <div class="metrics">
      {#each dynamicMetrics as metric}
        <div class="metric">
          <span>{metric.label}</span>
          <strong>{metric.value}</strong>
        </div>
      {/each}
    </div>

    <div class="genre-block">
      <div class="ring"></div>
      <div class="legend">
        {#each genreShares as item}
          <p><span>{item.label}</span><b>{item.value}</b></p>
        {/each}
      </div>
    </div>
  </div>
</section>

<style>
  .stats {
    padding: 0.4rem 0 0;
  }

  h2 {
    margin: 0 0 0.9rem;
    font-size: 0.95rem;
  }

  .heatmap {
    display: grid;
    grid-template-columns: repeat(49, 1fr);
    gap: 0.23rem;
    align-items: end;
  }

  .cell {
    height: 1.8rem;
    border-radius: 0.12rem;
    background: linear-gradient(180deg, rgba(179, 181, 190, 0.08), rgba(208, 210, 218, 0.42));
  }

  .summary {
    display: flex;
    gap: 1.6rem;
    justify-content: space-between;
    margin-top: 1rem;
  }

  .metrics {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 1rem 1.4rem;
    flex: 1;
  }

  .metric span {
    display: block;
    color: rgba(220, 216, 225, 0.46);
    font-size: 0.7rem;
  }

  .metric strong {
    display: block;
    margin-top: 0.25rem;
    font-size: 0.78rem;
    color: #f1eff4;
  }

  .genre-block {
    display: flex;
    align-items: center;
    gap: 1rem;
  }

  .ring {
    width: 3.1rem;
    height: 3.1rem;
    border-radius: 50%;
    background:
      radial-gradient(circle at center, #3f4149 46%, transparent 47%),
      conic-gradient(#d8d2ff 0 13%, #f0b8aa 13% 33%, #b9dd86 33% 60%, #8ea0ff 60% 100%);
    box-shadow: 0 0 1rem rgba(255, 255, 255, 0.08);
  }

  .legend p {
    display: flex;
    gap: 0.65rem;
    justify-content: space-between;
    margin: 0 0 0.32rem;
    min-width: 7rem;
    color: rgba(220, 216, 225, 0.62);
    font-size: 0.72rem;
  }

  .legend span {
    color: rgba(220, 216, 225, 0.62);
  }

  .legend b {
    color: #f5f3f8;
    font-weight: 600;
  }

  @media (max-width: 1100px) {
    .summary {
      flex-direction: column;
    }

    .metrics {
      grid-template-columns: repeat(2, minmax(0, 1fr));
    }
  }
</style>
