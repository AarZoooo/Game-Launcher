<script lang="ts">
import { createEventDispatcher } from "svelte";
import Button from "$lib/components/common/Button.svelte";
import EmptyState from "$lib/components/common/EmptyState.svelte";
import Loader from "$lib/components/common/Loader.svelte";
import Modal from "$lib/components/common/Modal.svelte";
import { pageLabels } from "$lib/data/labels";
import type { ImportedGameResult } from "$lib/types/Game";

const dispatch = createEventDispatcher<{
	close: void;
	cancel: void;
	addall: void;
	addselected: string[];
	addmanual: void;
}>();

export let open = false;
export let platform: "steam" | "epic" | "local" = "steam";
export let loading = false;
export let error = "";
export let results: ImportedGameResult[] = [];

let selectedIds: string[] = [];

$: if (!open) {
	selectedIds = [];
}

$: platformLabel =
	platform === "steam"
		? pageLabels.platforms.steam
		: platform === "epic"
			? pageLabels.platforms.epic
			: pageLabels.importModal.autoSearch;

$: allSelected = results.length > 0 && selectedIds.length === results.length;
$: showAddManualAction =
	!loading && !error && platform === "local" && results.length === 0;

function toggle(id: string) {
	selectedIds = selectedIds.includes(id)
		? selectedIds.filter((value) => value !== id)
		: [...selectedIds, id];
}

function toggleAll() {
	selectedIds = allSelected ? [] : results.map((item) => item.id);
}

function truncate(path: string) {
	return path.length > 58 ? `${path.slice(0, 55)}...` : path;
}
</script>

<Modal
  {open}
  hideActions
  showCancel={false}
  title={platform === 'local'
    ? platformLabel
    : `${pageLabels.importModal.importPrefix} ${platformLabel} Games`}
  message={loading
    ? pageLabels.importModal.searching
    : error
      ? ''
      : pageLabels.importModal.reviewResults}
  on:close={() => dispatch('close')}
>
  {#if loading}
    <div class="loader-wrap">
      <Loader loading inline message={pageLabels.importModal.scanning} />
    </div>
  {:else if error}
    <EmptyState kind="errorState" message={error} />
  {:else if !results.length}
    <EmptyState kind="noResults" message={pageLabels.importModal.noResults} />
  {:else}
    <div class="selection-bar">
      <label class="select-all selection-check-row">
        <span class="selection-check-shell">
          <input
            type="checkbox"
            checked={allSelected}
            on:change={toggleAll}
          />
          <span class="selection-check-indicator" aria-hidden="true"></span>
        </span>
        <span class="selection-check-label">{pageLabels.importModal.selectAll}</span>
      </label>
    </div>

    <div class="results">
      {#each results as game}
        <label class="row selection-check-row" title={game.path}>
          <span class="selection-check-shell">
            <input
              type="checkbox"
              checked={selectedIds.includes(game.id)}
              on:change={() => toggle(game.id)}
            />
            <span class="selection-check-indicator" aria-hidden="true"></span>
          </span>
          <div class="copy">
            <strong class="selection-copy-title">{game.title}</strong>
            <span class="selection-copy-meta">{truncate(game.path)}</span>
          </div>
        </label>
      {/each}
    </div>
  {/if}

  <svelte:fragment slot="footer">
    <div class="footer">
      <Button quiet compact on:click={() => dispatch('cancel')}>
        {pageLabels.importModal.cancel}
      </Button>
      {#if showAddManualAction}
        <Button compact on:click={() => dispatch('addmanual')}>
          {pageLabels.games.addManually}
        </Button>
      {:else}
        <Button compact disabled={!selectedIds.length} on:click={() => dispatch('addselected', selectedIds)}>
          {pageLabels.importModal.addSelected}
        </Button>
      {/if}
    </div>
  </svelte:fragment>
</Modal>

<style>
  .loader-wrap {
    min-height: 12rem;
    display: grid;
    place-items: center;
    text-align: center;
    color: var(--text-secondary);
  }

  .results {
    max-height: 18rem;
    overflow: auto;
    display: grid;
    gap: var(--space-2);
    padding-right: 0.2rem;
  }

  .selection-bar {
    display: flex;
    justify-content: flex-start;
    margin-bottom: var(--space-2);
    padding: 0 var(--space-1);
  }

  .row {
    align-items: start;
    padding: var(--space-3) var(--space-1);
    border-radius: var(--radius-panel-sm);
    background: var(--surface-card);
    border: 1px solid var(--surface-border-soft);
  }

  .row :global(.selection-check-shell) {
    margin-top: 0.08rem;
  }

  .footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    margin-top: var(--space-4);
  }
</style>
