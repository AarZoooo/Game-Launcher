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
    padding: 1.2rem 1.25rem;
    border-radius: 1.1rem;
    background: linear-gradient(180deg, rgba(255, 255, 255, 0.02), rgba(255, 255, 255, 0.01));
    border: 1px solid rgba(255, 255, 255, 0.08);
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
    border: 1px solid var(--surface-border);
    background: var(--surface-glass);
    color: #f4f2f7;
    min-height: 2.45rem;
    padding: 0.55rem 0.8rem;
    font: inherit;
    border-radius: 0.78rem;
    box-shadow: var(--surface-shadow);
    backdrop-filter: blur(var(--ui-blur));
    transition:
      background-color var(--motion-fast) ease,
      border-color var(--motion-fast) ease,
      box-shadow var(--motion-fast) ease;
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
    color: rgba(235, 232, 239, 0.68);
  }

  select:hover,
  input:hover,
  .radio-line:hover,
  select:focus,
  input:focus {
    background: rgba(255, 255, 255, 0.09);
    border-color: rgba(255, 255, 255, 0.14);
    outline: none;
  }

  @media (max-width: 820px) {
    .field {
      grid-template-columns: 1fr;
    }
  }
</style>
