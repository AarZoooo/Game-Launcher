<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import Button from '$lib/components/common/Button.svelte';
  import Loader from '$lib/components/common/Loader.svelte';
  import Modal from '$lib/components/common/Modal.svelte';
  import type { ImportedGameResult } from '$lib/stores/libraryStore';

  const dispatch = createEventDispatcher<{
    close: void;
    cancel: void;
    addall: void;
    addselected: string[];
  }>();

  export let open = false;
  export let platform: 'steam' | 'epic' = 'steam';
  export let loading = false;
  export let error = '';
  export let results: ImportedGameResult[] = [];

  let selectedIds: string[] = [];

  $: if (!open) {
    selectedIds = [];
  }

  $: if (results.length && !selectedIds.length) {
    selectedIds = results.map((item) => item.id);
  }

  function toggle(id: string) {
    selectedIds = selectedIds.includes(id)
      ? selectedIds.filter((value) => value !== id)
      : [...selectedIds, id];
  }

  function truncate(path: string) {
    return path.length > 58 ? `${path.slice(0, 55)}...` : path;
  }
</script>

<Modal
  {open}
  hideActions
  showCancel={false}
  title={`Import ${platform === 'steam' ? 'Steam' : 'Epic'} Games`}
  message={loading ? 'Scanning local libraries across available drives...' : error ? '' : 'Choose the detected games you want to add to your library.'}
  on:close={() => dispatch('close')}
>
  {#if loading}
    <div class="loader-wrap">
      <Loader loading inline message="Scanning..." />
    </div>
  {:else if error}
    <div class="state error">{error}</div>
  {:else if !results.length}
    <div class="state">No games were detected on your connected drives.</div>
  {:else}
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
      <Button quiet compact on:click={() => dispatch('cancel')}>Cancel</Button>
      <Button quiet compact on:click={() => dispatch('addall')}>Add All</Button>
      <Button compact on:click={() => dispatch('addselected', selectedIds)}>
        Add Selected
      </Button>
    </div>
  </svelte:fragment>
</Modal>

<style>
  .loader-wrap,
  .state {
    min-height: 12rem;
    display: grid;
    place-items: center;
    text-align: center;
    color: rgba(233, 230, 239, 0.66);
  }

  .state.error {
    color: #ffb4a6;
  }

  .results {
    max-height: 18rem;
    overflow: auto;
    display: grid;
    gap: 0.55rem;
    padding-right: 0.2rem;
  }

  .row {
    display: grid;
    grid-template-columns: auto 1fr;
    gap: 0.75rem;
    align-items: start;
    padding: 0.65rem 0.1rem;
  }

  .copy strong {
    display: block;
    font-size: 0.88rem;
  }

  .copy span {
    display: block;
    margin-top: 0.2rem;
    color: rgba(233, 230, 239, 0.52);
    font-size: 0.75rem;
  }

  .footer {
    display: flex;
    justify-content: flex-end;
    gap: 0.6rem;
    margin-top: 1rem;
  }
</style>
