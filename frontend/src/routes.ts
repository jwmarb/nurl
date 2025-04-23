import { type RouteConfig, route } from '@react-router/dev/routes';

export default [
  // * matches all URLs, the ? makes it optional so it will match / as well
  route('/auth', 'routes/auth/index.ts'),
  route('*?', 'catchall.tsx'),
] satisfies RouteConfig;
