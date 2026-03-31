<script lang="ts">
  import { createEventDispatcher, onMount } from 'svelte';
  import type { Game } from '$lib/stores/libraryStore';

  type GameMenuContext = 'library' | 'explore' | 'home';

  interface MenuAction {
    id: string;
    label: string;
    tone?: 'danger';
  }

  const dispatch = createEventDispatcher<{
    action: { id: string; game: Game };
  }>();

  export let game: Game;
  export let context: GameMenuContext = 'library';

  let open = false;
  let root: HTMLDivElement;
  $: menuGroups = actionsForContext();

  function actionsForContext(): MenuAction[][] {
    if (context === 'explore') {
      return [
        [
          { id: 'status-want', label: 'Want to Play' },
          { id: 'status-playing', label: 'Playing' },
          { id: 'status-played', label: 'Played' }
        ]
      ];
    }

    if (context === 'home') {
      return [
        [
          { id: 'play', label: 'Play' },
          { id: 'toggle-favorite', label: game.favorite ? 'Remove Favorite' : 'Favorite' },
          {
            id: game.resumeState === 'restart' ? 'restart' : 'resume',
            label: game.resumeState === 'restart' ? 'Restart' : 'Resume'
          }
        ],
        [{ id: 'open-folder', label: 'Open Folder' }],
        [{ id: 'view-playtime', label: 'View Playtime Details' }],
        [{ id: 'hide-continue', label: 'Remove from Continue Playing', tone: 'danger' }]
      ];
    }

    return [
      [
        { id: 'play', label: 'Play' },
        { id: 'toggle-favorite', label: game.favorite ? 'Remove Favorite' : 'Add Favorite' }
      ],
      [
        { id: 'open-folder', label: 'Open Game Folder' },
        { id: 'open-save-folder', label: 'Open Save Folder' }
      ],
      [
        { id: 'edit-details', label: 'Edit Details' },
        { id: 'change-cover', label: 'Change Cover Art' }
      ],
      [
        { id: 'sync-now', label: 'Sync Now' },
        {
          id: 'toggle-cloud-sync',
          label: game.cloudSyncEnabled ? 'Disable Cloud Sync' : 'Toggle Cloud Sync'
        }
      ],
      [
        { id: 'launch-options', label: 'Launch Options' },
        { id: 'create-shortcut', label: 'Create Desktop Shortcut' }
      ],
      [{ id: 'remove-library', label: 'Remove from Library', tone: 'danger' }]
    ];
  }

  function select(actionId: string) {
    open = false;
    dispatch('action', { id: actionId, game });
  }

  onMount(() => {
    const close = (event: MouseEvent) => {
      if (root && !root.contains(event.target as Node)) {
        open = false;
      }
    };

    window.addEventListener('click', close);
    return () => window.removeEventListener('click', close);
  });
</script>

<div class="menu-root" bind:this={root}>
  <button
    type="button"
    class="menu-trigger"
    aria-label={`Open menu for ${game.title}`}
    on:click|stopPropagation={() => (open = !open)}
  >
    ...
  </button>

  {#if open}
    <div class="menu" role="menu" tabindex="-1" on:click|stopPropagation on:keydown|stopPropagation>
      {#each menuGroups as group, index}
        <div class="group">
          {#each group as action}
            <button
              type="button"
              class:danger={action.tone === 'danger'}
              on:click={() => select(action.id)}
            >
              {action.label}
            </button>
          {/each}
        </div>

        {#if index < menuGroups.length - 1}
          <div class="separator"></div>
        {/if}
      {/each}
    </div>
  {/if}
</div>

<style>
  .menu-root {
    position: relative;
    z-index: 4;
  }

  .menu-trigger {
    min-width: auto;
    border: 0;
    background: rgba(255, 255, 255, 0.08);
    color: rgba(239, 236, 243, 0.78);
    width: 1.8rem;
    height: 1.55rem;
    padding: 0;
    cursor: pointer;
    font: inherit;
    font-weight: 700;
    line-height: 1;
  }

  .menu {
    position: absolute;
    right: 0;
    bottom: calc(100% + 0.45rem);
    min-width: 13.5rem;
    padding: 0.5rem;
    background: rgba(54, 55, 62, 0.96);
    border: 1px solid rgba(255, 255, 255, 0.08);
    box-shadow: 0 1rem 2rem rgba(0, 0, 0, 0.28);
    backdrop-filter: blur(10px);
  }

  .group {
    display: grid;
    gap: 0.12rem;
  }

  .group button {
    border: 0;
    background: transparent;
    color: #f1eef5;
    text-align: left;
    padding: 0.55rem 0.65rem;
    font: inherit;
    font-size: 0.78rem;
    cursor: pointer;
  }

  .group button:hover {
    background: rgba(255, 255, 255, 0.08);
  }

  .group button.danger {
    color: #ffb0a6;
  }

  .separator {
    height: 1px;
    margin: 0.35rem 0;
    background: rgba(255, 255, 255, 0.08);
  }
</style>
