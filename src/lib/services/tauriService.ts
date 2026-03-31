import { invoke } from '@tauri-apps/api/core';
import { openPath } from '@tauri-apps/plugin-opener';
import { uiStore } from '$lib/stores/uiStore';

export interface StoredGame {
  id: string;
  title: string;
  exePath: string;
  coverArt: string;
  platform: string;
  totalPlaytime: number;
  lastPlayed: string | null;
  status: string;
  genres: string[];
  description: string;
}

export async function getGames(): Promise<StoredGame[]> {
  return invoke<StoredGame[]>('read_games');
}

export async function saveGames(games: StoredGame[]): Promise<string> {
  return invoke<string>('write_games', { games });
}

export async function launchGame(exePath: string) {
  if (!exePath) {
    throw new Error('No executable path was provided for this game.');
  }

  try {
    uiStore.setGameRunning(true);
    await invoke('launch_game', { exePath });
  } catch (err) {
    uiStore.setGameRunning(false);
    console.error('Failed to launch game:', err);
    throw err instanceof Error ? err : new Error(String(err));
  }
}

export function setGameRunning(isRunning: boolean) {
  uiStore.setGameRunning(isRunning);
}

function getFolderPath(path: string) {
  const normalized = path.replace(/\//g, '\\');
  const index = normalized.lastIndexOf('\\');
  return index > 0 ? normalized.slice(0, index) : normalized;
}

export async function openGameFolder(path?: string) {
  if (!path) {
    throw new Error('No game path is available.');
  }

  await openPath(getFolderPath(path));
}

export async function openSaveFolder(path?: string) {
  if (!path) {
    throw new Error('No save path is configured for this game.');
  }

  await openPath(path);
}
