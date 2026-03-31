<script lang="ts">
import { games } from "$lib/stores/libraryStore";
import { genreShares } from "$lib/utils/constants";
import {
	buildActivityCalendar,
	formatActivityDate,
	mockPlayActivityHours,
} from "$lib/utils/playActivity";

function parseHours(value: string) {
	const match = value.match(/\d+/);
	return match ? parseInt(match[0], 10) : 0;
}

$: calendar = buildActivityCalendar(mockPlayActivityHours);
$: playedCount = $games.filter((game) => game.status === "played").length;
$: playingCount = $games.filter((game) => game.status === "playing").length;
$: favoriteCount = $games.filter((game) => game.favorite).length;
$: mostPlayed = [...$games].sort(
	(left, right) => parseHours(right.hours) - parseHours(left.hours),
)[0];
$: insightMetrics = [
	{ label: "Games Played", value: String(playedCount) },
	{ label: "Active Right Now", value: `${playingCount} games` },
	{ label: "Favorites", value: `${favoriteCount} picks` },
	{ label: "Most Played", value: mostPlayed?.title || "No data yet" },
];
</script>

<section class="stats">
  <div class="heading">
    <div>
      <p class="eyebrow">Yearly activity</p>
      <h2>Play Streak</h2>
    </div>

    <div class="hours-total">
      <span>Total Hours</span>
      <strong>[{calendar.totalHours}]</strong>
    </div>
  </div>

  <div class="heatmap-card" style={`--week-count:${calendar.monthHeaders.length}`}>
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
            title={`${formatActivityDate(cell.date)} • ${cell.hours} hour${cell.hours === 1 ? '' : 's'}`}
            aria-label={`${formatActivityDate(cell.date)}: ${cell.hours} hour${cell.hours === 1 ? '' : 's'} played`}
          ></button>
        {:else}
          <span class="cell empty" aria-hidden="true"></span>
        {/if}
      {/each}
    </div>

    <div class="streak-summary">
      <div class="streak-card">
        <span>Longest streak</span>
        <strong>{calendar.longestStreak} days</strong>
      </div>

      <div class="streak-card">
        <span>Current streak</span>
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
    color: rgba(220, 216, 225, 0.52);
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
    color: #eef5ec;
  }

  .heatmap-card {
    padding: 1rem 1rem 1.05rem;
    border-radius: 1.15rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: inset 0 1px rgba(255, 255, 255, 0.04);
    overflow-x: auto;
  }

  .month-row,
  .heatmap-grid {
    display: grid;
    grid-template-columns: repeat(var(--week-count), 0.82rem);
    gap: 0.28rem;
    min-width: max-content;
  }

  .month-row {
    margin-bottom: 0.45rem;
  }

  .month-row span {
    color: rgba(220, 216, 225, 0.46);
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
    background: #3d4046;
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
    background: #557a4b;
  }

  .cell.level-2 {
    background: #3b8f4f;
  }

  .cell.level-3 {
    background: #1f5f2f;
  }

  button.cell:hover,
  button.cell:focus-visible {
    transform: scale(1.18);
    filter: brightness(1.08);
    box-shadow: 0 0 0 1px rgba(255, 255, 255, 0.12);
    outline: none;
  }

  .cell.empty {
    background: transparent;
    pointer-events: none;
  }

  .streak-summary {
    display: flex;
    gap: 0.8rem;
    margin-top: 1rem;
    flex-wrap: wrap;
  }

  .streak-card {
    min-width: 10rem;
    padding: 0.8rem 0.9rem;
    border-radius: 0.9rem;
    background: var(--surface-glass);
    border: 1px solid rgba(255, 255, 255, 0.08);
    backdrop-filter: blur(var(--ui-blur));
  }

  .streak-card span,
  .metric span {
    display: block;
    color: rgba(220, 216, 225, 0.46);
    font-size: 0.72rem;
  }

  .streak-card strong,
  .metric strong {
    display: block;
    margin-top: 0.24rem;
    font-size: 0.88rem;
    color: #f1eff4;
  }

  .summary {
    display: flex;
    gap: 1.6rem;
    justify-content: space-between;
  }

  .metrics {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: 1rem 1.2rem;
    flex: 1;
  }

  .metric {
    padding: 0.95rem 1rem;
    border-radius: 0.95rem;
    background: rgba(255, 255, 255, 0.03);
    border: 1px solid rgba(255, 255, 255, 0.07);
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
