<script lang="ts">
import Button from "$lib/components/common/Button.svelte";
import EmptyState from "$lib/components/common/EmptyState.svelte";
import GlassSelect from "$lib/components/common/GlassSelect.svelte";
import FilterPanel from "$lib/components/game/FilterPanel.svelte";
import GameGrid from "$lib/components/game/GameGrid.svelte";
import ImportGamesModal from "$lib/components/game/ImportGamesModal.svelte";
import { defaultGameFilters, sortOptions } from "$lib/data/filters";
import { pageLabels } from "$lib/data/labels";
import { performGameAction } from "$lib/services/gameService";
import {
	addImportedGamesToLibrary,
	buildImportAddedMessage,
	pickManualImportCandidate,
	scanImportCandidates,
} from "$lib/services/importService";
import {
	type IgdbSearchResult,
	searchIgdbGame,
} from "$lib/services/tauriService";
import {
	catalogGames,
	hasDuplicateGame,
	installedGames,
} from "$lib/stores/libraryStore";
import { scanningState, uiStore } from "$lib/stores/uiStore";
import type { Game, ImportedGameResult } from "$lib/types/Game";
import type { GameMenuActionId } from "$lib/types/Menu";
import type { GameFilterState, ScanPlatform } from "$lib/types/UI";

let showFilters = false;
let sortBy = "default";
let scannedPlatform: ScanPlatform = "steam";
let showImportModal = false;
let scanError = "";
let scanResults: ImportedGameResult[] = [];
let autoSearchMessage = "";
let igdbDebugLoading = false;
let igdbDebugError = "";
let igdbDebugResults: IgdbSearchResult[] = [];
let filteredInstalledGames: Game[] = [];
let filteredCatalogGames: Game[] = [];
let filters: GameFilterState = { ...defaultGameFilters };

async function runIgdbDebugSearch() {
	igdbDebugLoading = true;
	igdbDebugError = "";

	try {
		const results = await searchIgdbGame("Satisfactory");
		igdbDebugResults = results;
		console.log("IGDB debug results for Satisfactory:", results);
	} catch (error) {
		igdbDebugResults = [];
		igdbDebugError = error instanceof Error ? error.message : String(error);
		console.error("IGDB debug search failed:", error);
	} finally {
		igdbDebugLoading = false;
	}
}

async function startScan(platform: "steam" | "epic") {
	scannedPlatform = platform;
	scanError = "";
	scanResults = [];
	showImportModal = true;
	uiStore.setScanning(platform, true);

	try {
		const results = await scanImportCandidates(platform);
		scanResults = results.filter((item) => !hasDuplicateGame(item));
	} catch (error) {
		scanError =
			error instanceof Error ? error.message : pageLabels.games.scanFailed;
	} finally {
		uiStore.setScanning(platform, false);
	}
}

function startAutoSearch() {
	if ($scanningState.local) {
		return;
	}

	scannedPlatform = "local";
	showImportModal = true;
	scanError = "";
	scanResults = [];
	autoSearchMessage = "";
	uiStore.setScanning("local", true);

	void scanImportCandidates("local")
		.then((results) => {
			scanResults = results.filter((item) => !hasDuplicateGame(item));
		})
		.catch((error) => {
			scanError =
				error instanceof Error
					? error.message
					: pageLabels.games.autoSearchFailed;
		})
		.finally(() => {
			uiStore.setScanning("local", false);
		});
}

async function addManualGame() {
	try {
		const candidate = await pickManualImportCandidate();
		if (!candidate || hasDuplicateGame(candidate)) {
			return;
		}

		await addImportedGamesToLibrary([candidate]);
		autoSearchMessage = buildImportAddedMessage(candidate.title);
	} catch (error) {
		scanError =
			error instanceof Error ? error.message : pageLabels.games.manualAddFailed;
	}
}

async function addManualFromImportModal() {
	await addManualGame();
	closeImport();
}

function addSelected(event: CustomEvent<string[]>) {
	const selected = scanResults.filter((item) => event.detail.includes(item.id));
	void addImportedGamesToLibrary(selected).then((message) => {
		autoSearchMessage = message;
		closeImport();
	});
}

