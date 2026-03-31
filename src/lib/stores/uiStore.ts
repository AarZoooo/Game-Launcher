import { derived, writable } from 'svelte/store';

export type UIModeVariant = 'auto' | 'normal' | 'gaming';
export type ResolvedUIMode = 'normal' | 'gaming';
export type ScanPlatform = 'steam' | 'epic';

interface ScanState {
  steam: boolean;
  epic: boolean;
}

interface UIState {
  isGameRunning: boolean;
  performanceMode: boolean;
  scanning: ScanState;
  activeAccent: string;
}

const initialState: UIState = {
  isGameRunning: false,
  performanceMode: false,
  scanning: {
    steam: false,
    epic: false
  },
  activeAccent: '#b69b57'
};

function createUIStore() {
  const { subscribe, update, set } = writable<UIState>(initialState);

  return {
    subscribe,
    reset: () => set(initialState),
    setGameRunning: (isGameRunning: boolean) =>
      update((state) => ({ ...state, isGameRunning })),
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
    setActiveAccent: (activeAccent: string) =>
      update((state) => ({ ...state, activeAccent }))
  };
}

export const uiStore = createUIStore();

export const isGameRunning = derived(uiStore, ($uiStore) => $uiStore.isGameRunning);
export const performanceMode = derived(uiStore, ($uiStore) => $uiStore.performanceMode);
export const scanningState = derived(uiStore, ($uiStore) => $uiStore.scanning);
export const activeAccent = derived(uiStore, ($uiStore) => $uiStore.activeAccent);
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
