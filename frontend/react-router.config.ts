import type { Config } from '@react-router/dev/config';

export default {
  appDirectory: 'src',
  ssr: false,
  async prerender() {
    return ['/', '/auth', '/auth/register'];
  },
} satisfies Config;
