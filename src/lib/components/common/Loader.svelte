<script lang="ts">
import {
	isGameRunning,
	performanceMode,
	resolveVariant,
} from "$lib/stores/uiStore";
import type { UIModeVariant } from "$lib/types/UI";

export let loading = false;
export let message = "Loading";
export let inline = false;
export let variant: UIModeVariant = "auto";
export let size: "sm" | "md" | "lg" = "md";

$: mode = resolveVariant(variant, $isGameRunning, $performanceMode);
</script>

{#if loading}
  <div class:overlay={!inline} class={`loader-shell ${mode} ${size}`}>
    <div class="loader" aria-hidden="true">
      <span></span>
      <span></span>
      <span></span>
      <span></span>
    </div>

    {#if message}
      <p>{message}</p>
    {/if}
  </div>
{/if}

<style>
  .loader-shell {
    --loader-accent: var(--interactive-primary-bg);
    --loader-track: var(--surface-border);
    --loader-bg: var(--surface-glass-strong);
    display: inline-flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    gap: 0.8rem;
    color: var(--text-primary);
    transition:
      opacity 160ms ease,
      transform 160ms ease,
      background-color 160ms ease;
  }

  .overlay {
    position: fixed;
    inset: 0;
    z-index: var(--z-loader);
    background: var(--color-overlay-1);
  }

  .loader-shell.normal.overlay {
    backdrop-filter: blur(var(--blur-sm));
  }

  .loader {
    position: relative;
    display: grid;
    place-items: center;
  }

  .loader span {
    position: absolute;
    border-radius: var(--radius-round);
    transform-origin: center;
  }

  .loader-shell.sm .loader {
    width: 1.6rem;
    height: 1.6rem;
  }

  .loader-shell.md .loader {
    width: 2.3rem;
    height: 2.3rem;
  }

  .loader-shell.lg .loader {
    width: 3rem;
    height: 3rem;
  }

  .normal .loader span:nth-child(1) {
    inset: 0;
    border: 0.18rem solid transparent;
    border-top-color: var(--loader-accent);
    border-right-color: rgb(var(--accent-rgb) / 0.5);
    animation: spin 900ms linear infinite;
  }

  .normal .loader span:nth-child(2) {
    inset: 0.35rem;
    border: 0.14rem solid var(--surface-border-soft);
  }

  .normal .loader span:nth-child(3),
  .normal .loader span:nth-child(4) {
    display: none;
  }

  .gaming .loader {
    width: 2.5rem;
    height: 0.55rem;
    grid-auto-flow: column;
    gap: 0.28rem;
  }

  .gaming .loader span {
    position: relative;
    width: 0.34rem;
    height: 0.34rem;
    background: var(--text-primary);
    opacity: 0.38;
    animation: stepPulse 900ms steps(1, end) infinite;
  }

  .gaming .loader span:nth-child(2) {
    animation-delay: 120ms;
  }

  .gaming .loader span:nth-child(3) {
    animation-delay: 240ms;
  }

  .gaming .loader span:nth-child(4) {
    animation-delay: 360ms;
  }

  .loader-shell p {
    margin: 0;
    font-size: 0.82rem;
    color: var(--text-secondary);
    letter-spacing: 0.01em;
  }

  .gaming.overlay {
    background: var(--color-overlay-1);
  }

  .gaming p {
    color: var(--text-muted);
  }

  @keyframes spin {
    to {
      transform: rotate(360deg);
    }
  }

  @keyframes stepPulse {
    0%,
    100% {
      opacity: 0.2;
      transform: scale(0.88);
    }

    50% {
      opacity: 0.95;
      transform: scale(1);
    }
  }

  @media (prefers-reduced-motion: reduce) {
    .loader-shell,
    .loader span {
      animation: none;
      transition: none;
    }

    .normal .loader span:nth-child(1) {
      animation: none;
    }

    .gaming .loader span {
      animation: none;
      opacity: 0.7;
    }
  }
</style>
