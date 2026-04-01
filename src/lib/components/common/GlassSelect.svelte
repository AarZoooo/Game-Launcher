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
	close();
});
</script>

<div
  class:full-width={fullWidth}
  class="select-root"
  bind:this={root}
  role="presentation"
  on:mouseleave={close}
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
    <span class="chevron" aria-hidden="true"></span>
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
          class="option"
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

<style>
  .select-root {
    position: relative;
    min-width: 7.2rem;
  }

  .select-root.full-width {
    width: 100%;
  }

  .select-trigger {
    width: 100%;
    min-height: var(--control-height-md);
    display: flex;
    align-items: center;
    justify-content: space-between;
    gap: var(--space-3);
    padding: 0.6rem 0.85rem;
    text-align: left;
    cursor: pointer;
  }

  .select-trigger:disabled {
    cursor: default;
    opacity: 0.6;
  }

  .select-value {
    min-width: 0;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .chevron {
    flex: 0 0 auto;
    width: 0.5rem;
    height: 0.5rem;
    border-right: 2px solid rgb(244 242 247 / 0.8);
    border-bottom: 2px solid rgb(244 242 247 / 0.8);
    transform: translateY(-0.08rem) rotate(45deg);
    transition: transform var(--motion-fast) ease;
  }

  .select-trigger.open .chevron {
    transform: translateY(0.08rem) rotate(-135deg);
  }

  .select-menu {
    position: absolute;
    top: calc(100% + 0.45rem);
    right: 0;
    left: 0;
    padding: var(--space-1);
    border-radius: var(--radius-lg);
    background: rgba(30, 30, 30, 0.6);
    border: 1px solid var(--surface-border);
    box-shadow: var(--shadow-md);
    backdrop-filter: blur(10px);
    z-index: var(--z-menu);
  }

  .option {
    width: 100%;
    border: 0;
    background: transparent;
    color: var(--text-primary);
    text-align: left;
    padding: var(--space-2) var(--space-3);
    font: inherit;
    font-size: 0.78rem;
    cursor: pointer;
    border-radius: var(--radius-sm);
    transition:
      background-color var(--motion-fast) ease,
      color var(--motion-fast) ease,
      opacity var(--motion-fast) ease;
  }

  .option:hover,
  .option.selected {
    background: rgba(132, 136, 146, 0.24);
  }
</style>
