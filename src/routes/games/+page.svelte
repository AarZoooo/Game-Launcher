<script lang="ts">
  import Button from '$lib/components/common/Button.svelte';
  import ImportGamesModal from '$lib/components/game/ImportGamesModal.svelte';
  import FilterPanel from '$lib/components/game/FilterPanel.svelte';
  import GameGrid from '$lib/components/game/GameGrid.svelte';
  import { scanEpicGames, scanSteamGames } from '$lib/services/gameService';
  import { launchGame, openGameFolder, openSaveFolder } from '$lib/services/tauriService';
  import {
    catalogGames,
    games,
    hasDuplicateGame,
    installedGames,
    type Game,
    type ImportedGameResult
  } from '$lib/stores/libraryStore';
  import { scanningState, uiStore } from '$lib/stores/uiStore';
  import { sortOptions } from '$lib/utils/constants';

  let showFilters = false;
  let sortBy = 'default';
  let scannedPlatform: 'steam' | 'epic' = 'steam';
  let showImportModal = false;
  let scanError = '';
  let scanResults: ImportedGameResult[] = [];
  let filteredInstalledGames: Game[] = [];
  let filteredCatalogGames: Game[] = [];
  let filters = {
    genre: '',
    coop: '',
    status: '',
    showFavorites: false
  };

  async function startScan(platform: 'steam' | 'epic') {
    scannedPlatform = platform;
    scanError = '';
    scanResults = [];
    showImportModal = true;
    uiStore.setScanning(platform, true);

    try {
      const results = platform === 'steam' ? await scanSteamGames() : await scanEpicGames();
      scanResults = results.filter((item) => !hasDuplicateGame(item));
    } catch (error) {
      scanError = error instanceof Error ? error.message : 'Scan failed. Please try again.';
    } finally {
      uiStore.setScanning(platform, false);
    }
  }

  function addSelected(event: CustomEvent<string[]>) {
    const selected = scanResults.filter((item) => event.detail.includes(item.id));
    games.addImportedGames(selected);
    closeImport();
  }

  function addAll() {
    games.addImportedGames(scanResults);
    closeImport();
  }

  function closeImport() {
    showImportModal = false;
    scanResults = [];
    scanError = '';
  }

  function applyFilters(event: CustomEvent<{ genre?: string; coop?: string; status?: string; showFavorites?: boolean }>) {
    filters = {
      genre: event.detail.genre || '',
      coop: event.detail.coop || '',
      status: event.detail.status || '',
      showFavorites: Boolean(event.detail.showFavorites)
    };

    showFilters = false;
  }

  function handleMenuAction(event: CustomEvent<{ id: string; game: Game }>) {
    const { id, game } = event.detail;

    if (id === 'play' && game.path) return launchGame(game.path);
    if (id === 'toggle-favorite') return games.toggleFavorite(game.id);
    if (id === 'open-folder') return openGameFolder(game.path);
    if (id === 'open-save-folder') return openSaveFolder(game.savePath);
    if (id === 'toggle-cloud-sync') return games.toggleCloudSync(game.id);
    if (id === 'remove-library') return games.removeFromLibrary(game.id);

    if (id === 'edit-details') {
      const title = window.prompt('Game title', game.title);
      if (title) games.updateDetails(game.id, { title });
      return;
    }

    if (id === 'change-cover') {
      const cover = window.prompt('Cover image URL or local asset path', game.cover);
      if (cover) games.updateDetails(game.id, { cover });
      return;
    }

    if (id === 'launch-options') {
      const value = window.prompt('Launch arguments', game.launchOptions || '');
      if (value !== null) games.setLaunchOptions(game.id, value);
      return;
    }

    if (id === 'create-shortcut') {
      window.alert(`Desktop shortcut will be supported by the backend layer for ${game.title}.`);
      return;
    }

    if (id === 'sync-now') {
      window.alert(`Cloud sync for ${game.title} will be handled once backend sync is connected.`);
    }
  }

  function filterGames(items: Game[]) {
    return items.filter((game) => {
      const matchesGenre = !filters.genre || game.genres.toLowerCase().includes(filters.genre);
      const matchesCoop = !filters.coop || game.coop.toLowerCase() === filters.coop;
      const matchesStatus = !filters.status || game.status === filters.status;
      const matchesFavorites = !filters.showFavorites || game.favorite;
      return matchesGenre && matchesCoop && matchesStatus && matchesFavorites;
    });
  }

  function sortGames(items: Game[]) {
    if (sortBy === 'title') {
      return [...items].sort((a, b) => a.title.localeCompare(b.title));
    }

    if (sortBy === 'hours') {
      return [...items].sort((a, b) => parseInt(b.hours) - parseInt(a.hours));
    }

    if (sortBy === 'rating') {
      return [...items].sort((a, b) => parseFloat(b.rating) - parseFloat(a.rating));
    }

    return items;
  }

  $: filteredInstalledGames = sortGames(filterGames($installedGames));
  $: filteredCatalogGames = sortGames(filterGames($catalogGames));
</script>

<div class="games">
  <div class="header">
    <h1>All Games</h1>

    <div class="controls">
      <Button quiet compact on:click={() => startScan('steam')}>
        {$scanningState.steam ? 'Scanning...' : 'Import Steam'}
      </Button>
      <Button quiet compact on:click={() => startScan('epic')}>
        {$scanningState.epic ? 'Scanning...' : 'Import Epic'}
      </Button>

      <label>
        <span>Sort by:</span>
        <select bind:value={sortBy}>
          {#each sortOptions as option}
            <option value={option.value}>{option.label}</option>
          {/each}
        </select>
      </label>

      <button on:click={() => (showFilters = true)}>Filters</button>
    </div>
  </div>

  <section class="section">
    <div class="section-header">
      <div>
        <h2>Installed Games</h2>
        <p>Loaded from your local backend storage.</p>
      </div>
      <span>{filteredInstalledGames.length} shown</span>
    </div>

    {#if filteredInstalledGames.length > 0}
      <GameGrid games={filteredInstalledGames} context="library" on:action={handleMenuAction} />
    {:else}
      <p class="empty">No installed games match the current filters.</p>
    {/if}
  </section>

  <section class="section">
    <div class="section-header">
      <div>
        <h2>Catalogue</h2>
        <p>Temporary catalog and recommendation data kept separate from your installed library.</p>
      </div>
      <span>{filteredCatalogGames.length} shown</span>
    </div>

    {#if filteredCatalogGames.length > 0}
      <GameGrid games={filteredCatalogGames} context="explore" on:action={handleMenuAction} />
    {:else}
      <p class="empty">No catalog games match the current filters.</p>
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
    gap: 1.5rem;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 1rem;
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
    font-family: 'Bahnschrift', 'Segoe UI Variable Text', sans-serif;
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
    color: rgba(226, 223, 231, 0.54);
    font-size: 0.8rem;
  }

  .controls {
    display: flex;
    align-items: center;
    flex-wrap: wrap;
    gap: 0.8rem;
  }

  label,
  button,
  select {
    font: inherit;
  }

  label {
    display: flex;
    align-items: center;
    gap: 0.55rem;
    color: rgba(226, 223, 231, 0.54);
    font-size: 0.8rem;
  }

  select,
  .controls > button {
    min-width: 7.2rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.08);
    color: #f4f2f7;
    padding: 0.52rem 0.8rem;
    border-radius: 0.12rem;
  }

  .controls > button {
    cursor: pointer;
    font-weight: 700;
  }

  .empty {
    padding: 1rem 1.1rem;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.04);
    color: rgba(226, 223, 231, 0.7);
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
  }
</style>
