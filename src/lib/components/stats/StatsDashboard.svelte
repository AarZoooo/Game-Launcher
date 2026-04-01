<script lang="ts">
import { genreShares, pageLabels } from "$lib/data/labels";
import { games } from "$lib/stores/libraryStore";
import {
	buildActivityCalendar,
	formatActivityDate,
	mockPlayActivityHours,
} from "$lib/utils/playActivity";

function parseHours(value: string) {
	const match = value.match(/\d+/);
	return match ? parseInt(match[0], 10) : 0;
}

let heatmapCard: HTMLDivElement;

$: calendar = buildActivityCalendar(mockPlayActivityHours);
$: playedCount = $games.filter((game) => game.status === "played").length;
$: playingCount = $games.filter((game) => game.status === "playing").length;
$: favoriteCount = $games.filter((game) => game.favorite).length;
$: mostPlayed = [...$games].sort(
	(left, right) => parseHours(right.hours) - parseHours(left.hours),
)[0];
$: insightMetrics = [
	{ label: pageLabels.stats.gamesPlayed, value: String(playedCount) },
	{ label: pageLabels.stats.activeRightNow, value: `${playingCount} games` },
	{ label: pageLabels.stats.favorites, value: `${favoriteCount} picks` },
	{
		label: pageLabels.stats.mostPlayed,
		value: mostPlayed?.title || pageLabels.stats.noDataYet,
	},
];

$: if (heatmapCard) {
	heatmapCard.style.setProperty(
		"--week-count",
		String(calendar.monthHeaders.length),
	);
}
</script>

<section class="stats">
  <div class="heading">
    <div>
      <p class="eyebrow">{pageLabels.stats.yearlyActivity}</p>
      <h2>{pageLabels.stats.playStreak}</h2>
    </div>

    <div class="hours-total">
      <span>{pageLabels.stats.totalHours}</span>
      <strong>{calendar.totalHours}</strong>
    </div>
  </div>

  <div bind:this={heatmapCard} class="heatmap-card">
    <div class="month-row" aria-hidden="true">
      {#each calendar.monthHeaders as month}
        <span>{month.label}</span>
      {/each}
    </div>

    <div class="heatmap-grid">
      {#each calendar.cells as cell}
        {#if cell.kind === 'day'}
          <button
            type="button"
            class={`cell level-${cell.level}`}
            title={`${formatActivityDate(cell.date)} - ${cell.hours} hour${cell.hours === 1 ? '' : 's'}`}
            aria-label={`${formatActivityDate(cell.date)}: ${cell.hours} hour${cell.hours === 1 ? '' : 's'} played`}
          ></button>
        {:else}
          <span class="cell empty" aria-hidden="true"></span>
        {/if}
      {/each}
    </div>

    <div class="streak-summary">
      <div class="streak-card">
        <span>{pageLabels.stats.longestStreak}</span>
        <strong>{calendar.longestStreak} days</strong>
      </div>

      <div class="streak-card">
        <span>{pageLabels.stats.currentStreak}</span>
        <strong>{calendar.currentStreak} days</strong>
      </div>
    </div>
  </div>

  <div class="summary">
    <div class="metrics">
      {#each insightMetrics as metric}
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
    display: flex;
    flex-direction: column;
    gap: 1.2rem;
  }

  .heading {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 1rem;
  }

  .eyebrow,
  .hours-total span {
    margin: 0 0 0.25rem;
    color: var(--text-muted);
    font-size: 0.74rem;
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  h2,
  .hours-total strong {
    margin: 0;
  }

  h2 {
    font-size: 1.05rem;
  }

  .hours-total {
    text-align: right;
  }

  .hours-total strong {
    display: block;
    font-size: 1.6rem;
    color: var(--text-primary);
  }

  .heatmap-card {
    padding: var(--space-4);
    border-radius: var(--radius-lg);
    background: var(--surface-card);
    border: 1px solid var(--surface-border);
    box-shadow: var(--shadow-inset);
    overflow-x: auto;
  }

  .month-row,
  .heatmap-grid {
    display: grid;
    grid-template-columns: repeat(var(--week-count), 0.82rem);
    gap: var(--space-1);
    min-width: max-content;
  }

  .month-row {
    margin-bottom: 0.45rem;
  }

  .month-row span {
    color: var(--text-muted);
    font-size: 0.68rem;
    min-width: 0.82rem;
  }

  .heatmap-grid {
    grid-auto-flow: column;
    grid-template-rows: repeat(7, 0.82rem);
  }

  .cell {
    width: 0.82rem;
    height: 0.82rem;
    padding: 0;
    border: 0;
    border-radius: 0.18rem;
    background: rgba(210, 214, 220, 0.18);
    transition:
      transform var(--motion-fast) ease,
      filter var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease,
      background-color var(--motion-fast) ease;
  }

  .cell.level-0 {
    background: rgba(210, 214, 220, 0.18);
  }

  .cell.level-1 {
    background: #7ea76d;
  }

  .cell.level-2 {
    background: #4a9c4f;
  }

  .cell.level-3 {
    background: #1f5f2f;
  }

  button.cell:hover,
  button.cell:focus-visible {
    transform: scale(1.18);
    filter: brightness(1.08);
    box-shadow: 0 0 0 1px var(--surface-border);
    outline: none;
  }

  .cell.empty {
    background: transparent;
    pointer-events: none;
  }

  .streak-summary {
    display: flex;
    gap: var(--space-3);
    margin-top: var(--space-4);
    flex-wrap: wrap;
  }

  .streak-card {
    min-width: 10rem;
    padding: var(--space-3) var(--space-4);
    border-radius: var(--radius-md);
    background: var(--surface-glass);
    border: 1px solid var(--surface-border-soft);
    backdrop-filter: blur(10px);
  }

  .streak-card span,
  .metric span {
    display: block;
    color: var(--text-muted);
    font-size: 0.72rem;
  }

  .streak-card strong,
  .metric strong {
    display: block;
    margin-top: var(--space-1);
    font-size: 0.88rem;
    color: var(--text-primary);
  }

  .summary {
    display: flex;
    gap: var(--space-6);
    justify-content: space-between;
  }

  .metrics {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--space-4) var(--space-5);
    flex: 1;
  }

  .metric {
    padding: var(--space-4);
    border-radius: var(--radius-md);
    background: var(--surface-card);
    border: 1px solid var(--surface-border-soft);
  }

  .genre-block {
    display: flex;
    align-items: center;
    gap: var(--space-4);
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
    gap: var(--space-3);
    justify-content: space-between;
    margin: 0 0 var(--space-1);
    min-width: 7rem;
    color: var(--text-secondary);
    font-size: 0.72rem;
  }

  .legend span {
    color: var(--text-secondary);
  }

  .legend b {
    color: var(--text-primary);
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

  @media (max-width: 720px) {
    .heading {
      flex-direction: column;
      align-items: flex-start;
    }

    .hours-total {
      text-align: left;
    }

    .metrics {
      grid-template-columns: 1fr;
    }
  }
</style>
