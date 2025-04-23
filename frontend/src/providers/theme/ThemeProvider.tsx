import { useTheme } from '$/providers/theme/theme';
import { ConfigProvider, ThemeConfig } from 'antd';
import { theme } from 'antd';
import React from 'react';

export default function ThemeProvider({ children }: React.PropsWithChildren) {
  const customTheme = useTheme((state) => state.theme);
  const setTheme = useTheme((state) => state.setTheme);
  const themeConfig: ThemeConfig = {
    algorithm: customTheme === 'dark' ? theme.darkAlgorithm : theme.defaultAlgorithm,
  };
  const token = theme.getDesignToken(themeConfig);
  React.useLayoutEffect(() => {
    if (customTheme == null) {
      setTheme(window.matchMedia && window.matchMedia('(prefers-color-scheme: dark)').matches ? 'dark' : 'light');
    }

    document.body.style.backgroundColor = token.colorBgContainer;
  }, [token.colorBgContainer, customTheme, setTheme]);
  return (
    <ConfigProvider
      theme={{
        algorithm: theme.darkAlgorithm,
      }}>
      {children}
    </ConfigProvider>
  );
}
