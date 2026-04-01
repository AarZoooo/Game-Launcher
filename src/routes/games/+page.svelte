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
let filteredInstalledGames: Game[] = [];
let filteredCatalogGames: Game[] = [];
let filters: GameFilterState = { ...defaultGameFilters };

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

function addSelected(event: CustomEvent<string[]>) {
	const selected = scanResults.filter((item) => event.detail.includes(item.id));
	void addImportedGamesToLibrary(selected).then((message) => {
		autoSearchMessage = message;
		closeImport();
	});
}

function addAll() {
	void addImportedGamesToLibrary(scanResults).then((message) => {
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
		return [...items].sort((a, b) => parseInt(b.hours) - parseInt(a.hours));
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
  on:addall={addAll}
  on:addselected={addSelected}
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
    padding: var(--space-5);
    border-radius: var(--radius-lg);
    background: var(--surface-card);
    border: 1px solid var(--surface-border);
  }

  .header {
    display: flex;
    justify-content: space-between;
    align-items: center;
    gap: 1rem;
  }

  h1 {
    margin: 0;
    font-size: 1.9rem;
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
    font-size: 0.8rem;
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
    font-size: 0.8rem;
  }

  .controls > button {
    min-width: 7.2rem;
    border: 1px solid var(--surface-border);
    background: var(--surface-glass);
    color: var(--text-primary);
    padding: 0.6rem 0.85rem;
    border-radius: var(--radius-md);
    box-shadow: var(--shadow-sm);
    backdrop-filter: blur(10px);
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
    background: rgba(255, 255, 255, 0.09);
  }

  .controls > button:hover {
    transform: translateY(-1px);
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
