import { create } from 'zustand';

type BackendState = {
  isAlive: boolean;
  setIsAlive: (isAlive: boolean) => void;
};

export const useBackendStore = create<BackendState>((set) => ({
  isAlive: true,
  setIsAlive: (isAlive: boolean) => set({ isAlive }),
}));
