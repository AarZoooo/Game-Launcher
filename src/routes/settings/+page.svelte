<script lang="ts">
import { pageLabels, settingsSections } from "$lib/data/labels";
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
              <select class="field-control glass-dropdown" aria-label={field.label}>
                {#each field.options || [] as option}
                  <option selected={option === field.value}>{option}</option>
                {/each}
              </select>
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

  select {
    appearance: none;
    background-image:
      linear-gradient(45deg, transparent 50%, rgba(244, 242, 247, 0.8) 50%),
      linear-gradient(135deg, rgba(244, 242, 247, 0.8) 50%, transparent 50%);
    background-position:
      calc(100% - 1.05rem) calc(50% - 0.12rem),
      calc(100% - 0.72rem) calc(50% - 0.12rem);
    background-size: 0.4rem 0.4rem;
    background-repeat: no-repeat;
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
