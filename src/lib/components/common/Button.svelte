<script lang="ts">
import type { AccentTone } from "$lib/types/Game";
import { resolveAccentPresentation } from "$lib/utils/accent";

export let type: "button" | "submit" = "button";
export let accent: AccentTone = "gold";
export let accentColor: string | undefined = undefined;
export let quiet = false;
export let wide = false;
export let compact = false;
export let iconFirst = false;
export let iconOnly = false;
export let disabled = false;
export let ariaLabel: string | undefined = undefined;

let element: HTMLButtonElement;

$: accentPresentation = resolveAccentPresentation({
	accent,
	accentColor,
	accentHex: accentColor,
});

$: if (element) {
	element.style.setProperty("--button-accent-rgb", accentPresentation.rgb);
	element.style.setProperty("--button-accent-text", accentPresentation.text);
}
</script>

<button
  bind:this={element}
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
    gap: var(--space-2);
    border: 0;
    border-radius: var(--radius-md);
    min-height: var(--control-height-md);
    padding: 0 var(--space-4);
    font: inherit;
    font-weight: 700;
    letter-spacing: 0.01em;
    background: rgb(var(--button-accent-rgb));
    color: var(--button-accent-text);
    box-shadow: var(--shadow-sm);
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
    filter: brightness(1.04);
    transform: translateY(-1px);
  }

  button:active:not(:disabled) {
    transform: translateY(0);
  }

  .wide {
    min-width: 6.25rem;
  }

  .compact {
    min-height: var(--control-height-sm);
    padding: 0 var(--space-3);
    border-radius: var(--radius-sm);
    font-size: 0.84rem;
  }

  .icon-only {
    width: var(--control-height-sm);
    min-width: var(--control-height-sm);
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
    background: rgb(var(--button-accent-rgb));
  }

  .olive {
    background: rgb(var(--button-accent-rgb));
  }

  .silver {
    background: rgb(var(--button-accent-rgb));
  }

  .quiet {
    background: var(--interactive-secondary-bg);
    color: var(--interactive-secondary-text);
    box-shadow: inset 0 0 0 1px var(--field-border);
  }

  button:disabled {
    cursor: default;
    background: var(--interactive-disabled-bg);
    color: var(--interactive-disabled-text);
    opacity: 1;
    transform: none;
    filter: none;
    box-shadow: none;
  }
</style>
