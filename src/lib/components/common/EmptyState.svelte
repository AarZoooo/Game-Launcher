<script lang="ts">
import { getEmptyStateIllustration } from "$lib/assets";
import Icon from "$lib/components/common/Icon.svelte";

export let kind: "emptyLibrary" | "noResults" | "errorState" | undefined =
	undefined;
export let title = "";
export let subtitle = "";
export let message = "";
export let icon: string | undefined = undefined;
export let variant: "card" | "full-width" = "full-width";

$: resolvedIllustration =
	!icon && kind ? getEmptyStateIllustration(kind) : undefined;
$: resolvedSubtitle = subtitle || message;
</script>

<div class="empty-state" class:card={variant === "card"}>
	<div class="empty-state-shell">
		{#if icon}
			<div class="icon-shell" aria-hidden="true">
				<Icon src={icon} size="1.4rem" />
			</div>
		{:else if resolvedIllustration}
			<img class="art" src={resolvedIllustration} alt="" loading="lazy" />
		{/if}

		<div class="copy">
			{#if title}
				<h3>{title}</h3>
			{/if}
			{#if resolvedSubtitle}
				<p>{resolvedSubtitle}</p>
			{/if}
		</div>
	</div>
</div>

<style>
	.empty-state {
		position: relative;
		overflow: hidden;
		border: 1px solid var(--surface-border);
		border-radius: var(--radius-panel);
		background:
			linear-gradient(180deg, rgb(255 255 255 / 0.04), rgb(255 255 255 / 0.015)),
			var(--surface-glass);
		box-shadow: var(--shadow-inset);
		backdrop-filter: blur(var(--blur-md));
	}

	.empty-state::before {
		content: "";
		position: absolute;
		inset: auto -10% -35% 20%;
		height: 60%;
		background: radial-gradient(circle, rgb(var(--accent-rgb) / 0.16), transparent 70%);
		filter: blur(32px);
		opacity: 0.6;
		pointer-events: none;
	}

	.empty-state-shell {
		position: relative;
		z-index: 1;
		display: grid;
		justify-items: center;
		gap: var(--space-4);
		min-height: 11rem;
		padding: var(--space-6);
		text-align: center;
	}

	.empty-state.card {
		width: min(100%, var(--card-width-md));
		border-radius: var(--radius-card);
		background:
			linear-gradient(180deg, rgb(255 255 255 / 0.05), rgb(255 255 255 / 0.015)),
			linear-gradient(180deg, rgb(6 9 13 / 0.18), rgb(6 9 13 / 0.5)),
			var(--surface-glass);
	}

	.empty-state.card .empty-state-shell {
		min-height: var(--card-height-sm);
		padding: var(--space-5) var(--space-4);
		align-content: end;
	}

	.icon-shell {
		display: grid;
		place-items: center;
		width: 3.5rem;
		height: 3.5rem;
		border: 1px solid var(--surface-border-soft);
		border-radius: var(--radius-round);
		background: rgb(255 255 255 / 0.06);
		color: var(--text-primary);
		box-shadow: var(--shadow-sm);
	}

	.art {
		width: min(100%, 20rem);
		height: auto;
		object-fit: contain;
		opacity: 0.92;
	}

	.copy {
		display: grid;
		gap: var(--space-2);
		max-width: 30rem;
	}

	h3,
	p {
		margin: 0;
	}

	h3 {
		color: var(--text-primary);
		font-size: var(--font-size-body);
		font-weight: 650;
	}

	p {
		color: var(--text-secondary);
		font-size: var(--font-size-control);
		line-height: 1.5;
	}
</style>
