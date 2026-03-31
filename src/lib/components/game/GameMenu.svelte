<script lang="ts">
  import { createEventDispatcher, onDestroy, onMount } from 'svelte';
  import type { Game } from '$lib/stores/libraryStore';
  import { activeGameId, activeMenuKey, isGameRunning, uiStore } from '$lib/stores/uiStore';

  type GameMenuContext = 'library' | 'explore' | 'home';
  type MenuPlacement = 'below-right' | 'side-right' | 'above-right';

  interface MenuAction {
    id: string;
    label: string;
    tone?: 'danger';
    disabled?: boolean;
  }

  const dispatch = createEventDispatcher<{
    action: { id: string; game: Game };
  }>();

  export let game: Game;
  export let context: GameMenuContext = 'library';
  export let placement: MenuPlacement = 'below-right';

  let root: HTMLDivElement;
  let menuKey = '';
  $: isActiveGame = $isGameRunning && $activeGameId === game.id;
  $: if (!menuKey && game?.id) {
    menuKey = `${game.id}-${Math.random().toString(36).slice(2, 10)}`;
  }
  $: open = menuKey !== '' && $activeMenuKey === menuKey;
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
          { id: 'play', label: isActiveGame ? 'Playing' : 'Play', disabled: isActiveGame },
          { id: 'toggle-favorite', label: game.favorite ? 'Remove Favorite' : 'Favorite' },
          {
            id: game.resumeState === 'restart' ? 'restart' : 'resume',
            label: game.resumeState === 'restart' ? 'Restart' : 'Resume',
            disabled: isActiveGame
          }
        ],
        [{ id: 'open-folder', label: 'Open Folder' }],
        [{ id: 'view-playtime', label: 'View Playtime Details' }],
        [{ id: 'hide-continue', label: 'Remove from Continue Playing', tone: 'danger' }]
      ];
    }

    return [
      [
        { id: 'play', label: isActiveGame ? 'Playing' : 'Play', disabled: isActiveGame },
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
    uiStore.closeOpenMenu();
    dispatch('action', { id: actionId, game });
  }

  function toggleMenu() {
    if (!menuKey) return;

    if (open) {
      uiStore.closeOpenMenu();
      return;
    }

    uiStore.setOpenMenu(menuKey);
  }

  onMount(() => {
    const close = (event: MouseEvent) => {
      if (open && root && !root.contains(event.target as Node)) {
        uiStore.closeOpenMenu();
      }
    };

    window.addEventListener('click', close);
    return () => window.removeEventListener('click', close);
  });

  onDestroy(() => {
    if (open) {
      uiStore.closeOpenMenu();
    }
  });
</script>

<div class="menu-root" bind:this={root}>
  <button
    type="button"
    class:open
    class="menu-trigger"
    aria-label={`Open menu for ${game.title}`}
    on:click|stopPropagation={toggleMenu}
  >
    <svg viewBox="0 0 16 16" aria-hidden="true">
      <circle cx="3" cy="8" r="1.2"></circle>
      <circle cx="8" cy="8" r="1.2"></circle>
      <circle cx="13" cy="8" r="1.2"></circle>
    </svg>
  </button>

  {#if open}
    <div
      class:side-right={placement === 'side-right'}
      class:above-right={placement === 'above-right'}
      class="menu"
      role="menu"
      tabindex="-1"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      {#each menuGroups as group, index}
        <div class="group">
          {#each group as action}
            <button
              type="button"
              class:danger={action.tone === 'danger'}
              disabled={action.disabled}
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
    z-index: 20;
  }

  .menu-root:focus-within {
    z-index: 80;
  }

  .menu-trigger {
    display: inline-grid;
    place-items: center;
    border: 0;
    background: rgba(36, 38, 44, 0.44);
    color: rgba(244, 242, 247, 0.82);
    width: 2rem;
    height: 2rem;
    padding: 0;
    cursor: pointer;
    font: inherit;
    font-weight: 700;
    line-height: 1;
    border-radius: 0.7rem;
    opacity: 0.68;
    transform: scale(0.96);
    box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.06);
    transition:
      opacity var(--motion-fast) ease,
      transform var(--motion-fast) ease,
      background-color var(--motion-fast) ease,
      color var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease;
  }

  .menu-root:hover .menu-trigger,
  .menu-root:focus-within .menu-trigger,
  .menu-trigger.open {
    opacity: 0.92;
    transform: scale(1);
  }

  .menu-trigger:hover,
  .menu-trigger:focus-visible,
  .menu-trigger.open {
    background: rgba(132, 136, 146, 0.38);
    color: #f4f2f7;
    box-shadow: inset 0 0 0 1px rgba(255, 255, 255, 0.12);
  }

  .menu-trigger svg {
    width: 0.9rem;
    height: 0.9rem;
    fill: currentColor;
  }

  .menu {
    position: absolute;
    right: 0;
    top: calc(100% + 0.45rem);
    min-width: 13.5rem;
    padding: 0.5rem;
    border-radius: 0.95rem;
    background: var(--surface-glass);
    border: 1px solid var(--surface-border);
    box-shadow: var(--surface-shadow);
    backdrop-filter: blur(var(--ui-blur));
    z-index: 90;
  }

  .menu.side-right {
    top: auto;
    bottom: 0;
    right: auto;
    left: calc(100% + 0.55rem);
    transform: none;
  }

  .menu.above-right {
    top: auto;
    bottom: calc(100% + 0.45rem);
    right: 0;
    left: auto;
    transform: none;
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
    border-radius: 0.6rem;
    transition:
      background-color var(--motion-fast) ease,
      color var(--motion-fast) ease,
      opacity var(--motion-fast) ease;
  }

  .group button:hover {
    background: var(--surface-hover);
  }

  .group button.danger {
    color: #ffb0a6;
  }

  .group button:disabled {
    cursor: default;
    opacity: 0.55;
  }

  .separator {
    height: 1px;
    margin: 0.35rem 0;
    background: rgba(255, 255, 255, 0.08);
  }
</style>
