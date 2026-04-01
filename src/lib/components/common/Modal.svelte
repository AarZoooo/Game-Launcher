<script lang="ts">
import { createEventDispatcher } from "svelte";
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

$: mode = resolveVariant(variant, $isGameRunning, $performanceMode);

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
</script>

{#if open}
  <div class={`app-modal-backdrop ${mode}`} on:click={handleBackdropClick} on:keydown={(event) => event.key === 'Escape' && close()} role="presentation">
    <div
      class={`app-modal-shell ${mode} ${size}`}
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="modal-title"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      {#if title}
        <h2 id="modal-title" class="app-modal-title">{title}</h2>
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
