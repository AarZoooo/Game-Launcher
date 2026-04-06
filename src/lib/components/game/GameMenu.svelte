<script lang="ts">
import { createEventDispatcher, onDestroy, onMount, tick } from "svelte";
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
import { portal } from "$lib/utils/portal";

const dispatch = createEventDispatcher<{
	action: { id: MenuAction["id"]; game: Game };
}>();

export let game: Game;
export let context: GameMenuContext = "library";
export let placement: MenuPlacement = "below-right";

let root: HTMLDivElement;
let trigger: HTMLButtonElement;
let menuPanel: HTMLDivElement;
let menuKey = "";
let portalTarget = "#portal-root";
let closeTimeout: ReturnType<typeof setTimeout> | null = null;
let viewportListeners: Array<() => void> = [];
let isFloating = false;
let menuStyle = "";

$: isActiveGame = $isGameRunning && $activeGameId === game.id;
$: if (!menuKey && game?.id) {
	menuKey = `${game.id}-${Math.random().toString(36).slice(2, 10)}`;
}
$: open = menuKey !== "" && $activeMenuKey === menuKey;
$: menuGroups = getGameMenuGroups(context, { game, isActiveGame });
$: if (open && !isFloating) {
	isFloating = true;
	void openFloatingMenu();
}
$: if (!open && isFloating) {
	isFloating = false;
	teardownViewportListeners();
	menuStyle = "";
}

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

function resolveLengthToken(tokenName: string, fallbackPx: number) {
	if (typeof window === "undefined") {
		return fallbackPx;
	}

	const rawValue = getComputedStyle(document.documentElement)
		.getPropertyValue(tokenName)
		.trim();

	if (!rawValue) {
		return fallbackPx;
	}

	if (rawValue.endsWith("px")) {
		const parsed = parseFloat(rawValue);
		return Number.isFinite(parsed) ? parsed : fallbackPx;
	}

	if (rawValue.endsWith("rem")) {
		const parsed = parseFloat(rawValue);
		if (!Number.isFinite(parsed)) {
			return fallbackPx;
		}

		const rootFontSize = parseFloat(
			getComputedStyle(document.documentElement).fontSize,
		);
		return parsed * (Number.isFinite(rootFontSize) ? rootFontSize : 16);
	}

	const parsed = parseFloat(rawValue);
	return Number.isFinite(parsed) ? parsed : fallbackPx;
}

function updateMenuPosition() {
	if (!open || !trigger || !menuPanel || typeof window === "undefined") {
		return;
	}

	const triggerRect = trigger.getBoundingClientRect();
	const panelRect = menuPanel.getBoundingClientRect();
	const gap = resolveLengthToken("--space-2", 8);
	const viewportPadding = resolveLengthToken("--space-3", 12);

	let top = triggerRect.bottom + gap;
	let left = triggerRect.right - panelRect.width;

	if (placement === "above-right") {
		top = triggerRect.top - panelRect.height - gap;
	}

	if (placement === "side-right") {
		top = triggerRect.bottom - panelRect.height;
		left = triggerRect.right + gap;
	}

	const maxTop = Math.max(
		viewportPadding,
		window.innerHeight - panelRect.height - viewportPadding,
	);
	const maxLeft = Math.max(
		viewportPadding,
		window.innerWidth - panelRect.width - viewportPadding,
	);

	top = Math.min(Math.max(viewportPadding, top), maxTop);
	left = Math.min(Math.max(viewportPadding, left), maxLeft);

	menuStyle = `top: ${top}px; left: ${left}px;`;
}

function teardownViewportListeners() {
	for (const dispose of viewportListeners) {
		dispose();
	}

	viewportListeners = [];
}

async function openFloatingMenu() {
	await tick();
	updateMenuPosition();
	teardownViewportListeners();

	if (typeof window === "undefined") {
		return;
	}

	const handleViewportChange = () => {
		updateMenuPosition();
	};

	window.addEventListener("resize", handleViewportChange);
	window.addEventListener("scroll", handleViewportChange, true);
	viewportListeners = [
		() => window.removeEventListener("resize", handleViewportChange),
		() => window.removeEventListener("scroll", handleViewportChange, true),
	];
}

onMount(() => {
	const close = (event: MouseEvent) => {
		const target = event.target as Node;
		const clickedTrigger = Boolean(trigger?.contains(target));
		const clickedMenu = Boolean(menuPanel?.contains(target));

		if (open && !clickedTrigger && !clickedMenu) {
			uiStore.closeOpenMenu();
		}
	};

	const closeOnEscape = (event: KeyboardEvent) => {
		if (event.key === "Escape" && open) {
			uiStore.closeOpenMenu();
		}
	};

	window.addEventListener("click", close);
	window.addEventListener("keydown", closeOnEscape);
	return () => {
		window.removeEventListener("click", close);
		window.removeEventListener("keydown", closeOnEscape);
	};
});

onDestroy(() => {
	clearCloseTimeout();
	teardownViewportListeners();

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
    bind:this={trigger}
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
      bind:this={menuPanel}
      use:portal={portalTarget}
      class="menu-panel menu-panel-floating"
      role="menu"
      tabindex="-1"
      style={menuStyle}
      on:click|stopPropagation
      on:keydown|stopPropagation
      on:mouseenter={clearCloseTimeout}
      on:mouseleave={scheduleClose}
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
