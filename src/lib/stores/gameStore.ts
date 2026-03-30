import { get, writable } from "svelte/store";

import type { Game, GameDraft } from "$lib/models/game";
import * as gameService from "$lib/services/gameService";

interface GameStoreState {
  games: Game[];
  loading: boolean;
  selectedGame: Game | null;
}

const initialState: GameStoreState = {
  games: [],
  loading: false,
  selectedGame: null,
};

function createGameStore() {
  const store = writable<GameStoreState>(initialState);
  const { subscribe, set, update } = store;

  return {
    subscribe,

    async fetchGames() {
      update((state) => ({ ...state, loading: true }));

      try {
        const games = await gameService.loadGames();

        update((state) => ({
          ...state,
          games,
          loading: false,
          selectedGame: state.selectedGame
            ? games.find((game) => game.id === state.selectedGame?.id) ?? null
            : games[0] ?? null,
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false }));
        throw error;
      }
    },

    async addGame(gameInput: GameDraft) {
      update((state) => ({ ...state, loading: true }));

      try {
        const games = await gameService.addGame(get(store).games, gameInput);

        update((state) => ({
          ...state,
          games,
          loading: false,
          selectedGame: games[games.length - 1] ?? state.selectedGame,
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false }));
        throw error;
      }
    },

    async updateGame(updatedGame: Game) {
      update((state) => ({ ...state, loading: true }));

      try {
        const games = await gameService.updateGame(get(store).games, updatedGame);

        update((state) => ({
          ...state,
          games,
          loading: false,
          selectedGame: games.find((game) => game.id === updatedGame.id) ?? state.selectedGame,
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false }));
        throw error;
      }
    },

    async removeGame(gameId: string) {
      update((state) => ({ ...state, loading: true }));

      try {
        const games = await gameService.deleteGame(get(store).games, gameId);

        update((state) => ({
          ...state,
          games,
          loading: false,
          selectedGame: state.selectedGame?.id === gameId ? games[0] ?? null : state.selectedGame,
        }));
      } catch (error) {
        update((state) => ({ ...state, loading: false }));
        throw error;
      }
    },

    selectGame(gameId: string | null) {
      update((state) => ({
        ...state,
        selectedGame: gameId ? state.games.find((game) => game.id === gameId) ?? null : null,
      }));
    },

    reset() {
      set(initialState);
    },
  };
}

export const gameStore = createGameStore();
