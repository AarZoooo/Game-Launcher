<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { coopOptions, genreOptions, statusOptions } from '$lib/utils/constants';

  const dispatch = createEventDispatcher();

  let showFavorites = false;
  let status = '';
  let genre = '';
  let coop = '';

  function applyFilters() {
    dispatch('apply', {
      showFavorites,
      status,
      genre,
      coop
    });
  }

  function close() {
    dispatch('close');
  }
</script>

<button type="button" class="overlay" aria-label="Close filters" on:click={close}></button>

<div class="panel">
  <h2>Filters</h2>

  <div class="section">
    <label>
      <input type="checkbox" bind:checked={showFavorites} />
      Show only favourites
    </label>
  </div>

  <div class="section">
    <p>Status</p>
    <select bind:value={status}>
      {#each statusOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  </div>

  <div class="section">
    <p>Genres</p>
    <select bind:value={genre}>
      {#each genreOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  </div>

  <div class="section">
    <p>Co-op Support</p>
    <select bind:value={coop}>
      {#each coopOptions as option}
        <option value={option.value}>{option.label}</option>
      {/each}
    </select>
  </div>

  <button class="apply" on:click={applyFilters}>
    Apply Filters
  </button>
</div>

<style>
  .overlay {
    position: fixed;
    inset: 0;
    border: 0;
    padding: 0;
    background: rgba(9, 9, 12, 0.36);
    backdrop-filter: blur(calc(var(--ui-blur) * 0.4));
  }

  .panel {
    position: fixed;
    right: 0;
    top: 0;
    width: min(24rem, calc(100vw - 2rem));
    height: 100%;
    background:
      linear-gradient(180deg, rgba(108, 86, 52, 0.14), rgba(61, 64, 74, 0.72)),
      rgba(58, 59, 66, 0.72);
    border-left: 1px solid rgba(228, 223, 236, 0.2);
    box-shadow: -1rem 0 3rem rgba(0, 0, 0, 0.3);
    backdrop-filter: blur(calc(var(--ui-blur) * 1.6));
    padding: 1.8rem;
    display: flex;
    flex-direction: column;
    gap: 1.4rem;
    z-index: 10;
  }

  h2 {
    margin: 0;
    font-size: 1rem;
  }

  .section {
    display: flex;
    flex-direction: column;
    gap: 0.5rem;
    color: rgba(235, 232, 239, 0.72);
  }

  .section p,
  label {
    margin: 0;
    font-size: 0.9rem;
  }

  select {
    padding: 0.68rem 0.85rem;
    border-radius: 0.75rem;
    background: var(--surface-glass);
    border: 1px solid var(--surface-border);
    color: white;
    backdrop-filter: blur(var(--ui-blur));
    box-shadow: var(--surface-shadow);
  }

  .apply {
    margin-top: auto;
    padding: 0.9rem;
    background: #b69b57;
    border: none;
    border-radius: 0.6rem;
    cursor: pointer;
    color: white;
    font-weight: 700;
  }
</style>
