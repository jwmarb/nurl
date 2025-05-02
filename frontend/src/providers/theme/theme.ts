import { MessageInstance } from 'antd/es/message/interface';
import React from 'react';
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

export const MessageContext = React.createContext<MessageInstance | null>(null);

export const useMessage = () => {
  const notificationInstance = React.useContext(MessageContext);
  if (!notificationInstance) {
    throw new Error('useNotification must be used within a ThemeProvider');
  }
  return notificationInstance;
};
