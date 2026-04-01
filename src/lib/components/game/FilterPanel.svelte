<script lang="ts">
import { createEventDispatcher } from "svelte";
import GlassSelect from "$lib/components/common/GlassSelect.svelte";
import { coopOptions, genreOptions, statusOptions } from "$lib/data/filters";
import { pageLabels } from "$lib/data/labels";
import type { GameFilterState } from "$lib/types/UI";

const dispatch = createEventDispatcher<{
	apply: GameFilterState;
	close: void;
}>();

let showFavorites = false;
let status = "";
let genre = "";
let coop = "";

function applyFilters() {
	dispatch("apply", {
		showFavorites,
		status,
		genre,
		coop,
	});
}

function close() {
	dispatch("close");
}
</script>

<button
  type="button"
  class="overlay"
  aria-label={pageLabels.filterPanel.close}
  on:click={close}
></button>

<div class="panel">
  <h2>{pageLabels.filterPanel.title}</h2>

  <div class="section">
    <label>
      <input type="checkbox" bind:checked={showFavorites} />
      {pageLabels.filterPanel.showFavorites}
    </label>
  </div>

  <div class="section">
    <p>{pageLabels.filterPanel.status}</p>
    <GlassSelect bind:value={status} options={statusOptions} ariaLabel={pageLabels.filterPanel.status} fullWidth />
  </div>

  <div class="section">
    <p>{pageLabels.filterPanel.genres}</p>
    <GlassSelect bind:value={genre} options={genreOptions} ariaLabel={pageLabels.filterPanel.genres} fullWidth />
  </div>

  <div class="section">
    <p>{pageLabels.filterPanel.coopSupport}</p>
    <GlassSelect bind:value={coop} options={coopOptions} ariaLabel={pageLabels.filterPanel.coopSupport} fullWidth />
  </div>

  <button class="apply" on:click={applyFilters}>
    {pageLabels.filterPanel.apply}
  </button>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    border: 0;
    padding: 0;
    background: var(--color-overlay-1);
    backdrop-filter: blur(calc(var(--ui-blur) * 0.4));
  }

  .panel {
    position: fixed;
    right: 0;
    top: 0;
    width: min(24rem, calc(100vw - 2rem));
    height: 100%;
    background:
      linear-gradient(180deg, rgb(var(--accent-rgb) / 0.14), var(--surface-glass)),
      var(--surface-glass);
    border-left: 1px solid var(--surface-border);
    box-shadow: var(--shadow-lg);
    backdrop-filter: blur(10px);
    padding: var(--space-7);
    display: flex;
    flex-direction: column;
    gap: var(--space-6);
    z-index: var(--z-modal);
  }

  h2 {
    margin: 0;
    font-size: 1rem;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: var(--space-2);
    color: var(--text-secondary);
  }

  .section p,
  label {
    margin: 0;
    font-size: 0.9rem;
  }

  .apply {
    margin-top: auto;
    min-height: var(--control-height-md);
    padding: 0 var(--space-4);
    background: var(--interactive-primary-bg);
    border: none;
    border-radius: var(--radius-md);
    cursor: pointer;
    color: var(--interactive-primary-text);
    font-weight: 700;
    box-shadow: var(--shadow-sm);
  }
</style>
