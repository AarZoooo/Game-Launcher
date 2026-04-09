<script lang="ts">
import { appIcons } from "$lib/assets";
import EmptyState from "$lib/components/common/EmptyState.svelte";
import GameCardSkeleton from "$lib/components/game/GameCardSkeleton.svelte";
import HeroBanner from "$lib/components/game/HeroBanner.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";

export let game: Game | undefined = undefined;
export let loading = false;
</script>

{#if loading}
	<section class="continue-empty">
		<p class="eyebrow">{pageLabels.continuePlaying.eyebrow}</p>
		<div class="placeholder-row" aria-hidden="true">
			<GameCardSkeleton />
		</div>
	</section>
{:else if game}
	<HeroBanner
		{game}
		imageType="horizontal"
		eyebrow={pageLabels.continuePlaying.eyebrow}
		showFavorite
	/>
{:else}
	<section class="continue-empty">
		<p class="eyebrow">{pageLabels.continuePlaying.eyebrow}</p>
		<div class="placeholder-row">
			<EmptyState
				variant="card"
				icon={appIcons.ui.emptySlot}
				title={pageLabels.continuePlaying.emptyTitle}
				subtitle={pageLabels.continuePlaying.emptySubtitle}
			/>
		</div>
	</section>
{/if}

<style>
	.continue-empty {
		display: grid;
		gap: var(--space-4);
	}

	.eyebrow {
		margin: 0;
		color: var(--text-secondary);
		font-size: var(--font-size-control-sm);
		font-weight: 600;
	}

	.placeholder-row {
		display: flex;
		min-height: calc(var(--card-height-sm) + var(--space-4));
		align-items: stretch;
	}
</style>
