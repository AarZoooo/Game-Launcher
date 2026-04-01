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
  class="app-button"
  aria-label={ariaLabel}
  {disabled}
  class:quiet
  class:wide
  class:compact
  class:icon-first={iconFirst}
  class:icon-only={iconOnly}
  on:click
>
  <slot />
</button>
