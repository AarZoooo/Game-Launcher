<script lang="ts">
export let status = "Up to date";
</script>

<span class="badge">{status}</span>

<style>
  .badge {
    display: inline-flex;
    align-items: center;
    gap: 0.4rem;
    color: #e4f1d6;
    font-size: 0.72rem;
    font-weight: 600;
  }

  .badge::before {
    content: '';
    width: 0.48rem;
    height: 0.48rem;
    border-radius: 999px;
    background: #87d76b;
    box-shadow: 0 0 0.6rem rgba(135, 215, 107, 0.7);
  }
</style>
