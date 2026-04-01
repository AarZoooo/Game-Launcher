<script lang="ts">
import GlassSelect from "$lib/components/common/GlassSelect.svelte";
import { pageLabels, settingsSections } from "$lib/data/labels";
import type { SelectOption } from "$lib/types/UI";

function toOptions(options: string[] = []): SelectOption[] {
	return options.map((option) => ({ label: option, value: option }));
}
</script>

<div class="settings">
  <h1>{pageLabels.settings.title}</h1>
  {#each settingsSections as section}
    <section>
      <h2>{section.title}</h2>

      <div class="fields">
        {#each section.fields as field}
          <div class={`field ${field.type}`}>
            <span class="label">{field.label}</span>

            {#if field.type === 'select'}
              <GlassSelect value={field.value} options={toOptions(field.options)} ariaLabel={field.label} fullWidth />
            {:else if field.type === 'text'}
              <input class="field-control" aria-label={field.label} value={field.value} />
            {:else}
              <div class="radio-line field-control">{field.value}</div>
            {/if}
          </div>
        {/each}
      </div>
    </section>
  {/each}
</div>

<style>
  .settings {
    max-width: 50rem;
    display: flex;
    flex-direction: column;
    gap: var(--space-7);
  }

  section {
    display: flex;
    flex-direction: column;
    gap: var(--space-3);
    padding: var(--space-5);
    border-radius: var(--radius-lg);
    background: var(--surface-card);
    border: 1px solid var(--surface-border);
  }

  h1,
  h2 {
    margin: 0;
  }

  h1 {
    font: 700 1.9rem/1 var(--font-display);
  }

  h2 {
    font-size: 1rem;
  }

  .fields {
    display: grid;
    gap: var(--space-3);
  }

  .field {
    display: grid;
    grid-template-columns: minmax(14rem, 1fr) minmax(0, 16rem);
    align-items: center;
    gap: var(--space-4);
  }

  .label {
    color: var(--text-secondary);
    font-size: 0.8rem;
  }

  .radio-line {
    width: 100%;
    justify-content: flex-start;
  }

  .radio-line {
    display: flex;
    align-items: center;
    color: var(--text-secondary);
  }

  @media (max-width: 820px) {
    .field {
      grid-template-columns: 1fr;
    }
  }
</style>
