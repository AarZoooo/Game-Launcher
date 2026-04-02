<script lang="ts">
import { pageLabels } from "$lib/data/labels";
import { games } from "$lib/stores/libraryStore";
import type { Game } from "$lib/types/Game";
import {
	buildActivityCalendar,
	formatActivityDate,
	mockPlayActivityHours,
} from "$lib/utils/playActivity";

type GenreSlice = {
	label: string;
	value: string;
	percent: number;
	minutes: number;
	color: string;
};

const genrePalette = ["#ffb48a", "#c884ff", "#7cf08e", "#f7cf74", "#a8aebb"];

function parseHours(value: string) {
	const match = value.match(/\d+/);
	return match ? parseInt(match[0], 10) : 0;
}

function parsePlayMinutes(value?: string) {
	if (!value) return 0;

	const hourMatch = value.match(/(\d+)\s*h/i);
	const minuteMatch = value.match(/(\d+)\s*m/i);

	if (hourMatch || minuteMatch) {
		const hours = hourMatch ? parseInt(hourMatch[1], 10) : 0;
		const minutes = minuteMatch ? parseInt(minuteMatch[1], 10) : 0;
		return hours * 60 + minutes;
	}

	return parseHours(value) * 60;
}

function normalizeGenreLabel(value: string) {
	const normalized = value.trim().toLowerCase();

	if (!normalized) return "";
	if (normalized === "rpg" || normalized === "role playing") return "RPG";

	return normalized
		.split(/\s+/)
		.map((part) => part.charAt(0).toUpperCase() + part.slice(1))
		.join(" ");
}

function getGenres(game: Game) {
	const rawGenres = game.storageGenres?.length
		? game.storageGenres
		: game.genres.split(/[•/,|]+/);

	return [...new Set(rawGenres.map(normalizeGenreLabel).filter(Boolean))];
}

function formatPercent(value: number) {
	const rounded = Math.round(value * 10) / 10;
	return Number.isInteger(rounded) ? `${rounded}%` : `${rounded.toFixed(1)}%`;
}

function buildGenreDistribution(items: Game[]) {
	const genreMinutes = new Map<string, number>();

	for (const game of items.filter((item) => item.inLibrary !== false)) {
		const minutes =
			parsePlayMinutes(game.totalPlaytime) || parsePlayMinutes(game.hours);
		const genres = getGenres(game);

		if (!minutes || !genres.length) continue;

		const share = minutes / genres.length;

		for (const genre of genres) {
			genreMinutes.set(genre, (genreMinutes.get(genre) || 0) + share);
		}
	}

	const ranked = [...genreMinutes.entries()]
		.map(([label, minutes]) => ({ label, minutes }))
		.sort((left, right) => right.minutes - left.minutes);

	const topGenres = ranked.slice(0, 4);
	const otherMinutes = ranked
		.slice(4)
		.reduce((sum, item) => sum + item.minutes, 0);

	const groups = otherMinutes
		? [...topGenres, { label: "Other", minutes: otherMinutes }]
		: topGenres;

	const totalMinutes = groups.reduce((sum, item) => sum + item.minutes, 0);
	const slices: GenreSlice[] = groups.map((item, index) => {
		const percent = totalMinutes ? (item.minutes / totalMinutes) * 100 : 0;

		return {
			label: item.label,
			minutes: item.minutes,
			percent,
			value: formatPercent(percent),
			color: genrePalette[index] || genrePalette[genrePalette.length - 1],
		};
	});

	let offset = 0;
	const gradientStops = slices.map((slice) => {
		const start = offset;
		offset += slice.percent;
		return `${slice.color} ${start}% ${offset}%`;
	});

	return {
		slices,
		gradient: gradientStops.length
			? `conic-gradient(from 210deg, ${gradientStops.join(", ")})`
			: "conic-gradient(from 210deg, rgb(255 255 255 / 0.12) 0% 100%)",
	};
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
$: genreDistribution = buildGenreDistribution($games);

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
      <div class="ring-shell">
        <div class="ring" style={`background: ${genreDistribution.gradient};`}></div>
      </div>
      <div class="legend">
        {#if genreDistribution.slices.length}
          {#each genreDistribution.slices as item}
            <p style={`--genre-color: ${item.color};`}>
              <span>{item.label}</span>
              <b>{item.value}</b>
            </p>
          {/each}
        {:else}
          <p class="empty-legend">
            <span>{pageLabels.stats.noDataYet}</span>
          </p>
        {/if}
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
    border-radius: var(--radius-panel);
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
    border-radius: var(--radius-heatmap-cell);
    background: var(--surface-border);
    transition:
      transform var(--motion-fast) ease,
      filter var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease,
      background-color var(--motion-fast) ease;
  }

  .cell.level-0 {
    background: var(--surface-border);
  }

  .cell.level-1 {
    background: var(--color-secondary-1);
  }

  .cell.level-2 {
    background: var(--color-success-2);
  }

  .cell.level-3 {
    background: var(--color-secondary-3);
  }

  button.cell:hover,
  button.cell:focus-visible {
    transform: scale(1.18);
    filter: brightness(1.08);
    box-shadow: var(--shadow-border);
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
    border-radius: var(--radius-panel-sm);
    background: var(--surface-glass);
    border: 1px solid var(--surface-border-soft);
    backdrop-filter: blur(var(--blur-md));
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
    border-radius: var(--radius-panel-sm);
    background: var(--surface-card);
    border: 1px solid var(--surface-border-soft);
  }

  .genre-block {
    display: flex;
    align-items: center;
    gap: var(--space-4);
  }

  .ring-shell {
    position: relative;
    width: 4.5rem;
    height: 4.5rem;
    flex: 0 0 auto;
    display: grid;
    place-items: center;
    border-radius: var(--radius-round);
    background: var(--color-surface-1);
    box-shadow: var(--shadow-outline-subtle);
  }

  .ring {
    width: 100%;
    height: 100%;
    border-radius: var(--radius-round);
    box-shadow: var(--shadow-soft-glow);
    -webkit-mask: radial-gradient(circle, transparent 0 58%, #000 60% 100%);
    mask: radial-gradient(circle, transparent 0 58%, #000 60% 100%);
  }

  .legend p {
    display: flex;
    gap: var(--space-3);
    justify-content: space-between;
    margin: 0 0 var(--space-1);
    min-width: 8.75rem;
    color: var(--text-secondary);
    font-size: 0.76rem;
  }

  .legend span {
    color: var(--text-primary);
  }

  .legend b {
    color: var(--genre-color, var(--text-primary));
    font-weight: 600;
  }

  .empty-legend span {
    color: var(--text-secondary);
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
