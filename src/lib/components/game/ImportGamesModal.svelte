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
		? "Steam"
		: platform === "epic"
			? "Epic"
			: pageLabels.importModal.autoSearch;

$: allSelected = results.length > 0 && selectedIds.length === results.length;

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
      <label class="select-all">
        <input
          type="checkbox"
          checked={allSelected}
          on:change={toggleAll}
        />
        <span>{pageLabels.importModal.selectAll}</span>
      </label>
    </div>

    <div class="results">
      {#each results as game}
        <label class="row" title={game.path}>
          <input
            type="checkbox"
            checked={selectedIds.includes(game.id)}
            on:change={() => toggle(game.id)}
          />
          <div class="copy">
            <strong>{game.title}</strong>
            <span>{truncate(game.path)}</span>
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
      <Button compact disabled={!selectedIds.length} on:click={() => dispatch('addselected', selectedIds)}>
        {pageLabels.importModal.addSelected}
      </Button>
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
    margin-bottom: var(--space-3);
  }

  .select-all {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--text-primary);
    font-size: 0.8rem;
    font-weight: 600;
  }

  .row {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: var(--space-3);
    align-items: start;
    padding: var(--space-3) var(--space-1);
    border-radius: var(--radius-md);
    background: var(--surface-card);
    border: 1px solid var(--surface-border-soft);
  }

  .copy strong {
    display: block;
    font-size: 0.88rem;
  }

  .copy span {
    display: block;
    margin-top: var(--space-1);
    color: var(--text-secondary);
    font-size: 0.75rem;
  }

  .footer {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    margin-top: var(--space-4);
  }
</style>
