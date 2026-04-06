<script lang="ts">
import { onDestroy, tick } from "svelte";
import { fade } from "svelte/transition";
import { pageLabels } from "$lib/data/labels";
import { games } from "$lib/stores/libraryStore";
import { activeGameId, isGameRunning } from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";
import { portal } from "$lib/utils/portal";
import {
	buildActivityCalendar,
	calculateActiveNow,
	calculateGamesPlayed,
	calculateMostPlayed,
	formatActivityDate,
	generateActivityByDate,
	getInstalledGames,
} from "$lib/utils/stats/analytics";
import {
	type DailyBreakdownEntry,
	getDailyBreakdown,
} from "$lib/utils/stats/getDailyBreakdown";

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
let tooltipPanel: HTMLDivElement;
let tooltipStyle = "";
let tooltipPortalTarget = "#portal-root";
let tooltipListeners: Array<() => void> = [];
let hoveredCell: HTMLButtonElement | null = null;
let hoverSequence = 0;
let hoveredDay: {
	label: string;
	breakdown: DailyBreakdownEntry[];
	totalMinutes: number;
} | null = null;

$: installedAnalyticsGames = getInstalledGames($games);
$: totalPlaytimeMinutes = installedAnalyticsGames.reduce(
	(sum, game) => sum + (game.storageTotalPlaytimeMinutes ?? 0),
	0,
);
$: activityByDate = generateActivityByDate(installedAnalyticsGames);
$: calendar = buildActivityCalendar(activityByDate, totalPlaytimeMinutes);
$: playedCount = calculateGamesPlayed(installedAnalyticsGames);
$: activeNowCount = calculateActiveNow(
	installedAnalyticsGames,
	$isGameRunning,
	$activeGameId,
);
$: favoriteCount = installedAnalyticsGames.filter(
	(game) => game.favorite,
).length;
$: mostPlayed = calculateMostPlayed(installedAnalyticsGames);
$: insightMetrics = [
	{ label: pageLabels.stats.gamesPlayed, value: String(playedCount) },
	{ label: pageLabels.stats.activeRightNow, value: `${activeNowCount} games` },
	{ label: pageLabels.stats.favorites, value: `${favoriteCount} picks` },
	{
		label: pageLabels.stats.longestStreak,
		value: `${calendar.longestStreak} days`,
	},
	{
		label: pageLabels.stats.currentStreak,
		value: `${calendar.currentStreak} days`,
	},
	{
		label: pageLabels.stats.mostPlayed,
		value: mostPlayed?.title || pageLabels.stats.noDataYet,
	},
];
$: genreDistribution = buildGenreDistribution(installedAnalyticsGames);

$: console.log("Stats Source:", {
	installedGames: installedAnalyticsGames.map((game) => ({
		id: game.id,
		title: game.title,
		inLibrary: game.inLibrary,
		favorite: game.favorite,
		totalPlaytimeMinutes: game.storageTotalPlaytimeMinutes ?? 0,
		sessionCount: game.storageSessions?.length ?? 0,
	})),
	totalPlaytimeMinutes,
	favoriteCount,
	playedCount,
	activeNowCount,
	mostPlayed: mostPlayed?.title ?? null,
});

$: if (heatmapCard) {
	heatmapCard.style.setProperty(
		"--week-count",
		String(calendar.monthHeaders.length),
	);
}

function resolveLengthToken(tokenName: string, fallbackPx: number) {
	if (typeof window === "undefined") {
		return fallbackPx;
	}

	const rawValue = getComputedStyle(document.documentElement)
		.getPropertyValue(tokenName)
		.trim();

	if (!rawValue) {
		return fallbackPx;
	}

	if (rawValue.endsWith("px")) {
		const parsed = parseFloat(rawValue);
		return Number.isFinite(parsed) ? parsed : fallbackPx;
	}

	if (rawValue.endsWith("rem")) {
		const parsed = parseFloat(rawValue);
		if (!Number.isFinite(parsed)) {
			return fallbackPx;
		}

		const rootFontSize = parseFloat(
			getComputedStyle(document.documentElement).fontSize,
		);
		return parsed * (Number.isFinite(rootFontSize) ? rootFontSize : 16);
	}

	const parsed = parseFloat(rawValue);
	return Number.isFinite(parsed) ? parsed : fallbackPx;
}

