<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import { appIcons } from "$lib/assets";
import Button from "$lib/components/common/Button.svelte";
import Icon from "$lib/components/common/Icon.svelte";
import { pageLabels } from "$lib/data/labels";
import { playGame } from "$lib/services/gameService";
import { activeGameId, isGameRunning } from "$lib/stores/uiStore";
import type { Game } from "$lib/types/Game";

const dispatch = createEventDispatcher<{
	launcherror: string;
}>();

export let game: Game;
export let compact = true;
export let wide = false;
export let showLabel = true;
export let quiet = false;

$: playing = $isGameRunning && $activeGameId === game.id;
$: label = playing ? pageLabels.buttons.playing : pageLabels.buttons.play;

async function handleClick() {
	if (playing) return;

	if (!game.path) {
		goto("/games");
		return;
	}

	try {
		await playGame(game);
	} catch (error) {
		dispatch(
			"launcherror",
			error instanceof Error ? error.message : String(error),
		);
	}
}
</script>

<Button
  accentColor={playing ? undefined : game.accentColor || game.accentHex}
  accent={playing ? 'silver' : game.accent}
  {compact}
  {wide}
  quiet={quiet || playing}
  iconFirst
  disabled={playing}
  ariaLabel={`${label} ${game.title}`}
  on:click={handleClick}
>
  <Icon src={appIcons.ui.play} size="0.95rem" />
  {#if showLabel}
    <span>{label}</span>
  {/if}
</Button>
