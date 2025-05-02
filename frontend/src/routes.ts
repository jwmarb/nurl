import { type RouteConfig, route } from '@react-router/dev/routes';

const routes = [
  // * matches all URLs, the ? makes it optional so it will match / as well
  route('/auth', 'routes/auth/index.ts'),
  route('/auth/register', 'routes/auth/register/index.ts'),
  route('/', 'App.tsx'),
] satisfies RouteConfig;

export default routes;
