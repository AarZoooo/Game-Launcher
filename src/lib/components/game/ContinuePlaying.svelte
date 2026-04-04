<script lang="ts">
import HeroBanner from "$lib/components/game/HeroBanner.svelte";
import { pageLabels } from "$lib/data/labels";
import type { Game } from "$lib/types/Game";

export let game: Game;
</script>

<HeroBanner
  {game}
  imageType="horizontal"
  eyebrow={pageLabels.continuePlaying.eyebrow}
  showFavorite
/>
