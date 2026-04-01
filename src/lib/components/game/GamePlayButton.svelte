<script lang="ts">
import { createEventDispatcher } from "svelte";
import { goto } from "$app/navigation";
import Button from "$lib/components/common/Button.svelte";
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
  <svg viewBox="0 0 16 16" fill="currentColor" aria-hidden="true">
    <path d="M5 3.75a.75.75 0 0 1 1.14-.64l5.5 3.5a.75.75 0 0 1 0 1.28l-5.5 3.5A.75.75 0 0 1 5 10.75z" />
  </svg>
  {#if showLabel}
    <span>{label}</span>
  {/if}
</Button>
