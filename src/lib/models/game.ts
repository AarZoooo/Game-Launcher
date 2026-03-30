export const GAME_STATUSES = ["installed", "playing", "completed", "backlog"] as const;

export const GAME_PLATFORMS = ["pc", "steam", "epic", "gog", "other"] as const;

export type GameStatus = (typeof GAME_STATUSES)[number];
export type GamePlatform = (typeof GAME_PLATFORMS)[number];

export interface Game {
  id: string;
  title: string;
  exePath: string;
  coverArt: string;
  platform: GamePlatform | string;
  totalPlaytime: number;
  lastPlayed: string | null;
  status: GameStatus | string;
  genres: string[];
  description: string;
}

export type GameDraft = Omit<Game, "id"> & {
  id?: string;
};
