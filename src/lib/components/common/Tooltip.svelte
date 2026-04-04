<script lang="ts">
export let text: string;
export let position: "top" | "bottom" | "left" | "right" = "top";
</script>

<div class="tooltip-wrapper" data-tooltip-position={position}>
  <slot />
  <span class="tooltip-text" role="tooltip">{text}</span>
</div>

<style>
  .tooltip-wrapper {
    position: relative;
    display: inline-flex;
  }

  .tooltip-text {
    position: absolute;
    z-index: var(--z-menu);
    padding: var(--space-2) var(--space-3);
    border: 1px solid var(--surface-border);
    border-radius: var(--radius-control-sm);
    background: var(--surface-glass);
    backdrop-filter: blur(var(--blur-md));
    color: var(--text-primary);
    font-size: var(--font-size-caption-sm);
    font-weight: 400;
    white-space: nowrap;
    pointer-events: none;
    opacity: 0;
    transition: opacity var(--motion-fast) ease;
  }

  .tooltip-wrapper:hover .tooltip-text {
    opacity: 1;
  }

  [data-tooltip-position="top"] .tooltip-text {
    bottom: calc(100% + var(--space-2));
    left: 50%;
    transform: translateX(-50%);
  }

  [data-tooltip-position="bottom"] .tooltip-text {
    top: calc(100% + var(--space-2));
    left: 50%;
    transform: translateX(-50%);
  }

  [data-tooltip-position="left"] .tooltip-text {
    right: calc(100% + var(--space-2));
    top: 50%;
    transform: translateY(-50%);
  }

  [data-tooltip-position="right"] .tooltip-text {
    left: calc(100% + var(--space-2));
    top: 50%;
    transform: translateY(-50%);
  }
</style>
