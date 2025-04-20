export const ENVIRONMENT = process.env.PROD;
export const BACKEND_PORT = process.env?.PORT ?? '8080';
export const BACKEND_URL = ENVIRONMENT == 'production' ? '' : `http://localhost:${BACKEND_PORT}`;
