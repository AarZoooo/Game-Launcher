import { GAME_PLATFORMS, GAME_STATUSES, type GamePlatform, type GameStatus } from "$lib/models/game";

export const DEFAULT_STATUSES: GameStatus[] = [...GAME_STATUSES];

export const DEFAULT_PLATFORMS: GamePlatform[] = [...GAME_PLATFORMS];

export const DEFAULT_SCAN_DIRECTORIES = [
  "C:\\Program Files",
  "C:\\Program Files (x86)",
  "C:\\Games",
  "D:\\Games",
  "D:\\Program Files",
];

export const FALLBACK_COVER_IMAGE = "/fallback-cover.svg";
