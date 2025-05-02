import { defineConfig, loadEnv } from 'vite';
import { reactRouter } from '@react-router/dev/vite';
import path from 'path';

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd(), '');

  return {
    plugins: [reactRouter()],
    define: {
      'process.env': env,
    },
    resolve: {
      alias: {
        $: path.resolve(__dirname, './src'),
      },
    },
  };
});
