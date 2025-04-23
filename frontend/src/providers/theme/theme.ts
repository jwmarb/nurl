import { create } from 'zustand';
import { createJSONStorage, persist } from 'zustand/middleware';

export type Theme = {
  theme: 'light' | 'dark' | null;
  setTheme: (theme: Theme['theme']) => void;
};

export const useTheme = create<Theme>()(
  persist(
    (set) => ({
      theme: null,
      setTheme: (theme: Theme['theme']) => set({ theme }),
    }),
    {
      name: 'theme', // name of the storage
      storage: createJSONStorage(() => localStorage),
    }
  )
);