function closeImport() {
	showImportModal = false;
	scanResults = [];
	scanError = "";
}

function applyFilters(event: CustomEvent<GameFilterState>) {
	filters = {
		genre: event.detail.genre || "",
		coop: event.detail.coop || "",
		status: event.detail.status || "",
		showFavorites: Boolean(event.detail.showFavorites),
	};

	showFilters = false;
}

function handleMenuAction(event: CustomEvent<{ id: string; game: Game }>) {
	return performGameAction(
		event.detail.id as GameMenuActionId,
		event.detail.game,
	);
}

function filterGames(items: Game[]) {
	return items.filter((game) => {
		const matchesGenre =
			!filters.genre || game.genres.toLowerCase().includes(filters.genre);
		const matchesCoop =
			!filters.coop || game.coop.toLowerCase() === filters.coop;
		const matchesStatus = !filters.status || game.status === filters.status;
		const matchesFavorites = !filters.showFavorites || game.favorite;
		return matchesGenre && matchesCoop && matchesStatus && matchesFavorites;
	});
}

function sortGames(items: Game[]) {
	if (sortBy === "title") {
		return [...items].sort((a, b) => a.title.localeCompare(b.title));
	}

	if (sortBy === "hours") {
		return [...items].sort(
			(a, b) =>
				(b.storageTotalPlaytimeMinutes ?? 0) -
				(a.storageTotalPlaytimeMinutes ?? 0),
		);
	}

	if (sortBy === "rating") {
		return [...items].sort(
			(a, b) => parseFloat(b.rating) - parseFloat(a.rating),
		);
	}

	return items;
}

$: filteredInstalledGames = sortGames(filterGames($installedGames));
$: filteredCatalogGames = sortGames(filterGames($catalogGames));
</script>

