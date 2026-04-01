<script lang="ts">
import { createEventDispatcher, onDestroy, onMount } from "svelte";
import { appIcons } from "$lib/assets";
import Icon from "$lib/components/common/Icon.svelte";
import { getGameMenuGroups } from "$lib/data/gameMenu";
import { pageLabels } from "$lib/data/labels";
import {
	activeGameId,
	activeMenuKey,
	isGameRunning,
	uiStore,
} from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";
import type {
	GameMenuContext,
	MenuAction,
	MenuPlacement,
} from "$lib/types/Menu";

const dispatch = createEventDispatcher<{
	action: { id: MenuAction["id"]; game: Game };
}>();

export let game: Game;
export let context: GameMenuContext = "library";
export let placement: MenuPlacement = "below-right";

let root: HTMLDivElement;
let menuKey = "";
let closeTimeout: ReturnType<typeof setTimeout> | null = null;
$: isActiveGame = $isGameRunning && $activeGameId === game.id;
$: if (!menuKey && game?.id) {
	menuKey = `${game.id}-${Math.random().toString(36).slice(2, 10)}`;
}
$: open = menuKey !== "" && $activeMenuKey === menuKey;
$: menuGroups = getGameMenuGroups(context, { game, isActiveGame });

function select(actionId: string) {
	uiStore.closeOpenMenu();
	dispatch("action", { id: actionId as MenuAction["id"], game });
}

function toggleMenu() {
	if (!menuKey) return;

	clearCloseTimeout();

	if (open) {
		uiStore.closeOpenMenu();
		return;
	}

	uiStore.setOpenMenu(menuKey);
}

function clearCloseTimeout() {
	if (closeTimeout) {
		clearTimeout(closeTimeout);
		closeTimeout = null;
	}
}

function scheduleClose() {
	clearCloseTimeout();
	closeTimeout = setTimeout(() => {
		uiStore.closeOpenMenu();
		closeTimeout = null;
	}, 120);
}

onMount(() => {
	const close = (event: MouseEvent) => {
		if (open && root && !root.contains(event.target as Node)) {
			uiStore.closeOpenMenu();
		}
	};

	window.addEventListener("click", close);
	return () => {
		window.removeEventListener("click", close);
	};
});

onDestroy(() => {
	clearCloseTimeout();

	if (open) {
		uiStore.closeOpenMenu();
	}
});
</script>

<div
  class="menu-root"
  bind:this={root}
  role="presentation"
  on:mouseenter={clearCloseTimeout}
  on:mouseleave={scheduleClose}
>
  <button
    type="button"
    class:open
    class="menu-trigger"
    aria-label={pageLabels.common.openMenuFor(game.title)}
    on:click|stopPropagation={toggleMenu}
  >
    <Icon src={appIcons.ui.more} size="0.9rem" />
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
    z-index: var(--z-card-hover);
  }

  .menu-root:focus-within {
    z-index: var(--z-menu);
  }

  .menu-trigger {
    display: inline-grid;
    place-items: center;
    border: 0;
    background: var(--surface-glass);
    color: var(--text-primary);
    width: 2rem;
    height: 2rem;
    padding: 0;
    cursor: pointer;
    font: inherit;
    font-weight: 700;
    line-height: 1;
    border-radius: var(--radius-sm);
    opacity: 0.26;
    transform: scale(0.96);
    box-shadow: inset 0 0 0 1px var(--surface-border-soft);
    backdrop-filter: blur(10px);
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
    background: var(--surface-hover);
    color: var(--text-primary);
    box-shadow: inset 0 0 0 1px var(--surface-border);
  }

  .menu {
    position: absolute;
    right: 0;
    top: calc(100% + 0.45rem);
    min-width: 10.5rem;
    max-width: 11.5rem;
    padding: var(--space-1);
    border-radius: var(--radius-lg);
    background: var(--surface-glass);
    border: 1px solid var(--surface-border);
    box-shadow: var(--shadow-md);
    backdrop-filter: blur(10px);
    z-index: var(--z-menu);
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
    gap: var(--space-1);
  }

  .group button {
    border: 0;
    background: transparent;
    color: var(--text-primary);
    text-align: left;
    width: 100%;
    padding: var(--space-2) var(--space-3);
    font: inherit;
    font-size: 0.72rem;
    line-height: 1.2;
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition:
      background-color var(--motion-fast) ease,
      color var(--motion-fast) ease,
      opacity var(--motion-fast) ease;
  }

  .group button:hover {
    background: var(--surface-hover);
  }

  .group button.danger {
    color: var(--color-danger-1);
  }

  .group button:disabled {
    cursor: default;
    opacity: 0.55;
  }

  .separator {
    height: 1px;
    margin: var(--space-2) 0;
    background: var(--surface-border-soft);
  }
</style>