function formatDetailedDate(date: Date) {
	return date.toLocaleDateString("en-US", {
		month: "long",
		day: "numeric",
		year: "numeric",
	});
}

function formatDuration(minutes: number) {
	if (!minutes) return "0m";

	const hours = Math.floor(minutes / 60);
	const remainingMinutes = minutes % 60;

	if (!hours) return `${remainingMinutes}m`;
	if (!remainingMinutes) return `${hours}h`;
	return `${hours}h ${remainingMinutes}m`;
}

function teardownTooltipListeners() {
	for (const dispose of tooltipListeners) {
		dispose();
	}

	tooltipListeners = [];
}

function updateTooltipPosition() {
	if (!hoveredCell || !tooltipPanel || typeof window === "undefined") {
		return;
	}

	const cellRect = hoveredCell.getBoundingClientRect();
	const panelRect = tooltipPanel.getBoundingClientRect();
	const gap = resolveLengthToken("--space-2", 8);
	const viewportPadding = resolveLengthToken("--space-3", 12);

	let top = cellRect.top - panelRect.height - gap;
	let left = cellRect.left + cellRect.width / 2 - panelRect.width / 2;

	if (top < viewportPadding) {
		top = cellRect.bottom + gap;
	}

	const maxTop = Math.max(
		viewportPadding,
		window.innerHeight - panelRect.height - viewportPadding,
	);
	const maxLeft = Math.max(
		viewportPadding,
		window.innerWidth - panelRect.width - viewportPadding,
	);

	top = Math.min(Math.max(viewportPadding, top), maxTop);
	left = Math.min(Math.max(viewportPadding, left), maxLeft);

	tooltipStyle = `top: ${top}px; left: ${left}px;`;
}

async function showTooltip(
	cell: {
		date: Date;
	},
	element: HTMLButtonElement,
) {
	const breakdown = getDailyBreakdown(cell.date, installedAnalyticsGames);
	const totalMinutes = breakdown.reduce(
		(sum, entry) => sum + entry.duration,
		0,
	);
	const sequence = ++hoverSequence;

	hoveredCell = element;
	hoveredDay = {
		label: formatDetailedDate(cell.date),
		breakdown,
		totalMinutes,
	};

	await tick();

	if (sequence !== hoverSequence || !hoveredDay) {
		return;
	}

	updateTooltipPosition();
	teardownTooltipListeners();

	if (typeof window !== "undefined") {
		const handleViewportChange = () => {
			updateTooltipPosition();
		};

		window.addEventListener("resize", handleViewportChange);
		window.addEventListener("scroll", handleViewportChange, true);
		tooltipListeners = [
			() => window.removeEventListener("resize", handleViewportChange),
			() => window.removeEventListener("scroll", handleViewportChange, true),
		];
	}
}

function hideTooltip() {
	hoverSequence += 1;
	hoveredCell = null;
	hoveredDay = null;
	tooltipStyle = "";
	teardownTooltipListeners();
}

onDestroy(() => {
	teardownTooltipListeners();
});
</script>

