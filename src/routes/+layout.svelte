<script lang="ts">
  import { onMount } from 'svelte';
  import AppShell from '$lib/components/layout/AppShell.svelte';
  import { games } from '$lib/stores/libraryStore';

  onMount(() => {
    // TODO: Keep temporary frontend-only features until matching backend commands exist.
    void games.loadFromBackend().catch((error) => {
      console.error('Failed to load stored library:', error);
    });
  });
</script>

<svelte:head>
  <title>Scoped Launcher</title>
</svelte:head>

<AppShell>
  <slot />
</AppShell>
