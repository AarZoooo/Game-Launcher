<script lang="ts">
import { createEventDispatcher, onDestroy, onMount } from "svelte";
import type { SelectOption } from "$lib/types/UI";

const dispatch = createEventDispatcher<{
	change: { value: string };
}>();

export let value = "";
export let options: SelectOption[] = [];
export let ariaLabel = "";
export let placeholder = "";
export let disabled = false;
export let fullWidth = false;

let root: HTMLDivElement;
let open = false;
let closeTimeout: ReturnType<typeof setTimeout> | null = null;

$: selectedOption =
	options.find((option) => option.value === value) ||
	options.find((option) => option.value === "") ||
	options[0];

$: selectedLabel = selectedOption?.label || placeholder;

function toggle() {
	if (disabled) return;
	open = !open;
}

function close() {
	open = false;
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
		close();
		closeTimeout = null;
	}, 120);
}

function selectOption(option: SelectOption) {
	value = option.value;
	dispatch("change", { value });
	close();
}

onMount(() => {
	const handleWindowClick = (event: MouseEvent) => {
		if (open && root && !root.contains(event.target as Node)) {
			close();
		}
	};

	window.addEventListener("click", handleWindowClick);
	return () => window.removeEventListener("click", handleWindowClick);
});

onDestroy(() => {
	clearCloseTimeout();
	close();
});
</script>

<div
  class:full-width={fullWidth}
  class="select-root"
  bind:this={root}
  role="presentation"
  on:mouseenter={clearCloseTimeout}
  on:mouseleave={scheduleClose}
>
  <button
    type="button"
    class:open
    class="select-trigger field-control glass-dropdown"
    aria-expanded={open}
    aria-haspopup="listbox"
    aria-label={ariaLabel}
    disabled={disabled}
    on:click|stopPropagation={toggle}
  >
    <span class="select-value">{selectedLabel}</span>
    <span class="select-chevron" aria-hidden="true"></span>
  </button>

  {#if open}
    <div
      class="select-menu"
      role="listbox"
      tabindex="-1"
      on:click|stopPropagation
      on:keydown|stopPropagation
    >
      {#each options as option}
        <button
          type="button"
          class:selected={option.value === value}
          class="select-option"
          role="option"
          aria-selected={option.value === value}
          on:click={() => selectOption(option)}
        >
          {option.label}
        </button>
      {/each}
    </div>
  {/if}
</div>
