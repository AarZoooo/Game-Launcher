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
  <div class={`backdrop ${mode}`} on:click={handleBackdropClick} on:keydown={(event) => event.key === 'Escape' && close()} role="presentation">
    <div
      class={`modal ${mode} ${size}`}
      role="dialog"
      tabindex="-1"
      aria-modal="true"
      aria-labelledby="modal-title"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      {#if title}
        <h2 id="modal-title">{title}</h2>
      {/if}

      {#if message}
        <p class="message">{message}</p>
      {/if}

      {#if content}
        <div class="content">{content}</div>
      {/if}

      <slot />

      {#if !hideActions}
        <div class="actions">
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

<style>
  .backdrop {
    position: fixed;
    inset: 0;
    z-index: var(--z-modal);
    display: grid;
    place-items: center;
    padding: var(--space-4);
    transition: opacity 140ms ease, background-color 140ms ease;
  }

  .backdrop.normal {
    background: rgba(10, 10, 13, 0.48);
    backdrop-filter: blur(calc(var(--ui-blur) * 0.8));
  }

  .backdrop.gaming {
    background: rgba(10, 10, 13, 0.34);
  }

  .modal {
    width: min(100%, 32rem);
    border: 1px solid var(--surface-border);
    background: rgba(30, 30, 30, 0.6);
    color: var(--text-primary);
    transform-origin: center;
    transition:
      transform var(--motion-fast) ease,
      opacity var(--motion-fast) ease,
      background-color var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease;
    backdrop-filter: blur(10px);
  }

  .modal.sm {
    max-width: 24rem;
  }

  .modal.md {
    max-width: 32rem;
  }

  .modal.lg {
    max-width: 42rem;
  }

  .modal.normal {
    border-radius: var(--radius-lg);
    padding: var(--space-5);
    box-shadow: var(--shadow-md);
    animation: modalIn 180ms ease;
  }

  .modal.gaming {
    border-radius: var(--radius-sm);
    padding: var(--space-4);
    background: var(--surface-glass-strong);
    box-shadow: var(--shadow-sm);
    animation: modalInFast 90ms linear;
  }

  h2 {
    margin: 0 0 0.55rem;
    font-size: 1.06rem;
  }

  .message,
  .content {
    color: var(--text-secondary);
    font-size: 0.86rem;
    line-height: 1.5;
  }

  .message {
    margin: 0 0 0.7rem;
  }

  .content {
    margin: 0 0 0.9rem;
    white-space: pre-wrap;
  }

  .actions {
    display: flex;
    justify-content: flex-end;
    gap: var(--space-3);
    margin-top: var(--space-4);
  }

  @keyframes modalIn {
    from {
      opacity: 0;
      transform: scale(0.97) translateY(0.4rem);
    }

    to {
      opacity: 1;
      transform: scale(1) translateY(0);
    }
  }

  @keyframes modalInFast {
    from {
      opacity: 0;
      transform: translateY(0.2rem);
    }

    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .backdrop,
    .modal,
    .actions :global(button) {
      transition: none;
      animation: none;
    }
  }
</style>
