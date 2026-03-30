import type { Game, GameDraft } from "$lib/models/game";
import { FALLBACK_COVER_IMAGE } from "$lib/utils/constants";

import * as tauriService from "./tauriService";

function createGameId(): string {
  return `game-${crypto.randomUUID()}`;
}

function normalizeGame(input: GameDraft): Game {
  return {
    id: input.id ?? createGameId(),
    title: input.title.trim(),
    exePath: input.exePath.trim(),
    coverArt: input.coverArt?.trim() || FALLBACK_COVER_IMAGE,
    platform: input.platform,
    totalPlaytime: input.totalPlaytime ?? 0,
    lastPlayed: input.lastPlayed ?? null,
    status: input.status,
    genres: input.genres ?? [],
    description: input.description?.trim() || "",
  };
}

export async function loadGames(): Promise<Game[]> {
  return tauriService.getGames();
}

export async function saveGameList(games: Game[]): Promise<Game[]> {
  await tauriService.saveGames(games);
  return games;
}

export async function addGame(currentGames: Game[], gameInput: GameDraft): Promise<Game[]> {
  const nextGames = [...currentGames, normalizeGame(gameInput)];
  return saveGameList(nextGames);
}

export async function updateGame(currentGames: Game[], updatedGame: Game): Promise<Game[]> {
  const nextGames = currentGames.map((game) => (game.id === updatedGame.id ? normalizeGame(updatedGame) : game));
  return saveGameList(nextGames);
}

export async function deleteGame(currentGames: Game[], gameId: string): Promise<Game[]> {
  const nextGames = currentGames.filter((game) => game.id !== gameId);
  return saveGameList(nextGames);
}

export async function launchSelectedGame(game: Game): Promise<string> {
  return tauriService.launchGame(game.exePath);
}
