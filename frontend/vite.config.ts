import { defineConfig, loadEnv } from 'vite';
import { reactRouter } from '@react-router/dev/vite';

// https://vite.dev/config/
export default defineConfig(({ mode }) => {
  return {
    plugins: [reactRouter()],
    define: {
      'process.env': loadEnv(mode, process.cwd(), ''),
    },
  };
});
