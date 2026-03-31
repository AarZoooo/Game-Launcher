<script lang="ts">
  import { settingsSections } from '$lib/utils/constants';
</script>

<div class="settings">
  {#each settingsSections as section}
    <section>
      <h2>{section.title}</h2>

      <div class="fields">
        {#each section.fields as field}
          <div class={`field ${field.type}`}>
            <span class="label">{field.label}</span>

            {#if field.type === 'select'}
              <select aria-label={field.label}>
                {#each field.options || [] as option}
                  <option selected={option === field.value}>{option}</option>
                {/each}
              </select>
            {:else if field.type === 'text'}
              <input aria-label={field.label} value={field.value} />
            {:else}
              <div class="radio-line">{field.value}</div>
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
    gap: 1.8rem;
  }

  section {
    display: flex;
    flex-direction: column;
    gap: 0.85rem;
  }

  h2 {
    margin: 0;
    font-size: 1rem;
  }

  .fields {
    display: grid;
    gap: 0.7rem;
  }

  .field {
    display: grid;
    grid-template-columns: minmax(14rem, 1fr) minmax(0, 16rem);
    align-items: center;
    gap: 1rem;
  }

  .label {
    color: rgba(230, 226, 235, 0.72);
    font-size: 0.8rem;
  }

  select,
  input,
  .radio-line {
    width: 100%;
    border: 1px solid rgba(255, 255, 255, 0.08);
    background: rgba(255, 255, 255, 0.08);
    color: #f4f2f7;
    min-height: 2rem;
    padding: 0.45rem 0.75rem;
    font: inherit;
  }

  .radio-line {
    display: flex;
    align-items: center;
    color: rgba(235, 232, 239, 0.68);
  }

  @media (max-width: 820px) {
    .field {
      grid-template-columns: 1fr;
    }
  }
</style>
