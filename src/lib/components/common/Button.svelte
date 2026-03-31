<script lang="ts">
  import type { AccentTone } from '$lib/stores/libraryStore';

  export let type: 'button' | 'submit' = 'button';
  export let accent: AccentTone = 'gold';
  export let quiet = false;
  export let wide = false;
  export let compact = false;
  export let iconFirst = false;
</script>

<button
  {type}
  class:quiet
  class:wide
  class:compact
  class:icon-first={iconFirst}
  class:gold={accent === 'gold'}
  class:olive={accent === 'olive'}
  class:silver={accent === 'silver'}
  on:click
>
  <slot />
</button>

<style>
  button {
    border: 0;
    border-radius: 0.6rem;
    min-height: 2.55rem;
    padding: 0.7rem 1.45rem;
    font: inherit;
    font-weight: 700;
    letter-spacing: 0.01em;
    cursor: pointer;
    transition:
      transform 160ms ease,
      filter 160ms ease,
      background-color 160ms ease,
      color 160ms ease;
  }

  button:hover {
    filter: brightness(1.03);
    transform: translateY(-1px);
  }

  button:active {
    transform: translateY(0);
  }

  .wide {
    min-width: 6.25rem;
  }

  .compact {
    min-height: 2rem;
    padding: 0.42rem 0.9rem;
    border-radius: 0.5rem;
    font-size: 0.82rem;
  }

  .icon-first {
    display: inline-flex;
    align-items: center;
    gap: 0.42rem;
  }

  .gold {
    background: #b69b57;
    color: #fcfaf2;
  }

  .olive {
    background: #8a9a54;
    color: #f7f7f2;
  }

  .silver {
    background: #d6d7dc;
    color: #2c2e35;
  }

  .quiet {
    background: rgba(255, 255, 255, 0.08);
    color: #f4f2f7;
  }
</style>
