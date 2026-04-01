<script lang="ts">
import { getEmptyStateIllustration } from "$lib/assets";

export let kind: "emptyLibrary" | "noResults" | "errorState" = "emptyLibrary";
export let title = "";
export let message = "";
</script>

<div class="empty-state">
	<img class="art" src={getEmptyStateIllustration(kind)} alt="" loading="lazy" />
	{#if title}
		<h3>{title}</h3>
	{/if}
	{#if message}
		<p>{message}</p>
	{/if}
</div>

<style>
	.empty-state {
		display: grid;
		justify-items: center;
		gap: var(--space-3);
		padding: var(--space-5);
		text-align: center;
		border: 1px solid var(--surface-border-soft);
		border-radius: var(--radius-lg);
		background: var(--surface-card);
	}

	.art {
		width: min(100%, 20rem);
		height: auto;
		object-fit: contain;
	}

	h3,
	p {
		margin: 0;
	}

	h3 {
		color: var(--text-primary);
		font-size: 1rem;
	}

	p {
		color: var(--text-secondary);
		font-size: 0.84rem;
		max-width: 30rem;
	}
</style>
