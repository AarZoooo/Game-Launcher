import { invoke } from "@tauri-apps/api/core";

import type { Game } from "$lib/models/game";

export async function getGames(): Promise<Game[]> {
  return invoke<Game[]>("read_games");
}

export async function saveGames(games: Game[]): Promise<string> {
  return invoke<string>("write_games", { games });
}

export async function launchGame(exePath: string): Promise<string> {
  return invoke<string>("launch_game", { exePath });
}
