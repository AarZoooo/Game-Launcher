import { derived, writable } from 'svelte/store';

export type UIModeVariant = 'auto' | 'normal' | 'gaming';
export type ResolvedUIMode = 'normal' | 'gaming';
export type ScanPlatform = 'steam' | 'epic' | 'local';

interface ScanState {
  steam: boolean;
  epic: boolean;
  local: boolean;
}

interface UIState {
  isGameRunning: boolean;
  activeGameId: string | null;
  activeMenuKey: string | null;
  performanceMode: boolean;
  scanning: ScanState;
  activeAccent: string;
  libraryBusy: boolean;
  libraryBusyMessage: string;
}

const initialState: UIState = {
  isGameRunning: false,
  activeGameId: null,
  activeMenuKey: null,
  performanceMode: false,
  scanning: {
    steam: false,
    epic: false,
    local: false
  },
  activeAccent: '#b69b57',
  libraryBusy: false,
  libraryBusyMessage: 'Updating library...'
};

function createUIStore() {
  const { subscribe, update, set } = writable<UIState>(initialState);

  return {
    subscribe,
    reset: () => set(initialState),
    setGameRunning: (isGameRunning: boolean, activeGameId: string | null = null) =>
      update((state) => ({
        ...state,
        isGameRunning,
        activeGameId: isGameRunning ? activeGameId ?? state.activeGameId : null
      })),
    setOpenMenu: (activeMenuKey: string | null) =>
      update((state) => ({
        ...state,
        activeMenuKey
      })),
    closeOpenMenu: () =>
      update((state) => ({
        ...state,
        activeMenuKey: null
      })),
    startGame: (activeGameId: string) =>
      update((state) => ({
        ...state,
        isGameRunning: true,
        activeGameId
      })),
    finishGame: (activeGameId?: string | null) =>
      update((state) => {
        if (activeGameId && state.activeGameId && state.activeGameId !== activeGameId) {
          return state;
        }

        return {
          ...state,
          isGameRunning: false,
          activeGameId: null
        };
      }),
    setPerformanceMode: (performanceMode: boolean) =>
      update((state) => ({ ...state, performanceMode })),
    togglePerformanceMode: () =>
      update((state) => ({ ...state, performanceMode: !state.performanceMode })),
    setScanning: (platform: ScanPlatform, value: boolean) =>
      update((state) => ({
        ...state,
        scanning: {
          ...state.scanning,
          [platform]: value
        }
      })),
    setLibraryBusy: (libraryBusy: boolean, libraryBusyMessage = 'Updating library...') =>
      update((state) => ({
        ...state,
        libraryBusy,
        libraryBusyMessage
      })),
    setActiveAccent: (activeAccent: string) =>
      update((state) => ({ ...state, activeAccent }))
  };
}

export const uiStore = createUIStore();

export const isGameRunning = derived(uiStore, ($uiStore) => $uiStore.isGameRunning);
export const activeGameId = derived(uiStore, ($uiStore) => $uiStore.activeGameId);
export const activeMenuKey = derived(uiStore, ($uiStore) => $uiStore.activeMenuKey);
export const performanceMode = derived(uiStore, ($uiStore) => $uiStore.performanceMode);
export const scanningState = derived(uiStore, ($uiStore) => $uiStore.scanning);
export const activeAccent = derived(uiStore, ($uiStore) => $uiStore.activeAccent);
export const libraryBusy = derived(uiStore, ($uiStore) => $uiStore.libraryBusy);
export const libraryBusyMessage = derived(uiStore, ($uiStore) => $uiStore.libraryBusyMessage);
export const effectiveUIMode = derived(
  uiStore,
  ($uiStore): ResolvedUIMode =>
    $uiStore.isGameRunning || $uiStore.performanceMode ? 'gaming' : 'normal'
);

export function resolveVariant(
  variant: UIModeVariant,
  isRunning: boolean,
  performance: boolean
): ResolvedUIMode {
  if (variant === 'normal' || variant === 'gaming') {
    return variant;
  }

  return isRunning || performance ? 'gaming' : 'normal';
}
