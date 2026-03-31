<script lang="ts">
  import type { AccentTone } from '$lib/stores/libraryStore';

  export let type: 'button' | 'submit' = 'button';
  export let accent: AccentTone = 'gold';
  export let quiet = false;
  export let wide = false;
  export let compact = false;
  export let iconFirst = false;
  export let iconOnly = false;
  export let disabled = false;
  export let ariaLabel: string | undefined = undefined;
</script>

<button
  {type}
  aria-label={ariaLabel}
  {disabled}
  class:quiet
  class:wide
  class:compact
  class:icon-first={iconFirst}
  class:icon-only={iconOnly}
  class:gold={accent === 'gold'}
  class:olive={accent === 'olive'}
  class:silver={accent === 'silver'}
  on:click
>
  <slot />
</button>

<style>
  button {
    display: inline-flex;
    align-items: center;
    justify-content: center;
    gap: 0.48rem;
    border: 0;
    border-radius: 0.8rem;
    min-height: 2.45rem;
    padding: 0.65rem 1.1rem;
    font: inherit;
    font-weight: 700;
    letter-spacing: 0.01em;
    cursor: pointer;
    transition:
      transform var(--motion-fast) ease,
      filter var(--motion-fast) ease,
      background-color var(--motion-fast) ease,
      color var(--motion-fast) ease,
      opacity var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease;
  }

  button:hover:not(:disabled) {
    filter: brightness(1.03);
    transform: translateY(-1px);
  }

  button:active:not(:disabled) {
    transform: translateY(0);
  }

  .wide {
    min-width: 6.25rem;
  }

  .compact {
    min-height: 2.35rem;
    padding: 0.55rem 0.95rem;
    border-radius: 0.72rem;
    font-size: 0.84rem;
  }

  .icon-only {
    width: 2.35rem;
    min-width: 2.35rem;
    padding: 0;
  }

  .icon-first {
    gap: 0.45rem;
  }

  .icon-first :global(svg) {
    width: 0.95rem;
    height: 0.95rem;
    flex: 0 0 auto;
  }

  .gold {
    background: #b69b57;
    color: #fcfaf2;
    box-shadow: 0 0.8rem 1.8rem rgba(182, 155, 87, 0.18);
  }

  .olive {
    background: #8a9a54;
    color: #f7f7f2;
    box-shadow: 0 0.8rem 1.8rem rgba(138, 154, 84, 0.16);
  }

  .silver {
    background: #d6d7dc;
    color: #2c2e35;
    box-shadow: 0 0.8rem 1.8rem rgba(214, 215, 220, 0.14);
  }

  .quiet {
    background: var(--field-background);
    color: #f4f2f7;
    box-shadow: inset 0 0 0 1px var(--field-border);
  }

  button:disabled {
    cursor: default;
    opacity: 0.58;
    transform: none;
    filter: none;
    box-shadow: none;
  }
</style>
