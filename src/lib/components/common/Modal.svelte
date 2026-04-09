<script lang="ts">
import { createEventDispatcher, onDestroy, tick } from "svelte";
import Button from "$lib/components/common/Button.svelte";
import { pageLabels } from "$lib/data/labels";
import {
	isGameRunning,
	performanceMode,
	resolveVariant,
} from "$lib/stores/uiStore";
import type { UIModeVariant } from "$lib/types/UI";

const dispatch = createEventDispatcher<{
	close: void;
	cancel: void;
	confirm: void;
}>();

export let open = false;
export let title = "";
export let message = "";
export let content = "";
export let confirmLabel = pageLabels.common.confirm;
export let cancelLabel = pageLabels.common.cancel;
export let showCancel = true;
export let hideActions = false;
export let closeOnBackdrop = true;
export let variant: UIModeVariant = "auto";
export let size: "sm" | "md" | "lg" = "md";
export let showCloseButton = true;
export let closeLabel = pageLabels.common.cancel;

const titleId = `modal-title-${Math.random().toString(36).slice(2, 10)}`;
const focusableSelector =
	'button:not([disabled]), [href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), [tabindex]:not([tabindex="-1"])';

let modalElement: HTMLDivElement | undefined;
let previouslyFocused: HTMLElement | null = null;
let wasOpen = false;

$: mode = resolveVariant(variant, $isGameRunning, $performanceMode);
$: if (open && !wasOpen) {
	wasOpen = true;
	if (typeof document !== "undefined") {
		previouslyFocused =
			document.activeElement instanceof HTMLElement
				? document.activeElement
				: null;
		document.body.style.overflow = "hidden";
	}
	void focusModal();
}

$: if (!open && wasOpen) {
	wasOpen = false;
	if (typeof document !== "undefined") {
		document.body.style.overflow = "";
	}
	previouslyFocused?.focus();
	previouslyFocused = null;
}

function close() {
	dispatch("close");
}

function cancel() {
	dispatch("cancel");
	close();
}

function confirm() {
	dispatch("confirm");
}

function handleBackdropClick() {
	if (closeOnBackdrop) {
		close();
	}
}

function getFocusableElements() {
	if (!modalElement) return [];

	return Array.from(
		modalElement.querySelectorAll<HTMLElement>(focusableSelector),
	).filter((element) => {
		if (element.hasAttribute("disabled")) return false;
		if (element.getAttribute("aria-hidden") === "true") return false;
		return !element.hasAttribute("hidden");
	});
}

async function focusModal() {
	await tick();

	if (!modalElement) return;

	const [firstFocusable] = getFocusableElements();
	(firstFocusable || modalElement).focus();
}

function handleModalKeydown(event: KeyboardEvent) {
	if (event.key === "Escape") {
		event.preventDefault();
		close();
		return;
	}

	if (event.key !== "Tab") {
		return;
	}

	const focusableElements = getFocusableElements();

	if (!focusableElements.length) {
		event.preventDefault();
		modalElement?.focus();
		return;
	}

	const firstFocusable = focusableElements[0];
	const lastFocusable = focusableElements[focusableElements.length - 1];
	const activeElement =
		typeof document !== "undefined" ? document.activeElement : null;

	if (event.shiftKey && activeElement === firstFocusable) {
		event.preventDefault();
		lastFocusable.focus();
		return;
	}

	if (!event.shiftKey && activeElement === lastFocusable) {
		event.preventDefault();
		firstFocusable.focus();
	}
}

onDestroy(() => {
	if (typeof document !== "undefined") {
		document.body.style.overflow = "";
	}
});
</script>

{#if open}
  <div
    class={`app-modal-backdrop ${mode}`}
    on:click={handleBackdropClick}
    role="presentation"
  >
    <div
      bind:this={modalElement}
      class={`app-modal-shell ${mode} ${size}`}
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby={title ? titleId : undefined}
      on:click|stopPropagation
      on:keydown|stopPropagation={handleModalKeydown}
    >
      {#if showCloseButton}
        <button
          type="button"
          class="app-modal-close"
          aria-label={closeLabel}
          on:click={close}
        >
          <span aria-hidden="true">&times;</span>
        </button>
      {/if}

      {#if title}
        <h2 id={titleId} class="app-modal-title">{title}</h2>
      {/if}

      {#if message}
        <p class="app-modal-message">{message}</p>
      {/if}

      {#if content}
        <div class="app-modal-content">{content}</div>
      {/if}

      <slot />

      {#if !hideActions}
        <div class="app-modal-actions">
          {#if showCancel}
            <Button quiet compact type="button" on:click={cancel}>{cancelLabel}</Button>
          {/if}

          <Button compact type="button" on:click={confirm}>{confirmLabel}</Button>
        </div>
      {/if}

      <slot name="footer" />
    </div>
  </div>
{/if}