<section class="stats">
  <hr class="divider" />

  <div class="summary">
    <div class="summary-left">
      <div class="metric-hero">
        <strong>{calendar.totalHours}</strong>
        <span>{pageLabels.stats.totalHours}</span>
      </div>

      <div class="metrics">
        {#each insightMetrics as metric}
          <div class="metric">
            <span>{metric.label}</span>
            <strong>{metric.value}</strong>
          </div>
        {/each}
      </div>
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
            aria-label={`${formatActivityDate(cell.date)}: ${cell.hours} hour${cell.hours === 1 ? '' : 's'} played`}
            on:mouseenter={(event) => showTooltip(cell, event.currentTarget as HTMLButtonElement)}
            on:focus={(event) => showTooltip(cell, event.currentTarget as HTMLButtonElement)}
            on:mouseleave={hideTooltip}
            on:blur={hideTooltip}
          ></button>
        {:else}
          <span class="cell empty" aria-hidden="true"></span>
        {/if}
      {/each}
    </div>

  </div>

  {#if hoveredDay}
    <div
      bind:this={tooltipPanel}
      use:portal={tooltipPortalTarget}
      class="menu-panel menu-panel-floating tooltip-popup"
      role="tooltip"
      style={tooltipStyle}
      transition:fade={{ duration: 140 }}
    >
      <p class="tooltip-date">{hoveredDay.label}</p>

      {#if hoveredDay.breakdown.length}
        <div class="tooltip-list">
          {#each hoveredDay.breakdown as entry}
            <p class="tooltip-row">
              <span>{entry.game}</span>
              <b>{formatDuration(entry.duration)}</b>
            </p>
          {/each}
        </div>

        <div class="tooltip-total">
          <span>Total</span>
          <strong>{formatDuration(hoveredDay.totalMinutes)}</strong>
        </div>
      {:else}
        <p class="tooltip-empty">No activity</p>
      {/if}
    </div>
  {/if}
</section>

<style>
  .stats {
    padding: var(--space-1) 0 var(--space-8);
    display: flex;
    flex-direction: column;
    gap: var(--space-8);
  }

  .divider {
    border: 0;
    border-top: 1px solid var(--surface-border-soft);
    margin: 0;
  }

  .metric-hero {
    display: flex;
    align-items: baseline;
    gap: var(--space-3);
    padding-bottom: var(--space-4);
  }

  .metric-hero span {
    color: var(--text-muted);
    font-size: var(--font-size-caption);
    text-transform: uppercase;
    letter-spacing: 0.08em;
  }

  .metric-hero strong {
    font-size: var(--font-size-display);
    font-family: var(--font-display);
    color: var(--text-primary);
  }

  .heatmap-card {
    overflow-x: auto;
  }

  .month-row,
  .heatmap-grid {
    display: grid;
    grid-template-columns: repeat(var(--week-count), 1fr);
    gap: var(--space-1);
  }

  .month-row {
    margin-bottom: 0.45rem;
  }

  .month-row span {
    color: var(--text-muted);
    font-size: var(--font-size-caption-sm);
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
    background: rgb(255 255 255 / 0.04);
  }

  .cell.level-1 {
    background: rgb(255 255 255 / 0.1);
  }

  .cell.level-2 {
    background: rgb(255 255 255 / 0.2);
  }

  .cell.level-3 {
    background: rgb(255 255 255 / 0.35);
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

  .tooltip-popup {
    position: fixed;
    z-index: var(--z-floating-menu);
    min-width: 13rem;
    max-width: min(18rem, calc(100vw - (var(--space-3) * 2)));
    padding: var(--space-3);
    border-radius: var(--radius-panel);
    pointer-events: none;
  }

  .tooltip-date {
    margin: 0;
    color: var(--text-primary);
    font-size: var(--font-size-caption);
    font-weight: 600;
  }

  .tooltip-list {
    display: grid;
    gap: var(--space-2);
    margin-top: var(--space-3);
  }

  .tooltip-row,
  .tooltip-total {
    display: flex;
    align-items: baseline;
    justify-content: space-between;
    gap: var(--space-3);
    margin: 0;
  }

  .tooltip-row span,
  .tooltip-total span {
    color: var(--text-secondary);
    font-size: var(--font-size-control-sm);
  }

  .tooltip-row b,
  .tooltip-total strong {
    color: var(--text-primary);
    font-size: var(--font-size-control-sm);
    font-weight: 600;
    white-space: nowrap;
  }

  .tooltip-total {
    margin-top: var(--space-3);
    padding-top: var(--space-3);
    border-top: 1px solid var(--surface-border-soft);
  }

  .tooltip-empty {
    margin: var(--space-3) 0 0;
    color: var(--text-secondary);
    font-size: var(--font-size-control-sm);
  }

  .streak-summary {
    display: flex;
    gap: var(--space-3);
    margin-top: var(--space-4);
    flex-wrap: wrap;
  }

  .streak-card {
    min-width: 10rem;
  }

  .metric span {
    display: block;
    color: var(--text-muted);
    font-size: var(--font-size-caption);
  }

  .metric strong {
    display: block;
    margin-top: var(--space-1);
    font-size: var(--font-size-body-sm);
    color: var(--text-primary);
  }

  .summary {
    display: flex;
    align-items: flex-start;
    gap: var(--space-6);
    justify-content: space-between;
  }

  .summary-left {
    flex: 1;
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .metrics {
    display: grid;
    grid-template-columns: repeat(4, minmax(0, 1fr));
    gap: var(--space-4) var(--space-5);
    flex: 1;
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
    font-size: var(--font-size-control-sm);
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
