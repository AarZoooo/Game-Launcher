<script lang="ts">
import GameGrid from "$lib/components/game/GameGrid.svelte";
import HeroBanner from "$lib/components/game/HeroBanner.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";

export let game: Game;
export let similarGames: Game[] = [];
export let backHref = "/";

$: showPlayButton = game.inLibrary !== false;
</script>

<HeroBanner
  {game}
  imageType="banner"
  showBackButton
  {backHref}
  {showPlayButton}
/>

<section class="similar">
  <h2>{pageLabels.game.similarTitles}</h2>
  <GameGrid games={similarGames} horizontal compact menuPlacement="above-right" />
</section>

<style>
  .similar {
    display: flex;
    flex-direction: column;
    gap: var(--space-4);
  }

  h2 {
    margin: 0;
  }
</style>
