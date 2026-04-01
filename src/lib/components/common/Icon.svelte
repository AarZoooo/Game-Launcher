<script lang="ts">
import type { appIcons } from "$lib/assets";

type IconGroup = keyof typeof appIcons;
type IconName<T extends IconGroup> = keyof (typeof appIcons)[T];

export let src: string;
export let label: string | undefined = undefined;
export let size = "1rem";

let element: HTMLSpanElement;

$: if (element) {
	element.style.setProperty("--icon-src", `url("${src}")`);
	element.style.setProperty("--icon-size", size);
}
</script>

<span bind:this={element} class="icon" aria-hidden={label ? undefined : "true"} aria-label={label}></span>

<style>
	.icon {
		display: inline-block;
		width: var(--icon-size);
		height: var(--icon-size);
		background-color: currentColor;
		mask-image: var(--icon-src);
		mask-repeat: no-repeat;
		mask-position: center;
		mask-size: contain;
		-webkit-mask-image: var(--icon-src);
		-webkit-mask-repeat: no-repeat;
		-webkit-mask-position: center;
		-webkit-mask-size: contain;
		flex: 0 0 auto;
	}
</style>
