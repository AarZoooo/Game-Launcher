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
      class="menu-panel"
      role="menu"
      tabindex="-1"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      {#each menuGroups as group, index}
        <div class="menu-group">
          {#each group as action}
            <button
              type="button"
              class="menu-item"
              class:danger={action.tone === 'danger'}
              disabled={action.disabled}
              on:click={() => select(action.id)}
            >
              {action.label}
            </button>
          {/each}
        </div>

        {#if index < menuGroups.length - 1}
          <div class="menu-separator"></div>
        {/if}
      {/each}
    </div>
  {/if}
</div>