<div class="games">
  <div class="header">
    <h1>{pageLabels.games.title}</h1>

    <div class="controls">
      <Button quiet compact on:click={() => startScan('steam')}>
        {$scanningState.steam ? pageLabels.importModal.scanning : pageLabels.games.importSteam}
      </Button>
      <Button quiet compact on:click={() => startScan('epic')}>
        {$scanningState.epic ? pageLabels.importModal.scanning : pageLabels.games.importEpic}
      </Button>
      <Button quiet compact on:click={startAutoSearch}>
        {$scanningState.local ? pageLabels.games.searching : pageLabels.games.autoSearch}
      </Button>
      <Button quiet compact on:click={addManualGame}>{pageLabels.games.addManually}</Button>

      <label>
        <span>{pageLabels.games.sortBy}</span>
        <GlassSelect bind:value={sortBy} options={sortOptions} ariaLabel={pageLabels.games.sortBy} />
      </label>

      <button on:click={() => (showFilters = true)}>{pageLabels.games.filters}</button>
    </div>
  </div>

  <section class="section">
    <div class="section-header">
      <div>
        <h2>{pageLabels.games.installedGames}</h2>
        <p>
          {pageLabels.games.installedDescription}
          {#if autoSearchMessage}
            <span class="status-copy">{autoSearchMessage}</span>
          {/if}
        </p>
      </div>
      <span>{filteredInstalledGames.length} {pageLabels.games.shownSuffix}</span>
    </div>

    {#if filteredInstalledGames.length > 0}
      <GameGrid games={filteredInstalledGames} context="library" on:action={handleMenuAction} />
    {:else}
      <EmptyState kind="emptyLibrary" message={pageLabels.games.installedEmpty} />
    {/if}
  </section>

  <section class="section">
    <div class="section-header">
      <div>
        <h2>{pageLabels.games.catalogue}</h2>
        <p>{pageLabels.games.catalogueDescription}</p>
      </div>
      <span>{filteredCatalogGames.length} {pageLabels.games.shownSuffix}</span>
    </div>

    {#if filteredCatalogGames.length > 0}
      <GameGrid games={filteredCatalogGames} context="explore" on:action={handleMenuAction} />
    {:else}
      <EmptyState kind="noResults" message={pageLabels.games.catalogEmpty} />
    {/if}
  </section>

  <section class="section debug-panel">
    <div class="section-header">
      <div>
        <h2>IGDB Debug</h2>
        <p>Temporary frontend-only test hook for `search_igdb_game` with hardcoded title `Satisfactory`.</p>
      </div>
      <Button quiet compact on:click={runIgdbDebugSearch}>
        {igdbDebugLoading ? 'Searching...' : 'Test IGDB Search'}
      </Button>
    </div>

    {#if igdbDebugError}
      <p class="debug-error">{igdbDebugError}</p>
    {/if}

    {#if igdbDebugResults.length > 0}
      <div class="debug-results">
        {#each igdbDebugResults as result}
          <article class="debug-result">
            <strong>{result.name}</strong>
            <span>ID: {result.id}</span>
            <span>Slug: {result.slug || 'n/a'}</span>
            <span>Genres: {result.genres.length ? result.genres.join(', ') : 'none'}</span>
            <span>Cover: {result.coverUrl || 'n/a'}</span>
            <span>Screenshots: {result.screenshotUrls.length}</span>
            {#if result.summary}
              <p>{result.summary}</p>
            {/if}
          </article>
        {/each}
      </div>
    {:else if !igdbDebugLoading}
      <p class="debug-placeholder">No debug results yet.</p>
    {/if}
  </section>
</div>

{#if showFilters}
  <FilterPanel on:close={() => (showFilters = false)} on:apply={applyFilters} />
{/if}

<ImportGamesModal
  open={showImportModal}
  platform={scannedPlatform}
  loading={$scanningState[scannedPlatform]}
  error={scanError}
  results={scanResults}
  on:cancel={closeImport}
  on:close={closeImport}
  on:addselected={addSelected}
  on:addmanual={addManualFromImportModal}
/>

<style>
  .games {
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  h1 {
    margin: 0;
    font-size: var(--font-size-display);
    font-family: var(--font-display);
  }

  h2,
  p {
    margin: 0;
  }

  .section-header {
    display: flex;
    justify-content: space-between;
    align-items: flex-end;
    gap: 1rem;
  }

  .section-header p,
  .section-header span {
    color: var(--text-secondary);
    font-size: var(--font-size-control);
  }

  .status-copy {
    display: inline-block;
    margin-left: var(--space-2);
    color: var(--text-primary);
  }

  .controls {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: var(--space-3);
  }

  label,
  button {
    font: inherit;
  }

  label {
    display: flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--text-secondary);
    font-size: var(--font-size-control);
  }

  .controls > button {
    min-width: 7.2rem;
    border: 1px solid var(--surface-border);
    background: var(--surface-glass);
    color: var(--text-primary);
    padding: 0.6rem 0.85rem;
    border-radius: var(--radius-control-md);
    box-shadow: var(--shadow-sm);
    backdrop-filter: blur(var(--blur-md));
  }

  .controls > button {
    cursor: pointer;
    font-weight: 700;
    transition:
      background-color var(--motion-fast) ease,
      transform var(--motion-fast) ease;
  }

  .controls > button:hover,
  :global(.select-trigger:hover) {
    background: var(--surface-hover);
  }

  .controls > button:hover {
    transform: translateY(-1px);
  }

  .debug-panel {
    gap: var(--space-3);
  }

  .debug-results {
    display: grid;
    gap: var(--space-3);
  }

  .debug-result {
    display: grid;
    gap: var(--space-1);
    padding: var(--space-4);
    border: 1px solid var(--surface-border-soft);
    background: var(--surface-glass);
    color: var(--text-secondary);
  }

  .debug-result strong {
    color: var(--text-primary);
    font-size: var(--font-size-body);
  }

  .debug-result span,
  .debug-result p,
  .debug-placeholder,
  .debug-error {
    margin: 0;
    font-size: var(--font-size-control);
  }

  .debug-result p {
    margin-top: var(--space-2);
    line-height: 1.45;
  }

  .debug-error {
    color: var(--color-danger-1);
  }

  @media (max-width: 980px) {
    .header {
      flex-direction: column;
      align-items: flex-start;
    }

    .section-header {
      flex-direction: column;
      align-items: flex-start;
    }

    .status-copy {
      display: block;
      margin-left: 0;
      margin-top: 0.35rem;
    }
  }
</style>
