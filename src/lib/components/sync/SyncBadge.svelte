<script lang="ts">
export let status = "Up to date";
</script>

<span class="badge">{status}</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: var(--space-2);
    color: var(--color-success-1);
    font-size: 0.72rem;
    font-weight: 600;
  }

  .badge::before {
    content: '';
    width: 0.48rem;
    height: 0.48rem;
    border-radius: var(--radius-pill);
    background: var(--color-success-1);
    box-shadow: 0 0 0.6rem rgb(135 215 107 / 0.7);
  }
</style>
