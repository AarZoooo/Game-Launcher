<script lang="ts">
export let compact = false;
</script>

<article class="game-card game-card-skeleton" class:compact>
	<div class="game-card-poster">
		<div class="game-card-image skeleton-panel"></div>
	</div>

	<div class="game-card-overlay">
		<div class="game-card-title-row">
			<div class="game-card-copy">
				<span class="skeleton-line title"></span>
				<span class="skeleton-line meta"></span>
			</div>
			<div class="game-card-actions">
				<span class="skeleton-dot"></span>
				<span class="skeleton-dot"></span>
			</div>
		</div>
	</div>
</article>

<style>
	.game-card-skeleton {
		--card-width: var(--card-width-md);
		width: var(--card-width);
		pointer-events: none;
	}

	.game-card-skeleton.compact {
		--card-width: var(--card-width-sm);
	}

	.game-card-skeleton:hover {
		transform: none;
	}

	.skeleton-panel,
	.skeleton-line,
	.skeleton-dot {
		position: relative;
		overflow: hidden;
		background: rgb(255 255 255 / 0.08);
	}

	.skeleton-panel::after,
	.skeleton-line::after,
	.skeleton-dot::after {
		content: "";
		position: absolute;
		inset: 0;
		transform: translateX(-100%);
		background: linear-gradient(
			90deg,
			transparent,
			rgb(255 255 255 / 0.18),
			transparent
		);
		animation: shimmer 1.4s ease-in-out infinite;
	}

	.game-card-image {
		aspect-ratio: var(--card-media-aspect-portrait);
		border-radius: var(--radius-card);
		background:
			linear-gradient(180deg, rgb(255 255 255 / 0.08), rgb(255 255 255 / 0.03)),
			var(--surface-glass);
		mask-image: linear-gradient(180deg, black 50%, transparent 100%);
		-webkit-mask-image: linear-gradient(180deg, black 50%, transparent 100%);
	}

	.title {
		width: 78%;
		height: 1rem;
		border-radius: var(--radius-control-sm);
	}

	.meta {
		width: 52%;
		height: 0.75rem;
		border-radius: var(--radius-control-sm);
	}

	.skeleton-dot {
		width: var(--control-height-xs);
		height: var(--control-height-xs);
		border-radius: var(--radius-round);
	}

	@keyframes shimmer {
		to {
			transform: translateX(100%);
		}
	}
</style>
