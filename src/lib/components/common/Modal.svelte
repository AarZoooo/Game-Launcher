<script lang="ts">
  import { createEventDispatcher } from 'svelte';
  import { isGameRunning, performanceMode, resolveVariant, type UIModeVariant } from '$lib/stores/uiStore';

  const dispatch = createEventDispatcher<{
    close: void;
    cancel: void;
    confirm: void;
  }>();

  export let open = false;
  export let title = '';
  export let message = '';
  export let content = '';
  export let confirmLabel = 'Confirm';
  export let cancelLabel = 'Cancel';
  export let showCancel = true;
  export let hideActions = false;
  export let closeOnBackdrop = true;
  export let variant: UIModeVariant = 'auto';
  export let size: 'sm' | 'md' | 'lg' = 'md';

  $: mode = resolveVariant(variant, $isGameRunning, $performanceMode);

  function close() {
    dispatch('close');
  }

  function cancel() {
    dispatch('cancel');
    close();
  }

  function confirm() {
    dispatch('confirm');
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
            <button class="secondary" type="button" on:click={cancel}>
              {cancelLabel}
            </button>
          {/if}

          <button class="primary" type="button" on:click={confirm}>
            {confirmLabel}
          </button>
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
    z-index: 80;
    display: grid;
    place-items: center;
    padding: 1rem;
    transition: opacity 140ms ease, background-color 140ms ease;
  }

  .backdrop.normal {
    background: rgba(10, 10, 13, 0.48);
    backdrop-filter: blur(8px);
  }

  .backdrop.gaming {
    background: rgba(10, 10, 13, 0.34);
  }

  .modal {
    width: min(100%, 32rem);
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: #1e1e1e;
    color: #f5f3f8;
    transform-origin: center;
    transition:
      transform 160ms ease,
      opacity 160ms ease,
      background-color 160ms ease,
      box-shadow 160ms ease;
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
    border-radius: 1rem;
    padding: 1.3rem 1.35rem 1.1rem;
    box-shadow:
      0 1.4rem 3.5rem rgba(0, 0, 0, 0.34),
      0 0.2rem 1rem rgba(0, 0, 0, 0.18);
    animation: modalIn 180ms ease;
  }

  .modal.gaming {
    border-radius: 0.55rem;
    padding: 1rem 1rem 0.9rem;
    background: #171717;
    box-shadow: 0 0.8rem 1.4rem rgba(0, 0, 0, 0.22);
    animation: modalInFast 90ms linear;
  }

  h2 {
    margin: 0 0 0.55rem;
    font-size: 1.06rem;
  }

  .message,
  .content {
    color: rgba(235, 232, 241, 0.76);
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
    gap: 0.7rem;
    margin-top: 1rem;
  }

  .actions button {
    border: 0;
    min-height: 2.25rem;
    padding: 0.65rem 1rem;
    font: inherit;
    font-weight: 700;
    cursor: pointer;
    transition: opacity 120ms ease, transform 120ms ease;
  }

  .actions button:hover {
    opacity: 0.95;
    transform: translateY(-1px);
  }

  .primary {
    background: #b69b57;
    color: #fcfaf2;
  }

  .secondary {
    background: rgba(255, 255, 255, 0.08);
    color: #f5f3f8;
  }

  .normal .actions button {
    border-radius: 0.7rem;
  }

  .gaming .actions button {
    border-radius: 0.45rem;
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
    .actions button {
      transition: none;
      animation: none;
    }
  }
</style>
